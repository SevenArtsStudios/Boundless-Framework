use std::ops::Deref;

use godot::{obj::Gd, prelude::GodotClass};

use crate::framework::{DamageDealer, Damageable, GdTraitsProvider, TraitsHolder, TraitsProvider};



#[derive(GodotClass)]
#[class(base=CharacterBody3D, init, tool)]
pub struct Entity {
	#[export]
	pub traits: Option<Gd<GdTraitsProvider>>,

	#[export]
	pub health: f32,
}

impl TraitsHolder for Entity {
	fn traits(&self) -> Option<impl Deref<Target = impl TraitsProvider> + '_> {
		self.traits.as_ref().map(Gd::bind)
	}
}

impl DamageDealer for Entity {
	fn get_damageable(&self) -> Option<impl Deref<Target = impl Damageable> + '_> {
		Some(self)
	}
}

impl Damageable for Entity {
	fn get_damage_dealer(&self) -> Option<impl Deref<Target = impl DamageDealer> + '_> {
		Some(self)
	}

	fn apply_damage(&mut self, amount: f32) {
		self.health -= amount;
	}

	fn get_health(&self) -> Option<f32> {
		Some(self.health)
	}
}