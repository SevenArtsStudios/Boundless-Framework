use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::Hasher;

use godot::builtin::{PackedInt32Array, PackedVector3Array, Callable, Projection};
use godot::classes::mesh::ArrayType;
use godot::classes::RenderingServer;
use godot::meta::ToGodot;
use godot::obj::{Gd, IndexEnum, InstanceId, Singleton, WithBaseField};
use godot::register::{godot_api, GodotClass};

use crate::projection_to_floats;
use crate::water::WaterMesh;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WaterMeshGpuData {
	pub vertex_floats: Box<[f32]>,
	pub indices: Box<[u32]>,
	pub water_info_floats: Box<[f32]>,
	pub water_params_floats: Box<[f32]>,
}

const VERTEX_STRIDE: usize = 4; // x, y, z, mesh_index
const INDEX_STRIDE: usize = 1; // u32
const WATER_INFO_STRIDE: usize = 20; // 4x4 projection + intensity + scale + 4 padding
const WATER_PARAMS_STRIDE: usize = 12; // shallow_color + fog_distance + deep_color + fog_fade + transparency_distance + transparency_fade + 2 padding

#[derive(Debug, Clone, PartialEq, Default)]
enum WaterMeshDataCache {
	#[default]
	Uninitialized,
	None(u64),
	Some(WaterMeshGpuData, u64),
}

#[derive(GodotClass)]
#[class(singleton, init)]
pub struct WaterMeshRegistry {
	water_data: BTreeMap<InstanceId, (PackedVector3Array, PackedInt32Array)>,
	scheduled_rebuilds: BTreeSet<InstanceId>,
	cached_data: WaterMeshDataCache,
}

#[godot_api]
impl WaterMeshRegistry {
	pub fn add_id(&mut self, id: InstanceId) {
		// godot_print!("Adding water mesh with ID {} to registry.", id);

		self.schedule_rebuild_buffers(id);
		// godot_print!("Added water mesh with ID {} to registry and scheduled buffers rebuild.", id);
	}

	pub fn remove_id(&mut self, id: &InstanceId) {
		// godot_print!("Removing water mesh with ID {} from registry.", id);

		if self.water_data.remove(id).is_some() {
			// godot_print!("Removed water mesh with ID {} from registry and cleared its buffers.", id);
		}
	}

	pub fn schedule_rebuild_buffers(&mut self, id: InstanceId) {
		if ! self.scheduled_rebuilds.insert(id) {
			// godot_print!("Rebuild already scheduled, skipping redundant schedule call.");
			return;
		}

		if self.scheduled_rebuilds.len() == 1 {
			// godot_print!("Scheduling water buffers rebuild from schedule_rebuild_buffers.");
			Self::scheduled_rebuild_callable().call_deferred(&[]);
		}
	}

	fn scheduled_buffers_rebuild(&mut self) {
		// godot_print!("Running scheduled water buffers rebuild for {} meshes.", self.scheduled_rebuilds.len());

		let ids = std::mem::take(&mut self.scheduled_rebuilds);

		for id in ids {
			self.rebuild_buffers(id);
		}
	}

	fn rebuild_buffers(&mut self, instance_id: InstanceId) -> bool {
		let server = RenderingServer::singleton();

		let Some(mesh_object) = Gd::<WaterMesh>::try_from_instance_id(instance_id).ok() else {
			return false;
		};

		let mesh = mesh_object.bind();
		let Some(surface_mesh) = mesh.get_water_mesh() else {
			return false;
		};

		let (water_vertices, water_indices) = self.water_data
			.entry(instance_id)
			.and_modify(|e| {
				e.0.clear();
				e.1.clear();
			})
			.or_insert_with(|| (PackedVector3Array::new(), PackedInt32Array::new()));

		let surface_count = surface_mesh.get_surface_count();
		for surface_index in 0..surface_count {
			let surface_arrays = server.mesh_surface_get_arrays(surface_mesh.get_rid(), surface_index);

			let Some(vertex_variant) = surface_arrays.get(ArrayType::VERTEX.to_index()) else {
				continue;
			};

			let Ok(surface_vertices) = vertex_variant.try_to::<PackedVector3Array>() else {
				continue;
			};

			let surface_indices = surface_arrays
				.get(ArrayType::INDEX.to_index())
				.and_then(|value| value.try_to::<PackedInt32Array>().ok())
				.unwrap_or_default();

			// let vertex_offset = (vertex_floats.len() / 4) as u32;
			for vertex in surface_vertices.as_slice().iter().copied() {
				water_vertices.push(vertex);
			}

			for index in surface_indices.as_slice().iter().copied() {
				water_indices.push(index/*  + vertex_offset */);
			}
		}

		true
	}

