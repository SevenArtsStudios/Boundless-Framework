use godot::{builtin::{Color, StringName, Variant}, classes::{IMeshInstance3D, Mesh, MeshInstance3D}, meta::ToGodot, obj::{Base, Gd, InstanceId, WithBaseField}, prelude::{GodotClass, godot_api}};

use crate::framework::WaterMeshRegistry;


#[derive(GodotClass)]
#[class(base = MeshInstance3D, tool)]
pub struct WaterMesh {
	#[base]
	base: Base<MeshInstance3D>,
	tracked_mesh_id: Option<InstanceId>,

	#[var(set = set_shallow_color)]
	#[export(color_no_alpha)]
	shallow_color: Color,

	#[var(set = set_deep_color)]
	#[export(color_no_alpha)]
	deep_color: Color,

	#[var(set = set_water_intensity)]
	#[export]
	water_intensity: f32,

	#[var(set = set_water_scale)]
	#[export]
	water_scale: f32,

	#[var(set = set_fog_distance)]
	#[export]
	fog_distance: f32,

	#[var(set = set_fog_fade)]
	#[export]
	fog_fade: f32,

	#[var(set = set_transparency_distance)]
	#[export]
	transparency_distance: f32,

	#[var(set = set_transparency_fade)]
	#[export]
	transparency_fade: f32,
}

#[godot_api]
impl IMeshInstance3D for WaterMesh {
	fn init(base: Base<MeshInstance3D>) -> Self {
		let base_gd = base.to_init_gd();
		base_gd.signals()
			.visibility_changed()
			.connect(WaterMeshRegistry::schedule_rebuild);

		let tracked_mesh_id = Self::connect_mesh_callbacks(base_gd.get_mesh());

		Self {
			base,
			tracked_mesh_id,
			shallow_color: Color::from_rgb(0.0, 0.45, 0.75),
			deep_color: Color::from_rgb(0.0, 0.08, 0.18),
			water_intensity: 1.0,
			water_scale: 1.0,
			fog_distance: 25.0,
			fog_fade: 10.0,
			transparency_distance: 25.0,
			transparency_fade: 10.0,
		}
	}

	fn enter_tree(&mut self) {
		self.refresh_mesh_callbacks();
		WaterMeshRegistry::add_id(self.base().instance_id());
	}

	fn exit_tree(&mut self) {
		WaterMeshRegistry::remove_id(self.base().instance_id());
	}

	fn on_set(&mut self, property: StringName, value: Variant) -> bool {
		if property != "mesh" {
			return false;
		}

		let Ok(mesh) = value.try_to::<Option<Gd<Mesh>>>() else {
			return false;
		};

		self.set_base_mesh(mesh);
		true
	}
}

#[godot_api]
impl WaterMesh {
	fn connect_mesh_callbacks(mesh: Option<Gd<Mesh>>) -> Option<InstanceId> {
		let Some(mesh) = mesh else {
			return None;
		};

		mesh.signals().changed().connect(WaterMeshRegistry::schedule_rebuild);
		Some(mesh.instance_id())
	}

	fn refresh_mesh_callbacks(&mut self) {
		let current_mesh_id = self.base().get_mesh().map(|mesh| mesh.instance_id());
		if self.tracked_mesh_id == current_mesh_id {
			return;
		}

		self.tracked_mesh_id = Self::connect_mesh_callbacks(self.base().get_mesh());
		WaterMeshRegistry::schedule_rebuild();
	}

	fn set_base_mesh(&mut self, mesh: Option<Gd<Mesh>>) {
		match mesh.as_ref() {
			Some(mesh_ref) => self.base_mut().set_mesh(mesh_ref),
			None => self.base_mut().set_mesh(Option::<&Gd<Mesh>>::None),
		}

		self.refresh_mesh_callbacks();
	}

	#[func]
	pub fn set_shallow_color(&mut self, color: Color) {
		self.shallow_color = color;
		if let Some(mesh) = self.base().get_mesh() {
			if let Some(mut material) = mesh.surface_get_material(0) {
				material.set("shader_parameter/shallow_color", &color.to_variant());
			}
		}
	}

	#[func]
	pub fn set_deep_color(&mut self, color: Color) {
		self.deep_color = color;
	}

	#[func]
	pub fn set_water_intensity(&mut self, intensity: f32) {
		self.water_intensity = intensity;
	}

	#[func]
	pub fn set_water_scale(&mut self, scale: f32) {
		self.water_scale = scale;
	}

	#[func]
	pub fn set_fog_distance(&mut self, distance: f32) {
		self.fog_distance = distance;
	}

	#[func]
	pub fn set_fog_fade(&mut self, fade: f32) {
		self.fog_fade = fade;
	}

	#[func]
	pub fn set_transparency_distance(&mut self, distance: f32) {
		self.transparency_distance = distance;
	}

	#[func]
	pub fn set_transparency_fade(&mut self, fade: f32) {
		self.transparency_fade = fade;
	}


	pub fn get_water_mesh(&self) -> Option<Gd<Mesh>> {
		self.base().get_mesh()
	}

	pub fn shallow_color_value(&self) -> Color {
		self.shallow_color
	}

	pub fn deep_color_value(&self) -> Color {
		self.deep_color
	}

	pub fn water_intensity_value(&self) -> f32 {
		self.water_intensity
	}

	pub fn water_scale_value(&self) -> f32 {
		self.water_scale
	}

	pub fn fog_distance_value(&self) -> f32 {
		self.fog_distance
	}

	pub fn fog_fade_value(&self) -> f32 {
		self.fog_fade
	}

	pub fn transparency_distance_value(&self) -> f32 {
		self.transparency_distance
	}

	pub fn transparency_fade_value(&self) -> f32 {
		self.transparency_fade
	}
}