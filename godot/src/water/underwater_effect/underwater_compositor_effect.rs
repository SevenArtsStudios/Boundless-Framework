use godot::builtin::{Array, Vector2i, Projection, Rid};
use godot::classes::compositor_effect::EffectCallbackType;
use godot::classes::notify::ObjectNotification;
use godot::classes::rendering_device::{
	CompareOperator, DataFormat, RenderPrimitive, SamplerFilter, SamplerRepeatMode, ShaderLanguage, ShaderStage, TextureSamples
};
use godot::classes::{
	CompositorEffect, ICompositorEffect, RdAttachmentFormat, RdPipelineColorBlendState, RdPipelineColorBlendStateAttachment, RdPipelineDepthStencilState, RdPipelineMultisampleState, RdPipelineRasterizationState, RdSamplerState, RdShaderFile, RdVertexAttribute, RenderData, RenderSceneBuffersRd, RenderingDevice, RenderingServer, Texture2Drd
};
use godot::global::{godot_error};
use godot::obj::{Base, Gd, NewGd, Singleton, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

use crate::WaterMeshRegistry;
use crate::water::{apply_effect_pass, render_mask_pass};
use crate::rendering::{groups_for_size, shader::{create_shader_from_source, create_shader_from_sources}, buffer::{create_storage_buffer, create_vertex_buffers, create_index_buffers}};


pub(crate) const BUILTIN_WATER_MASK_VERTEX_GLSL: &str =
	include_str!("../../assets/shaders/RenderWaterMaskVertex.glsl");

pub(crate) const BUILTIN_WATER_MASK_FRAGMENT_GLSL: &str =
	include_str!("../../assets/shaders/RenderWaterMaskFragment.glsl");

pub(crate) const UNDERWATER_EFFECT_GLSL: &str =
	include_str!("../../assets/shaders/RenderUnderwaterEffect.glsl");

pub(crate) const UNDERWATER_CONTEXT: &str = "UnderwaterEffect";
pub(crate) const WATER_MASK_NAME: &str = "water_mask";
pub(crate) const WATER_DEPTH_NAME: &str = "water_depth";

pub(crate) const WATER_MASK_FORMAT: DataFormat = DataFormat::R32G32B32A32_SFLOAT;
pub(crate) const WATER_MASK_USAGE: u32 = 2u32 | 1u32 | 8u32; // TextureUsageBits::COLOR_ATTACHMENT_BIT | TextureUsageBits::SAMPLING_BIT | TextureUsageBits::STORAGE_BIT;

pub(crate) const WATER_DEPTH_FORMAT: DataFormat = DataFormat::D32_SFLOAT;
pub(crate) const WATER_DEPTH_USAGE: u32 = 4u32 | 1u32 | 8u32; // TextureUsageBits::DEPTH_STENCIL_ATTACHMENT_BIT | TextureUsageBits::SAMPLING_BIT | TextureUsageBits::STORAGE_BIT;

pub(crate) const WATER_TEXTURES_SAMPLES: TextureSamples = TextureSamples::SAMPLES_1;


#[derive(GodotClass)]
#[class(base = CompositorEffect, tool)]
pub struct UnderwaterEffect {
	#[base]
	base: Base<CompositorEffect>,

	#[var(set = set_render_shader_file)]
	#[export]
	render_shader_file: Option<Gd<RdShaderFile>>,

	/// Switching the texture RD RID from a scene-buffer texture is currently broken: <https://github.com/godotengine/godot/issues/118292>
	/// Otherwise, we would use a `Texture2Drd` as an output for the mask and depth texture buffers for easier user consumption

	// #[var(get = get_water_mask_texture, set = set_water_mask_texture)]
	// #[export]
	// water_mask_texture: Option<Gd<Texture2Drd>>,
	// water_mask_map: Rid,

	// #[var(get = get_water_depth_texture, set = set_water_depth_texture)]
	// #[export]
	// water_depth_texture: Option<Gd<Texture2Drd>>,
	// water_depth_map: Rid,

	#[export]
	main_view_index: u32,

	#[export]
	water_displacement_texture: Option<Gd<Texture2Drd>>,

	render_shader: Rid,
	compute_shader: Rid,
	render_pipeline: Rid,
	compute_pipeline: Rid,
	displacement_sampler: Rid,
	depth_sampler: Rid,
	framebuffer_format: i64,
	vertex_format: i64,
}

impl UnderwaterEffect {
	#[allow(clippy::too_many_lines)]
	fn run(&self, _effect_callback_type: i32, render_data: Option<Gd<RenderData>>) -> Result<(), &'static str> {
		// if effect_callback_type != EffectCallbackType::POST_TRANSPARENT.ord() {
		// 	return Err("Effect callback type is not POST_TRANSPARENT");
		// }

		let rs = RenderingServer::singleton();

		let Some(mut rendering_device) = rs.get_rendering_device() else {
			return Err("Rendering_device is not available");
		};
		let rd = &mut *rendering_device;

		if !self.render_pipeline.is_valid() ||
		   !self.compute_pipeline.is_valid() ||
		   !self.render_shader.is_valid() ||
		   !self.compute_shader.is_valid()
		{
			return Err("Effect is not properly initialized");
		}

		let Some(displacement_texture) = self
			.water_displacement_texture
			.as_ref()
			.map(|texture| texture.get_texture_rd_rid())
		else {
			return Err("Water_displacement_texture is not set");
		};

		if !displacement_texture.is_valid() {
			return Err("Water_displacement_texture is not valid");
		}

		let Some(render_data) = render_data else {
			return Err("Render_data is not set");
		};
		let Some(scene_buffers_base) = render_data.get_render_scene_buffers() else {
			return Err("Render_scene_buffers is not valid");
		};
		let Ok(mut scene_buffers) = scene_buffers_base.try_cast::<RenderSceneBuffersRd>() else {
			return Err("Render_scene_buffers is not valid");
		};

		let Some(scene_data) = render_data.get_render_scene_data() else {
			return Err("Render_scene_data is not valid");
		};

		let render_size = scene_buffers.get_internal_size();
		if render_size.x <= 0 || render_size.y <= 0 {
			return Err("Render_size is invalid");
		}

		let mut registry = WaterMeshRegistry::singleton();
		let mut registry_bind = registry.bind_mut();

		let Some(mesh_data) = registry_bind.get_updated_cached_data() else {
			return Ok(());
		};
		if mesh_data.vertex_floats.is_empty() || mesh_data.indices.is_empty() {
			return Err("Mesh_data is invalid");
		}

		let view_count = scene_buffers.get_view_count();

		#[allow(clippy::cast_possible_truncation)]
		/* let water_mask_map =  */scene_buffers.create_texture(
			UNDERWATER_CONTEXT,
			WATER_MASK_NAME,
			WATER_MASK_FORMAT,
			WATER_MASK_USAGE,
			WATER_TEXTURES_SAMPLES,
			render_size,
			view_count,
			1,
			true,
			false,
		);
		// if ! rd.texture_is_valid(self.water_mask_map) {
		// 	// godot_print!("Water Mask invalidated");

		// 	self.water_mask_map = water_mask_map;
		// 	// if let Some(texture) = self.water_mask_texture.as_mut() {
		// 	// 	texture.set_texture_rd_rid(self.water_mask_map);
		// 	// }
		// }

		#[allow(clippy::cast_possible_truncation)]
		/* let water_depth_map =  */scene_buffers.create_texture(
			UNDERWATER_CONTEXT,
			WATER_DEPTH_NAME,
			WATER_DEPTH_FORMAT,
			WATER_DEPTH_USAGE,
			WATER_TEXTURES_SAMPLES,
			render_size,
			view_count,
			1,
			true,
			false,
		);
		// if ! rd.texture_is_valid(self.water_depth_map) {
		// 	// godot_print!("Water Depth invalidated");

		// 	self.water_depth_map = water_depth_map;
		// 	// if let Some(texture) = self.water_depth_texture.as_mut() {
		// 	// 	texture.set_texture_rd_rid(self.water_depth_map);
		// 	// }
		// }

		#[allow(clippy::cast_possible_truncation)]
		let vertex_count = (mesh_data.vertex_floats.len() / 4) as u32;
		let (vertex_buffer, vertex_array) = create_vertex_buffers(
			rd,
			&mesh_data.vertex_floats,
			self.vertex_format,
			vertex_count,
		);
		if !vertex_buffer.is_valid() || !vertex_array.is_valid() {
			if vertex_array.is_valid() {
				rd.free_rid(vertex_array);
			}
			if vertex_buffer.is_valid() {
				rd.free_rid(vertex_buffer);
			}
			return Err("Vertex buffers are invalid");
		}

		let (index_buffer, index_array) = create_index_buffers(rd, &mesh_data.indices);
		if !index_buffer.is_valid() || !index_array.is_valid() {
			rd.free_rid(vertex_array);
			rd.free_rid(vertex_buffer);
			if index_array.is_valid() {
				rd.free_rid(index_array);
			}
			if index_buffer.is_valid() {
				rd.free_rid(index_buffer);
			}
			return Err("Index buffers are invalid");
		}

		let info_buffer = create_storage_buffer(rd, &mesh_data.water_info_floats);
		let parameters_buffer = create_storage_buffer(rd, &mesh_data.water_params_floats);
		if !info_buffer.is_valid() || !parameters_buffer.is_valid() {
			rd.free_rid(vertex_array);
			rd.free_rid(vertex_buffer);
			rd.free_rid(index_array);
			rd.free_rid(index_buffer);
			if info_buffer.is_valid() {
				rd.free_rid(info_buffer);
			}
			if parameters_buffer.is_valid() {
				rd.free_rid(parameters_buffer);
			}
			return Err("Storage buffers are invalid");
		}

		let (x_groups, y_groups) = groups_for_size(render_size, 8);
		let screen_size = Vector2i::new((render_size.x - 1).max(1), (render_size.y - 1).max(1));

		for view in 0..view_count {
			let water_map = scene_buffers.get_texture_slice(UNDERWATER_CONTEXT, WATER_MASK_NAME, view, 0, 1, 1);
			let water_depth = scene_buffers.get_texture_slice(UNDERWATER_CONTEXT, WATER_DEPTH_NAME, view, 0, 1, 1);
			if !water_map.is_valid() || !water_depth.is_valid() {
				continue;
			}

			let mut attachments = Array::<Rid>::new();
			attachments.push(water_map);
			attachments.push(water_depth);
			let water_framebuffer = rd
				.framebuffer_create_ex(&attachments)
				.validate_with_format(self.framebuffer_format)
				.done();
			if !water_framebuffer.is_valid() {
				continue;
			}

			let projection = scene_data.get_view_projection(view);
			let world_to_view = Projection::from(scene_data.get_cam_transform().affine_inverse());
			let world_to_clip = projection * world_to_view;

			let rendered_mask = render_mask_pass::run_render_mask_pass(
				rd,
				water_framebuffer,
				self.render_pipeline,
				self.render_shader,
				self.displacement_sampler,
				displacement_texture,
				info_buffer,
				vertex_array,
				index_array,
				world_to_clip,
			);
			if !rendered_mask {
				rd.free_rid(water_framebuffer);
				continue;
			}

			let color = scene_buffers.get_color_layer(view);
			let depth = scene_buffers.get_depth_layer(view);
			if !color.is_valid() || !depth.is_valid() {
				rd.free_rid(water_framebuffer);
				continue;
			}

			apply_effect_pass::run_apply_underwater_pass(
				rd,
				self.compute_pipeline,
				self.compute_shader,
				self.depth_sampler,
				color,
				depth,
				water_map,
				parameters_buffer,
				projection,
				screen_size,
				(x_groups, y_groups),
			);

			rd.free_rid(water_framebuffer);
		}

		rd.free_rid(vertex_array);
		rd.free_rid(vertex_buffer);
		rd.free_rid(index_array);
		rd.free_rid(index_buffer);
		rd.free_rid(info_buffer);
		rd.free_rid(parameters_buffer);

		Ok(())
	}
}