	pub fn clear_buffers(&mut self, instance_id: &InstanceId) {
		self.water_data.remove(instance_id);
	}


	#[func]
	pub fn schedule_rebuild(id: InstanceId) {
		let mut registry = Self::singleton();
		registry.bind_mut().schedule_rebuild_buffers(id);
	}
	#[must_use]
	pub fn schedule_rebuild_callable(instance_id: InstanceId) -> Callable {
		Callable::from_class_static("WaterMeshRegistry", "schedule_rebuild").bind(&[instance_id.to_variant()])
	}

	#[func]
	fn scheduled_rebuild() {
		let mut registry = Self::singleton();
		registry.bind_mut().scheduled_buffers_rebuild();
	}
	#[must_use]
	pub fn scheduled_rebuild_callable() -> Callable {
		Callable::from_class_static("WaterMeshRegistry", "scheduled_rebuild")
	}

	#[func]
	pub fn clear(id: InstanceId) {
		let mut registry = Self::singleton();
		registry.bind_mut().clear_buffers(&id);
	}
	#[must_use]
	pub fn clear_callable(instance_id: InstanceId) -> Callable {
		Callable::from_class_static("WaterMeshRegistry", "clear").bind(&[instance_id.to_variant()])
	}

	#[func]
	pub fn add(id: InstanceId) {
		let mut registry = Self::singleton();
		registry.bind_mut().add_id(id);
	}
	#[must_use]
	pub fn add_callable(instance_id: InstanceId) -> Callable {
		Callable::from_class_static("WaterMeshRegistry", "add").bind(&[instance_id.to_variant()])
	}

	#[func]
	pub fn remove(id: InstanceId) {
		let mut registry = Self::singleton();
		registry.bind_mut().remove_id(&id);
	}
	#[must_use]
	pub fn remove_callable(instance_id: InstanceId) -> Callable {
		Callable::from_class_static("WaterMeshRegistry", "remove").bind(&[instance_id.to_variant()])
	}


	pub fn get_mesh_ids(&self) -> impl Iterator<Item = &InstanceId> {
		self.water_data.keys()
	}
	#[must_use]
	pub fn get_mesh_data(&self, instance_id: &InstanceId) -> Option<&(PackedVector3Array, PackedInt32Array)> {
		self.water_data.get(instance_id)
	}

