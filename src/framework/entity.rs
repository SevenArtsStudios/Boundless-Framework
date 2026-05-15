use std::ops::Deref;

use godot::{classes::{CharacterBody3D}, obj::{Base, Gd}, prelude::{GodotClass, godot_api}};

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
	fn traits(&self) -> Option<impl Deref<Target = impl TraitsProvider> + '_> {
		self.traits.as_ref().map(Gd::bind)
	}
}

impl DamageDealer for Entity {}

impl Damageable for Entity {
	fn apply_damage(&mut self, amount: f32) {
		self.health -= amount;
	}

	fn get_health(&self) -> Option<f32> {
		Some(self.health)
	}
}