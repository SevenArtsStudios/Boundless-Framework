use std::sync::{Arc, Mutex};
use crate::{damage::{DamageDealer, DamageInstance, Damageable}, id::Id};

pub trait DamageModifier {
	fn modify(
		&mut self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		let _ = damage;
	}

	fn apply(
		&mut self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		let _ = damage;
	}
}


pub fn scale_damage(
	base_amount: f32,
	resistance_trait: &Id,
	power_trait: &Id,
	target: &dyn Damageable,
	damage_dealer: Option<&dyn DamageDealer>,
	allow_negative: bool,
) -> f32 {
	let mut modified_amount: f32 = base_amount;

	if let Some(dealer_traits) = damage_dealer.and_then(|d| d.attributes()) {
		let mut strength_value = dealer_traits.get_value(power_trait).unwrap_or(1.0);
		if !allow_negative {
			strength_value = strength_value.max(0.0);
		}
		modified_amount *= strength_value;
	}

	if let Some(target_traits) = target.attributes() {
		let mut resistance_value = target_traits.get_value(resistance_trait).unwrap_or(1.0);
		if !allow_negative {
			resistance_value = resistance_value.max(0.0);
		}
		if resistance_value != 0.0 {
			modified_amount /= resistance_value;
		}
	}

	modified_amount
}