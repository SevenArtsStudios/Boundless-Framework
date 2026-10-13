use godot::builtin::{PackedByteArray, Projection};

use crate::rendering::buffer::projection_to_bytes;

pub fn packed_bytes_from_projection(projection: Projection) -> PackedByteArray {
	let bytes = projection_to_bytes(projection);
	PackedByteArray::from(bytes)
}


#[must_use]
pub fn packed_bytes_from_i32(values: &[i32]) -> PackedByteArray {
	// SAFETY: `i32` is a plain data type, and reading its representation as bytes is valid.
	// The byte slice cannot outlive `values` and is only read here.
	let bytes = unsafe {
		std::slice::from_raw_parts(
			values.as_ptr().cast::<u8>(),
			std::mem::size_of_val(values),
		)
	};
	PackedByteArray::from(bytes)
}
