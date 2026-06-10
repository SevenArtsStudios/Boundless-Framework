use std::sync::{Arc, Mutex};
use crate::{attributes::AttributeHolder, damage::{DamageInstance, Damageable}};

pub trait DamageDealer: AttributeHolder {
	fn award_damage(&mut self, _damage: &Arc<Mutex<DamageInstance>>, _target: &dyn Damageable) { }
}

pub trait DamageDealerProvider {
	fn get_damage_dealer(&mut self) -> Option<&mut dyn DamageDealer>;
}

impl<T> DamageDealerProvider for T
	where
		T: DamageDealer
{
	fn get_damage_dealer(&mut self) -> Option<&mut dyn DamageDealer> {
		Some(self)
	}
}