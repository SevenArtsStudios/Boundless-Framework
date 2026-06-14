use crate::{attributes::AttributeProvider, damage::{DamageInstance, Damageable}};

pub trait DamageDealer: AttributeProvider {
	fn award_damage(&mut self, damage: &DamageInstance, target: &dyn Damageable) {
		let (_, _) = (damage, target);
	}
}