#[godot_api]
impl ICompositorEffect for UnderwaterEffect {
	fn init(base: Base<CompositorEffect>) -> Self {

		let mut init_base = base.to_init_gd();
		init_base.set_effect_callback_type(EffectCallbackType::POST_TRANSPARENT);

		let mut effect = Self {
			base,
			render_shader_file: None,
			// water_mask_texture: None,
			// water_mask_map: Rid::Invalid,
			// water_depth_texture: None,
			// water_depth_map: Rid::Invalid,
			main_view_index: 0,
			water_displacement_texture: None,
			render_shader: Rid::Invalid,
			compute_shader: Rid::Invalid,
			render_pipeline: Rid::Invalid,
			compute_pipeline: Rid::Invalid,
			displacement_sampler: Rid::Invalid,
			depth_sampler: Rid::Invalid,
			framebuffer_format: -1,
			vertex_format: -1
		};

		effect.construct();

		effect
	}

	fn on_notification(&mut self, what: ObjectNotification) {
		if what == ObjectNotification::POSTINITIALIZE {
			self.construct();
		}
		else if what == ObjectNotification::PREDELETE {
			self.destruct();
		}
	}

	fn render_callback(&mut self, effect_callback_type: i32, render_data: Option<Gd<RenderData>>) {
		self.run(effect_callback_type, render_data).unwrap_or_else(|err| {
			self.base_mut().set_enabled(false);
			godot_error!("UnderwaterEffect render_callback error: {}", err);
		});
	}
}

