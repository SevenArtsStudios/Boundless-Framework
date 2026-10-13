use godot::{classes::{CompositorEffect, ICompositorEffect, RdSamplerState, RdTextureFormat, RdTextureView, RenderData, RenderingDevice, RenderingServer, Texture2Drd, Time, compositor_effect::EffectCallbackType, notify::ObjectNotification, rendering_device::{DataFormat, SamplerFilter, SamplerRepeatMode, ShaderLanguage, ShaderStage, TextureSamples, TextureType, TextureUsageBits}}, prelude::*};

use crate::{rendering::{create_shader_from_source}, displacement_effect::{compute_pass, fetch_pass}};


pub(crate) const BUILTIN_COMPUTE_WATER_DISPLACEMENT_GLSL: &str =
	include_str!("../../assets/shaders/ComputeWaterDisplacement.glsl");
pub(crate) const BUILTIN_FETCH_WATER_DISPLACEMENT_GLSL: &str =
	include_str!("../../assets/shaders/FetchWaterDisplacement.glsl");


#[derive(GodotClass)]
#[class(base = CompositorEffect, tool)]
pub struct WaterDisplacementEffect {
	base: Base<CompositorEffect>,

	#[var(get = get_render_texture, set = set_render_texture)]
	#[export]
	render_texture: Option<Gd<Texture2Drd>>,

	#[export]
	fetch_water_displacement: bool,

	compute_shader: Rid,
	compute_pipeline: Rid,
	fetch_shader: Rid,
	fetch_pipeline: Rid,
	displacement_sampler: Rid,
	water_displacement_map: Rid,
}

impl WaterDisplacementEffect {
	fn run(&mut self, effect_callback_type: i32, _render_data: Option<Gd<RenderData>>) -> Result<(), &'static str> {
		if effect_callback_type != EffectCallbackType::PRE_TRANSPARENT.ord() {
			return Err("Effect callback type is not PRE_TRANSPARENT");
		}

		if !self.compute_pipeline.is_valid() || !self.compute_shader.is_valid() {
			return Err("Effect is not properly initialized");
		}

		let rs = RenderingServer::singleton();

		let Some(mut rendering_device) = rs.get_rendering_device() else {
			return Err("Rendering_device is not available");
		};
		let rd = &mut *rendering_device;


		if !self.water_displacement_map.is_valid() {
			self.construct_displacement_map(rd);
		}
		if !self.water_displacement_map.is_valid() {
			return Err("Water_displacement_map is not valid");
		}

		let render_size = Vector2i::new(128, 128);
		let t = Time::singleton().get_ticks_msec() as f32 / 1000.0;
		compute_pass::run_displacement_compute_pass(
			rd,
			self.compute_pipeline,
			self.compute_shader,
			self.water_displacement_map,
			render_size,
			t,
		);

		if self.fetch_water_displacement {
			fetch_pass::run_fetch_displacement_pass(
				rd,
				self.fetch_pipeline,
				self.fetch_shader,
				self.displacement_sampler,
				self.water_displacement_map,
			);
		}

		Ok(())
	}
}

#[godot_api]
impl ICompositorEffect for WaterDisplacementEffect {
	fn init(base: Base<CompositorEffect>) -> Self {
		let mut init_base = base.to_init_gd();
		init_base.set_effect_callback_type(EffectCallbackType::PRE_TRANSPARENT);

		let mut effect = Self {
			base,
			render_texture: None,
			fetch_water_displacement: false,
			compute_shader: Rid::Invalid,
			compute_pipeline: Rid::Invalid,
			fetch_shader: Rid::Invalid,
			fetch_pipeline: Rid::Invalid,
			displacement_sampler: Rid::Invalid,
			water_displacement_map: Rid::Invalid,
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
			godot_error!("WaterDisplacementEffect render_callback error: {}", err);
		});
	}
}