	#[must_use]
	pub fn len(&self) -> usize {
		self.water_data.len()
	}
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.water_data.is_empty()
	}


	fn build_gpu_data(&self) -> Option<WaterMeshGpuData> {
		let mesh_count = self.len();

		let mut vertex_floats = Vec::<f32>::new();
		let mut indices = Vec::<u32>::new();
		let mut water_info_floats = Vec::<f32>::with_capacity(mesh_count * WATER_INFO_STRIDE);
		let mut water_params_floats = Vec::<f32>::with_capacity(mesh_count * WATER_PARAMS_STRIDE);


		let mut mesh_index: u32 = 0;
		for mesh_id in self.get_mesh_ids() {
			let Ok(mesh_obj) = Gd::<WaterMesh>::try_from_instance_id(*mesh_id) else {
				continue;
			};

			let mesh = mesh_obj.bind();
			if ! mesh.base().is_visible_in_tree() {
				continue;
			}

			let Some((water_vertices, water_indices)) = self.get_mesh_data(mesh_id) else {
				continue;
			};

			let vertex_count = water_vertices.len();
			let index_count = water_indices.len();

			if vertex_floats.try_reserve_exact(vertex_count * VERTEX_STRIDE).is_err() {
				break;
			}
			if indices.try_reserve_exact(index_count * INDEX_STRIDE).is_err() {
				break;
			}

			#[allow(clippy::cast_possible_truncation)]
			let idx_offset = vertex_floats.len() as u32 / 4;
			for vertex in water_vertices.as_slice() {
				vertex_floats.push(vertex.x);
				vertex_floats.push(vertex.y);
				vertex_floats.push(vertex.z);
				vertex_floats.push(f32::from_bits(mesh_index));
			}
			for index in water_indices.as_slice() {
				indices.push(index.cast_unsigned() + idx_offset);
			}

			let transform = Projection::from(mesh.base().get_global_transform());
			water_info_floats.extend_from_slice(&projection_to_floats(transform));
			water_info_floats.push(mesh.water_intensity);
			water_info_floats.push(mesh.water_scale);
			water_info_floats.push(0.0);
			water_info_floats.push(0.0);

			let shallow = mesh.shallow_color.srgb_to_linear();
			let deep = mesh.deep_color.srgb_to_linear();
			water_params_floats.push(shallow.r);
			water_params_floats.push(shallow.g);
			water_params_floats.push(shallow.b);
			water_params_floats.push(mesh.fog_distance);
			water_params_floats.push(deep.r);
			water_params_floats.push(deep.g);
			water_params_floats.push(deep.b);
			water_params_floats.push(mesh.fog_fade);
			water_params_floats.push(mesh.transparency_distance);
			water_params_floats.push(mesh.transparency_fade);
			water_params_floats.push(0.0);
			water_params_floats.push(0.0);

			mesh_index += 1;
		}

		if vertex_floats.is_empty() || indices.is_empty() || water_info_floats.is_empty() {
			return None;
		}

		Some(WaterMeshGpuData {
			vertex_floats: vertex_floats.into_boxed_slice(),
			indices: indices.into_boxed_slice(),
			water_info_floats: water_info_floats.into_boxed_slice(),
			water_params_floats: water_params_floats.into_boxed_slice(),
		})
	}

	pub const fn get_cached_data(&self) -> Option<&WaterMeshGpuData> {
		match &self.cached_data {
			WaterMeshDataCache::Some(data, _) => Some(data),
			_ => None,
		}
	}


	fn generate_mesh_data_hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		hasher.write_usize(self.water_data.len());
		for (instance_id, (vertices, indices)) in &self.water_data {
			hasher.write_i64(instance_id.to_i64());
			let Ok(mesh_obj) = Gd::<WaterMesh>::try_from_instance_id(*instance_id) else {
				continue;
			};
			if ! mesh_obj.bind().base().is_visible_in_tree() {
				continue;
			}
			let mesh = mesh_obj.bind();

			let transform = Projection::from(mesh.base().get_global_transform());
			for col in transform.cols {
				hasher.write(&col.x.to_ne_bytes());
				hasher.write(&col.y.to_ne_bytes());
				hasher.write(&col.z.to_ne_bytes());
				hasher.write(&col.w.to_ne_bytes());
			}
			hasher.write(&mesh.water_intensity.to_ne_bytes());
			hasher.write(&mesh.water_scale.to_ne_bytes());

			let shallow = mesh.shallow_color.srgb_to_linear();
			let deep = mesh.deep_color.srgb_to_linear();
			hasher.write(&shallow.r.to_ne_bytes());
			hasher.write(&shallow.g.to_ne_bytes());
			hasher.write(&shallow.b.to_ne_bytes());
			hasher.write(&mesh.fog_distance.to_ne_bytes());
			hasher.write(&deep.r.to_ne_bytes());
			hasher.write(&deep.g.to_ne_bytes());
			hasher.write(&deep.b.to_ne_bytes());
			hasher.write(&mesh.fog_fade.to_ne_bytes());
			hasher.write(&mesh.transparency_distance.to_ne_bytes());
			hasher.write(&mesh.transparency_fade.to_ne_bytes());

			hasher.write_usize(vertices.len());
			hasher.write_usize(indices.len());
		}

		hasher.finish()
	}

	pub fn update_cached_data(&mut self) {
		let new_hash = self.generate_mesh_data_hash();
		let cached_hash: Option<u64> = match &self.cached_data {
			WaterMeshDataCache::Uninitialized => None,
			WaterMeshDataCache::None(hash) | WaterMeshDataCache::Some(_, hash) => Some(*hash),
		};

		if cached_hash != Some(new_hash) {
			self.cached_data = match self.build_gpu_data() {
				Some(data) => WaterMeshDataCache::Some(data, new_hash),
				None => WaterMeshDataCache::None(new_hash),
			};
		}
	}

	pub fn get_updated_cached_data(&mut self) -> Option<&WaterMeshGpuData> {
		self.update_cached_data();
		self.get_cached_data()
	}
}