#[godot_api]
impl UnderwaterEffect {
	fn construct_samplers(&mut self, rd: &mut RenderingDevice) {
		let mut displacement_state = RdSamplerState::new_gd();
		displacement_state.set_min_filter(SamplerFilter::LINEAR);
		displacement_state.set_mag_filter(SamplerFilter::LINEAR);
		displacement_state.set_repeat_u(SamplerRepeatMode::REPEAT);
		displacement_state.set_repeat_v(SamplerRepeatMode::REPEAT);
		displacement_state.set_repeat_w(SamplerRepeatMode::REPEAT);
		self.displacement_sampler = rd.sampler_create(&displacement_state);

		let mut depth_state = RdSamplerState::new_gd();
		depth_state.set_min_filter(SamplerFilter::NEAREST);
		depth_state.set_mag_filter(SamplerFilter::NEAREST);
		self.depth_sampler = rd.sampler_create(&depth_state);
	}

	fn construct_render_pipeline(&mut self, rd: &mut RenderingDevice) {
		self.render_shader = if let Some(shader_file) = self.render_shader_file.as_ref() {
			shader_file.get_spirv()
				.map_or(Rid::Invalid, |spirv| rd.shader_create_from_spirv(&spirv))
		} else {
			create_shader_from_sources(
				rd,
				&[
					(ShaderStage::VERTEX, BUILTIN_WATER_MASK_VERTEX_GLSL),
					(ShaderStage::FRAGMENT, BUILTIN_WATER_MASK_FRAGMENT_GLSL)
				],
				ShaderLanguage::GLSL
			)

		};
		if !self.render_shader.is_valid() {
			return;
		}

		let mut water_map_attachment = RdAttachmentFormat::new_gd();
		water_map_attachment.set_format(WATER_MASK_FORMAT);
		water_map_attachment.set_samples(WATER_TEXTURES_SAMPLES);
		#[allow(clippy::cast_possible_truncation)]
		water_map_attachment.set_usage_flags(
			WATER_MASK_USAGE,
		);

		let mut water_depth_attachment = RdAttachmentFormat::new_gd();
		water_depth_attachment.set_format(WATER_DEPTH_FORMAT);
		water_depth_attachment.set_samples(WATER_TEXTURES_SAMPLES);
		#[allow(clippy::cast_possible_truncation)]
		water_depth_attachment.set_usage_flags(
			WATER_DEPTH_USAGE
		);

		let mut attachments = Array::<Gd<RdAttachmentFormat>>::new();
		attachments.push(&water_map_attachment);
		attachments.push(&water_depth_attachment);
		self.framebuffer_format = rd.framebuffer_format_create(&attachments);
		if self.framebuffer_format < 0 {
			return;
		}

		let mut vertex_attribute = RdVertexAttribute::new_gd();
		vertex_attribute.set_format(DataFormat::R32G32B32A32_SFLOAT);
		vertex_attribute.set_location(0);
		#[allow(clippy::cast_possible_truncation)]
		vertex_attribute.set_stride(4 * size_of::<f32>() as u32);
		vertex_attribute.set_offset(0);

		let mut attributes = Array::<Gd<RdVertexAttribute>>::new();
		attributes.push(&vertex_attribute);
		self.vertex_format = rd.vertex_format_create(&attributes);
		if self.vertex_format < 0 {
			return;
		}

		let mut blend_attachment = RdPipelineColorBlendStateAttachment::new_gd();
		blend_attachment.set_write_r(true);
		blend_attachment.set_write_g(true);
		blend_attachment.set_write_b(true);
		blend_attachment.set_write_a(true);
		blend_attachment.set_enable_blend(false);

		let mut blend_attachments = Array::<Gd<RdPipelineColorBlendStateAttachment>>::new();
		blend_attachments.push(&blend_attachment);

		let mut blend_state = RdPipelineColorBlendState::new_gd();
		blend_state.set_attachments(&blend_attachments);

		let mut depth_state = RdPipelineDepthStencilState::new_gd();
		depth_state.set_enable_depth_test(true);
		depth_state.set_enable_depth_write(true);
		depth_state.set_depth_compare_operator(CompareOperator::GREATER_OR_EQUAL);

		let raster_state = RdPipelineRasterizationState::new_gd();
		let multisample_state = RdPipelineMultisampleState::new_gd();

		self.render_pipeline = rd.render_pipeline_create(
			self.render_shader,
			self.framebuffer_format,
			self.vertex_format,
			RenderPrimitive::TRIANGLES,
			&raster_state,
			&multisample_state,
			&depth_state,
			&blend_state,
		);
	}
	fn destruct_render_pipeline(&mut self) {
		let mut rs = RenderingServer::singleton();

		if self.render_pipeline.is_valid() {
			rs.free_rid(self.render_pipeline);
			self.render_pipeline = Rid::Invalid;
		}
		if self.render_shader.is_valid() {
			rs.free_rid(self.render_shader);
			self.render_shader = Rid::Invalid;
		}
	}

