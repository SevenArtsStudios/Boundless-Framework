use godot::builtin::{Array, Color, PackedByteArray, Rid};
use godot::classes::rendering_device::UniformType;
use godot::classes::{RdUniform, RenderingDevice};
use godot::obj::{Gd, NewGd};

use crate::water::WaterDisplacement;


pub fn run_fetch_displacement_pass(
	rd: &mut RenderingDevice,
	fetch_pipeline: Rid,
	fetch_shader: Rid,
	displacement_sampler: Rid,
	water_displacement_map: Rid,
) {
	let batch = WaterDisplacement::collect_fetch_queries();
	if batch.is_empty() {
		return;
	}

	let mut input_floats = Vec::<f32>::with_capacity(batch.len() * 4);
	for query in batch.queries() {
		input_floats.push(query.x);
		input_floats.push(query.z);
		input_floats.push(query.intensity);
		input_floats.push(query.scale.max(0.0001));
	}
	// SAFETY: `f32` is a plain data type, and reading its representation as bytes is valid.
	// The byte slice cannot outlive `input_floats` and is only read here.
	let input_bytes = unsafe {
		std::slice::from_raw_parts(
			input_floats.as_slice().as_ptr().cast::<u8>(),
			input_floats.len() * std::mem::size_of::<f32>()
		)
	};

	let input_data = PackedByteArray::from(input_bytes);

	#[allow(clippy::cast_possible_truncation)]
	let output_byte_count = (batch.len() * 3 * std::mem::size_of::<f32>()) as u32;

	#[allow(clippy::cast_possible_truncation)]
	let input_buffer = rd
		.storage_buffer_create_ex(input_data.len() as u32)
		.data(&input_data)
		.done();
	let output_buffer = rd.storage_buffer_create(output_byte_count);

	let mut set0_uniforms = Array::<Gd<RdUniform>>::new();
	let mut displacement_uniform = RdUniform::new_gd();
	displacement_uniform.set_uniform_type(UniformType::SAMPLER_WITH_TEXTURE);
	displacement_uniform.set_binding(0);
	displacement_uniform.add_id(displacement_sampler);
	displacement_uniform.add_id(water_displacement_map);
	set0_uniforms.push(&displacement_uniform);
	let uniform_set0 = rd.uniform_set_create(&set0_uniforms, fetch_shader, 0);

	let mut set1_uniforms = Array::<Gd<RdUniform>>::new();
	let mut input_uniform = RdUniform::new_gd();
	input_uniform.set_uniform_type(UniformType::STORAGE_BUFFER);
	input_uniform.set_binding(0);
	input_uniform.add_id(input_buffer);
	set1_uniforms.push(&input_uniform);
	let uniform_set1 = rd.uniform_set_create(&set1_uniforms, fetch_shader, 1);

	let mut set2_uniforms = Array::<Gd<RdUniform>>::new();
	let mut output_uniform = RdUniform::new_gd();
	output_uniform.set_uniform_type(UniformType::STORAGE_BUFFER);
	output_uniform.set_binding(0);
	output_uniform.add_id(output_buffer);
	set2_uniforms.push(&output_uniform);
	let uniform_set2 = rd.uniform_set_create(&set2_uniforms, fetch_shader, 2);

	if !uniform_set0.is_valid() || !uniform_set1.is_valid() || !uniform_set2.is_valid() {
		if uniform_set0.is_valid() {
			rd.free_rid(uniform_set0);
		}
		if uniform_set1.is_valid() {
			rd.free_rid(uniform_set1);
		}
		if uniform_set2.is_valid() {
			rd.free_rid(uniform_set2);
		}
		rd.free_rid(input_buffer);
		rd.free_rid(output_buffer);
		return;
	}

	rd.draw_command_begin_label("Fetch Water Displacement", Color::from_rgb(1.0, 1.0, 1.0));
	let fetch_list = rd.compute_list_begin();
	rd.compute_list_bind_compute_pipeline(fetch_list, fetch_pipeline);
	rd.compute_list_bind_uniform_set(fetch_list, uniform_set0, 0);
	rd.compute_list_bind_uniform_set(fetch_list, uniform_set1, 1);
	rd.compute_list_bind_uniform_set(fetch_list, uniform_set2, 2);
	#[allow(clippy::cast_possible_truncation)]
	rd.compute_list_dispatch(fetch_list, batch.len() as u32, 1, 1);
	rd.compute_list_end();
	rd.draw_command_end_label();

	let output_data = rd.buffer_get_data(output_buffer);
	// SAFETY: `u8` is a plain data type, and reading its representation as floats is valid.
	// The float slice cannot outlive `output_data` and is only read here.
	#[allow(clippy::cast_ptr_alignment)]
	let fetched_xyz = unsafe {
		std::slice::from_raw_parts(
			output_data.as_slice().as_ptr().cast::<f32>(),
			output_data.len() / 4
		)
	};

	WaterDisplacement::dispatch_fetched_updates(&batch, fetched_xyz);

	rd.free_rid(uniform_set0);
	rd.free_rid(uniform_set1);
	rd.free_rid(uniform_set2);
	rd.free_rid(input_buffer);
	rd.free_rid(output_buffer);
}
