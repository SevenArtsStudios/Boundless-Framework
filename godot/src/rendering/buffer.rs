use godot::builtin::{Array, PackedByteArray, Projection, Rid};
use godot::classes::{RenderingDevice, rendering_device::IndexBufferFormat};

use godot::builtin::Vector2i;

#[must_use]
pub fn groups_for_size(size: Vector2i, group_size: u32) -> (u32, u32) {
	let width = size.x.max(1).cast_unsigned();
	let height = size.y.max(1).cast_unsigned();
	let x_groups = (width - 1) / group_size + 1;
	let y_groups = (height - 1) / group_size + 1;
	(x_groups, y_groups)
}

pub fn create_storage_buffer(rd: &mut RenderingDevice, floats: &[f32]) -> Rid {
	// SAFETY: `f32` is a plain data type, and reading its representation as bytes is valid.
	// The byte slice cannot outlive `floats` and is only read here.
	let bytes = unsafe {
		std::slice::from_raw_parts(
			floats.as_ptr().cast::<u8>(),
			std::mem::size_of_val(floats),
		)
	};
	let packed = PackedByteArray::from(bytes);
	#[allow(clippy::cast_possible_truncation)]
	rd.storage_buffer_create_ex(packed.len() as u32)
		.data(&packed)
		.done()
}

pub fn create_vertex_buffers(
	rd: &mut RenderingDevice,
	vertex_floats: &[f32],
	vertex_format: i64,
	vertex_count: u32,
) -> (Rid, Rid) {
	// SAFETY: `f32` is a plain data type, and reading its representation as bytes is valid.
	// The byte slice cannot outlive `vertex_floats` and is only read here.
	let bytes = unsafe {
		std::slice::from_raw_parts(
			vertex_floats.as_ptr().cast::<u8>(),
			std::mem::size_of_val(vertex_floats),
		)
	};
	let packed = PackedByteArray::from(bytes);
	#[allow(clippy::cast_possible_truncation)]
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

pub fn create_index_buffers(rd: &mut RenderingDevice, indices: &[u32]) -> (Rid, Rid) {
	// SAFETY: `u32` is a plain data type, and reading its representation as bytes is valid.
	// The byte slice cannot outlive `indices` and is only read here.
	let bytes = unsafe {
		std::slice::from_raw_parts(
			indices.as_ptr().cast::<u8>(),
			std::mem::size_of_val(indices),
		)
	};
	let packed = PackedByteArray::from(bytes);

	let index_count = indices.len();

	#[allow(clippy::cast_possible_truncation)]
	let index_buffer = rd
		.index_buffer_create_ex(index_count as u32, IndexBufferFormat::UINT32)
		.data(&packed)
		.done();
	if !index_buffer.is_valid() {
		return (Rid::Invalid, Rid::Invalid);
	}

	#[allow(clippy::cast_possible_truncation)]
	let index_array = rd.index_array_create(index_buffer, 0, index_count as u32);

	(index_buffer, index_array)
}

pub fn projection_to_floats(projection: Projection) -> [f32; 16] {
	let mut floats = [0f32; 16];
	for (i, col) in projection.cols.iter().enumerate() {
		floats[i * 4] = col.x;
		floats[i * 4 + 1] = col.y;
		floats[i * 4 + 2] = col.z;
		floats[i * 4 + 3] = col.w;
	}
	floats
}
pub fn projection_to_bytes(projection: Projection) -> [u8; 16 * 4] {
	let mut bytes = [0u8; 16 * 4];
	for (i, col) in projection.cols.iter().enumerate() {
		bytes[i * 16..i * 16 + 4].copy_from_slice(&col.x.to_ne_bytes());
		bytes[i * 16 + 4..i * 16 + 8].copy_from_slice(&col.y.to_ne_bytes());
		bytes[i * 16 + 8..i * 16 + 12].copy_from_slice(&col.z.to_ne_bytes());
		bytes[i * 16 + 12..i * 16 + 16].copy_from_slice(&col.w.to_ne_bytes());
	}
	bytes
}