	fn construct_compute_pipeline(&mut self, rd: &mut RenderingDevice) {
		self.compute_shader = create_shader_from_source(
			rd,
			UNDERWATER_EFFECT_GLSL,
			ShaderStage::COMPUTE,
			ShaderLanguage::GLSL
		);

		if self.compute_shader.is_valid() {
			self.compute_pipeline = rd.compute_pipeline_create(self.compute_shader);
		}
	}
	fn destruct_compute_pipeline(&mut self) {
		let mut rs = RenderingServer::singleton();

		if self.compute_pipeline.is_valid() {
			rs.free_rid(self.compute_pipeline);
		}
		self.compute_pipeline = Rid::Invalid;

		if self.compute_shader.is_valid() {
			rs.free_rid(self.compute_shader);
		}
		self.compute_shader = Rid::Invalid;
	}

	fn construct(&mut self) {
		let rs = RenderingServer::singleton();
		if let Some(rd) = rs.get_rendering_device().as_deref_mut() {
			self.construct_samplers(rd);
			self.construct_render_pipeline(rd);
			self.construct_compute_pipeline(rd);
		}
	}

	fn destruct(&mut self) {
		let mut rs = RenderingServer::singleton();

		if self.displacement_sampler.is_valid() {
			rs.free_rid(self.displacement_sampler);
			self.displacement_sampler = Rid::Invalid;
		}
		if self.depth_sampler.is_valid() {
			rs.free_rid(self.depth_sampler);
			self.depth_sampler = Rid::Invalid;
		}

		self.destruct_render_pipeline();
		self.destruct_compute_pipeline();

		self.framebuffer_format = -1;
		self.vertex_format = -1;
	}

