use godot::{classes::CharacterBody3D, obj::{Base, Gd}, prelude::{GodotClass, godot_api}, register::godot_dyn};

use boundless::{attributes::{AttributeHolder, AttributeProvider}, damage::{DamageDealer, Damageable}};
use boundless_godot_macros::godot_damageable;

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

#[godot_damageable]
#[godot_api]
impl GodotEntity {}

impl AttributeHolder for GodotEntity {
	fn attributes(&self) -> Option<Box<dyn AttributeProvider>> {
		self.attributes.as_ref().map(|gd| {
			let bound = gd.bind();
			Box::new((*bound).clone()) as Box<dyn AttributeProvider>
		})
	}
}

#[godot_dyn]
impl DamageDealer for GodotEntity {}

#[godot_dyn]
impl Damageable for GodotEntity {
	fn apply_damage(&mut self, amount: f32) {
		self.health -= amount;
	}

	fn get_health(&self) -> Option<f32> {
		Some(self.health)
	}
}