use godot::{classes::CharacterBody3D, obj::{Base, Gd}, prelude::{GodotClass, godot_api}, register::godot_dyn};

use boundless::{attributes::AttributeProvider, damage::{DamageDealer, DamageInstance, Damageable}, entity::Entity, id::Id};

use crate::attribute_provider::GodotAttributeProvider;


#[derive(GodotClass)]
#[class(base=CharacterBody3D, init, tool, rename=Entity)]
pub struct GodotEntity {
	#[export]
	pub attributes: Option<Gd<GodotAttributeProvider>>,

	#[export]
	pub health: f32,

	#[base]
	pub base: Base<CharacterBody3D>,
}

#[godot_api]
impl GodotEntity {}

#[godot_dyn]
impl Entity for GodotEntity {}

#[godot_dyn]
impl AttributeProvider for GodotEntity {
	fn get_value(&self, id: &Id) -> Option<f32> {
		match &self.attributes {
			Some(attrs) => attrs.bind().get_value(id),
			None => None
		}
	}
}

#[godot_dyn]
impl DamageDealer for GodotEntity {}

#[godot_dyn]
impl Damageable for GodotEntity {
	fn damage(&mut self, damage: &DamageInstance) {
		self.health -= damage.amount();
	}

	fn get_health(&self) -> Option<f32> {
		Some(self.health)
	}
}