	#[func]
	fn set_render_shader_file(&mut self, shader_file: Option<Gd<RdShaderFile>>) {
		self.destruct_render_pipeline();

		if let Some(rd) = RenderingServer::singleton().get_rendering_device().as_deref_mut() {
			self.render_shader_file = shader_file;
			self.construct_render_pipeline(rd);
		}
	}

	// #[func]
	// fn get_water_mask_texture(&self) -> Option<Gd<Texture2Drd>> {
	// 	self.water_mask_texture.clone()
	// }

	// #[func]
	// fn set_water_mask_texture(&mut self, &mut texture: Option<Gd<Texture2Drd>>) {
	// 	if let Some(old_texture) = self.water_mask_texture.as_mut() {
	// 		old_texture.set_texture_rd_rid(Rid::Invalid);
	// 	}
	// 	if let Some(new_texture) = texture.as_mut() && self.water_mask_map.is_valid() {
	// 		new_texture.set_texture_rd_rid(self.water_mask_map);
	// 	}
	// 	self.water_mask_texture = texture;
	// }

	// #[func]
	// fn get_water_depth_texture(&self) -> Option<Gd<Texture2Drd>> {
	// 	self.water_depth_texture.clone()
	// }

	// #[func]
	// fn set_water_depth_texture(&mut self, &mut texture: Option<Gd<Texture2Drd>>) {
	// 	if let Some(old_texture) = self.water_depth_texture.as_mut() {
	// 		old_texture.set_texture_rd_rid(Rid::Invalid);
	// 	}
	// 	if let Some(new_texture) = texture.as_mut() && self.water_depth_map.is_valid() {
	// 		new_texture.set_texture_rd_rid(self.water_depth_map);
	// 	}
	// 	self.water_depth_texture = texture;
	// }
}
