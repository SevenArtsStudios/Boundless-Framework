use boundless::{attributes::AttributeProvider, damage::{DamageDealer, DamageInstance, Damageable}, id::Id};
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

impl AttributeProvider for GodotDamageDealer {
	fn get_value(&self, id: &Id) -> Option<f32> {
		self.0.dyn_bind().get_value(id)
	}
}

impl DamageDealer for GodotDamageDealer {
	fn award_damage(&mut self, damage: &DamageInstance, target: &dyn Damageable) {
		self.0.dyn_bind_mut().award_damage(damage, target);
	}
}