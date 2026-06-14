use std::sync::{Arc, Mutex};
use crate::{damage::{DamageDealer, DamageInstance, Damageable}, id::Id};

pub trait DamageModifier {
	fn apply(
		&self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		let _ = damage;
	}

	fn add_effects(
		&self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		let _ = damage;
	}
}


pub fn scale_damage(
	base_amount: f32,
	resistance_attribute: &Id,
	strength_attribute: &Id,
	target: &dyn Damageable,
	damage_dealer: Option<&dyn DamageDealer>,
	allow_negative: bool,
) -> f32 {
	let mut modified_amount: f32 = base_amount;

	if let Some(dealer) = damage_dealer {
		let mut strength_value = dealer.get_value(strength_attribute).unwrap_or(1.0);
		if !allow_negative {
			strength_value = strength_value.max(0.0);
		}
		modified_amount *= strength_value;
	}

	let mut resistance_value = target.get_value(resistance_attribute).unwrap_or(1.0);
	if !allow_negative {
		resistance_value = resistance_value.max(0.0);
	}
	if resistance_value != 0.0 {
		modified_amount /= resistance_value;
	}

	modified_amount
}