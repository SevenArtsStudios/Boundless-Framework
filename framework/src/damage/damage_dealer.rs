use std::sync::{Arc, Mutex};
use crate::{attributes::AttributeHolder, damage::{DamageInstance, Damageable}};

pub trait DamageDealer: AttributeHolder {
	fn award_damage(&mut self, _damage: &Arc<Mutex<DamageInstance>>, _target: &dyn Damageable) { }
}