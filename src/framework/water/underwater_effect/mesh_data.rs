use godot::builtin::{
	Array, PackedByteArray, PackedInt32Array, PackedVector3Array, Projection, Rid,
};
use godot::classes::mesh::ArrayType;
use godot::classes::rendering_device::IndexBufferFormat;
use godot::classes::{RenderingDevice, RenderingServer};
use godot::obj::{EngineEnum, Gd, Singleton, WithBaseField};

use crate::framework::{WaterMesh, WaterMeshRegistry};

/// 4x4 matrix + intensity + scale = 18 | 20 with padding
const WATER_INFO_STRIDE: usize = 20;

pub(crate) struct WaterMeshGpuData {
	pub vertex_floats: Vec<f32>,
	pub indices: Vec<u32>,
	pub water_info_floats: Vec<f32>,
	pub water_params_floats: Vec<f32>,
}

pub(crate) fn collect_water_mesh_data() -> Option<WaterMeshGpuData> {
	let mut vertex_floats = Vec::<f32>::new();
	let mut indices = Vec::<u32>::new();
	let mut water_info_floats = Vec::<f32>::new();
	let mut water_params_floats = Vec::<f32>::new();

	let server = RenderingServer::singleton();
	let mesh_ids = WaterMeshRegistry::mesh_ids();

	for mesh_id in mesh_ids {
		let Ok(mesh_obj) = Gd::<WaterMesh>::try_from_instance_id(mesh_id) else {
			continue;
		};

		let mesh = mesh_obj.bind();
		if !mesh.base().is_visible_in_tree() {
			continue;
		}

		let Some(surface_mesh) = mesh.get_water_mesh() else {
			continue;
		};

		let mesh_index = (water_info_floats.len() / WATER_INFO_STRIDE) as u32;
		let surface_count = surface_mesh.get_surface_count();
		for surface_index in 0..surface_count {
			let surface_arrays = server.mesh_surface_get_arrays(surface_mesh.get_rid(), surface_index);

			let Some(vertex_variant) = surface_arrays.get(ArrayType::VERTEX.ord() as usize) else {
				continue;
			};
			let Ok(surface_vertices): Result<PackedVector3Array, _> = vertex_variant.try_to() else {
				continue;
			};

			let surface_indices = surface_arrays
				.get(ArrayType::INDEX.ord() as usize)
				.and_then(|value| value.try_to::<PackedInt32Array>().ok())
				.unwrap_or_default();

			let vertex_offset = (vertex_floats.len() / 4) as u32;
			for vertex in surface_vertices.as_slice().iter().copied() {
				vertex_floats.push(vertex.x);
				vertex_floats.push(vertex.y);
				vertex_floats.push(vertex.z);
				vertex_floats.push(f32::from_bits(mesh_index));
			}

			for index in surface_indices.as_slice().iter().copied() {
				indices.push(index as u32 + vertex_offset);
			}
		}

		let transform = Projection::from(mesh.base().get_global_transform());
		append_projection_floats(&mut water_info_floats, transform);
		water_info_floats.push(mesh.water_intensity_value());
		water_info_floats.push(mesh.water_scale_value());
		water_info_floats.push(0.0);
		water_info_floats.push(0.0);

		let shallow = mesh.shallow_color_value().srgb_to_linear();
		let deep = mesh.deep_color_value().srgb_to_linear();
		water_params_floats.push(shallow.r);
		water_params_floats.push(shallow.g);
		water_params_floats.push(shallow.b);
		water_params_floats.push(mesh.fog_distance_value());
		water_params_floats.push(deep.r);
		water_params_floats.push(deep.g);
		water_params_floats.push(deep.b);
		water_params_floats.push(mesh.fog_fade_value());
		water_params_floats.push(mesh.transparency_distance_value());
		water_params_floats.push(mesh.transparency_fade_value());
		water_params_floats.push(0.0);
		water_params_floats.push(0.0);
	}

	if vertex_floats.is_empty() || indices.is_empty() || water_info_floats.is_empty() {
		return None;
	}

	Some(WaterMeshGpuData {
		vertex_floats,
		indices,
		water_info_floats,
		water_params_floats,
	})
}

pub(crate) fn create_storage_buffer(rd: &mut Gd<RenderingDevice>, floats: &[f32]) -> Rid {
	let mut bytes = Vec::<u8>::with_capacity(floats.len() * 4);
	for value in floats {
		bytes.extend_from_slice(&value.to_ne_bytes());
	}
	let packed = PackedByteArray::from(bytes.as_slice());
	rd.storage_buffer_create_ex(packed.len() as u32)
		.data(&packed)
		.done()
}

pub(crate) fn create_vertex_buffers(
	rd: &mut Gd<RenderingDevice>,
	vertex_floats: &[f32],
	vertex_format: i64,
	vertex_count: u32,
) -> (Rid, Rid) {
	let mut bytes = Vec::<u8>::with_capacity(vertex_floats.len() * 4);
	for value in vertex_floats {
		bytes.extend_from_slice(&value.to_ne_bytes());
	}
	let packed = PackedByteArray::from(bytes.as_slice());
	let vertex_buffer = rd
		.vertex_buffer_create_ex(packed.len() as u32)
		.data(&packed)
		.done();

	if !vertex_buffer.is_valid() {
		return (Rid::Invalid, Rid::Invalid);
	}

	let mut src_buffers = Array::<Rid>::new();
	src_buffers.push(vertex_buffer);
	let vertex_array = rd.vertex_array_create(vertex_count, vertex_format, &src_buffers);
	(vertex_buffer, vertex_array)
}

pub(crate) fn create_index_buffers(rd: &mut Gd<RenderingDevice>, indices: &[u32]) -> (Rid, Rid) {
	let mut bytes = Vec::<u8>::with_capacity(indices.len() * 4);
	for index in indices {
		bytes.extend_from_slice(&index.to_ne_bytes());
	}
	let packed = PackedByteArray::from(bytes.as_slice());
	let index_buffer = rd
		.index_buffer_create_ex(indices.len() as u32, IndexBufferFormat::UINT32)
		.data(&packed)
		.done();
	if !index_buffer.is_valid() {
		return (Rid::Invalid, Rid::Invalid);
	}

	let index_array = rd.index_array_create(index_buffer, 0, indices.len() as u32);
	(index_buffer, index_array)
}

fn append_projection_floats(output: &mut Vec<f32>, projection: Projection) {
	for col in projection.cols {
		output.push(col.x);
		output.push(col.y);
		output.push(col.z);
		output.push(col.w);
	}
}
