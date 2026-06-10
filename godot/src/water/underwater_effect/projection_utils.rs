use godot::builtin::{PackedByteArray, Projection};

pub(crate) fn append_projection_bytes(output: &mut Vec<u8>, projection: Projection) {
	for col in projection.cols {
		output.extend_from_slice(&col.x.to_ne_bytes());
		output.extend_from_slice(&col.y.to_ne_bytes());
		output.extend_from_slice(&col.z.to_ne_bytes());
		output.extend_from_slice(&col.w.to_ne_bytes());
	}
}

pub(crate) fn pack_projection(projection: Projection) -> PackedByteArray {
	let mut bytes = Vec::<u8>::with_capacity(16 * 4);
	append_projection_bytes(&mut bytes, projection);
	PackedByteArray::from(bytes.as_slice())
}
