use std::sync::{Arc, Mutex};

use boundless::{attributes::{AttributeHolder, AttributeProvider}, damage::{DamageDealer, DamageInstance, Damageable}};
use godot::prelude::*;

#[derive(Clone)]
pub struct GodotDamageDealer(DynGd<Node, dyn DamageDealer>);

impl GodotDamageDealer {
	pub fn from(damage_dealer: DynGd<Node, dyn DamageDealer>) -> Self {
		return Self {
			0: damage_dealer,
		}
	}
}

impl AttributeHolder for GodotDamageDealer {
	fn attributes(&self) -> Option<Box<dyn AttributeProvider>> {
		self.0.dyn_bind().attributes()
	}
}

impl DamageDealer for GodotDamageDealer {
	fn award_damage(&mut self, damage: &Arc<Mutex<DamageInstance>>, target: &dyn Damageable) {
		self.0.dyn_bind_mut().award_damage(damage, target);
	}
}