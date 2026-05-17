use godot::{classes::CharacterBody3D, obj::{Base, Gd}, prelude::{GodotClass, godot_api}, register::godot_dyn};

use boundless_macros::godot_damageable;

use crate::framework::{DamageDealer, Damageable, BaseTraitsProvider, TraitsHolder, TraitsProvider};



#[derive(GodotClass)]
#[class(base=CharacterBody3D, init, tool)]
pub struct Entity {
	#[export]
	pub traits: Option<Gd<BaseTraitsProvider>>,

	#[export]
	pub health: f32,

	#[base]
	pub base: Base<CharacterBody3D>,
}

#[godot_damageable]
#[godot_api]
impl Entity {}

impl TraitsHolder for Entity {
	fn traits(&self) -> Option<Box<dyn TraitsProvider>> {
		self.traits.as_ref().map(|gd| {
			let bound = gd.bind();
			Box::new((*bound).clone()) as Box<dyn TraitsProvider>
		})
	}
}

#[godot_dyn]
impl DamageDealer for Entity {}

#[godot_dyn]
impl Damageable for Entity {
	fn apply_damage(&mut self, amount: f32) {
		self.health -= amount;
	}

	fn get_health(&self) -> Option<f32> {
		Some(self.health)
	}
}