use std::ops::Deref;

use crate::framework::{DamageDealer, TraitsHolder};

pub trait Damageable: TraitsHolder {
	fn get_damage_dealer(&self) -> Option<impl Deref<Target = impl DamageDealer> + '_>
	where
		Self: DamageDealer + Sized,
	{
		None::<&Self>
	}

	fn apply_damage(&mut self, amount: f32);
	fn get_health(&self) -> Option<f32>;
}