#[godot_api]
impl WaterDisplacementEffect {
	fn construct_sampler(&mut self, rd: &mut RenderingDevice) {
		let mut sampler_state = RdSamplerState::new_gd();
		sampler_state.set_min_filter(SamplerFilter::LINEAR);
		sampler_state.set_mag_filter(SamplerFilter::LINEAR);
		sampler_state.set_repeat_u(SamplerRepeatMode::REPEAT);
		sampler_state.set_repeat_v(SamplerRepeatMode::REPEAT);
		sampler_state.set_repeat_w(SamplerRepeatMode::REPEAT);

		self.displacement_sampler = rd.sampler_create(&sampler_state);
	}

	fn construct_compute_pipeline(&mut self, rd: &mut RenderingDevice) {
		self.compute_shader = create_shader_from_source(
			rd,
			BUILTIN_COMPUTE_WATER_DISPLACEMENT_GLSL,
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

	fn construct_fetch_pipeline(&mut self, rd: &mut RenderingDevice) {
		self.fetch_shader = create_shader_from_source(
			rd,
			BUILTIN_FETCH_WATER_DISPLACEMENT_GLSL,
			ShaderStage::COMPUTE,
			ShaderLanguage::GLSL
		);

		if self.fetch_shader.is_valid() {
			self.fetch_pipeline = rd.compute_pipeline_create(self.fetch_shader);
		}
	}
	fn destruct_fetch_pipeline(&mut self) {
		let mut rs = RenderingServer::singleton();

		if self.fetch_pipeline.is_valid() {
			rs.free_rid(self.fetch_pipeline);
		}
		self.fetch_pipeline = Rid::Invalid;

		if self.fetch_shader.is_valid() {
			rs.free_rid(self.fetch_shader);
		}
		self.fetch_shader = Rid::Invalid;
	}

	fn construct_displacement_map(&mut self, rd: &mut RenderingDevice) {
		if self.water_displacement_map.is_valid() {
			return;
		}

		let mut format = RdTextureFormat::new_gd();
		format.set_width(128);
		format.set_height(128);
		format.set_texture_type(TextureType::TYPE_2D);
		format.set_format(DataFormat::R8G8B8A8_UNORM);
		format.set_samples(TextureSamples::SAMPLES_1);
		format.set_usage_bits(
			TextureUsageBits::STORAGE_BIT
				| TextureUsageBits::SAMPLING_BIT
				| TextureUsageBits::CAN_COPY_FROM_BIT,
		);

		let view = RdTextureView::new_gd();
		self.water_displacement_map = rd.texture_create(&format, &view);

		if let Some(texture) = self.render_texture.as_mut() {
			texture.set_texture_rd_rid(self.water_displacement_map);
		}
	}

	fn construct(&mut self) {
		let rs = RenderingServer::singleton();
		if let Some(rd) = rs.get_rendering_device().as_deref_mut() {
			self.construct_sampler(rd);
			self.construct_compute_pipeline(rd);
			self.construct_fetch_pipeline(rd);
			self.construct_displacement_map(rd);
		}
	}

	fn destruct(&mut self) {
		let mut rs = RenderingServer::singleton();

		if let Some(texture) = self.render_texture.as_mut() {
			texture.set_texture_rd_rid(Rid::Invalid);
		}

		if self.displacement_sampler.is_valid() {
			rs.free_rid(self.displacement_sampler);
			self.displacement_sampler = Rid::Invalid;
		}
		if self.water_displacement_map.is_valid() {
			rs.free_rid(self.water_displacement_map);
			self.water_displacement_map = Rid::Invalid;
		}

		self.destruct_compute_pipeline();
		self.destruct_fetch_pipeline();
	}


	#[func]
	fn get_render_texture(&self) -> Option<Gd<Texture2Drd>> {
		self.render_texture.clone()
	}

	#[func]
	fn set_render_texture(&mut self, &mut texture: Option<Gd<Texture2Drd>>) {
		if let Some(old_texture) = self.render_texture.as_mut() {
			old_texture.set_texture_rd_rid(Rid::Invalid);
		}
		if let Some(new_texture) = texture.as_mut() {
			new_texture.set_texture_rd_rid(self.water_displacement_map);
		}
		self.render_texture = texture;
	}
}