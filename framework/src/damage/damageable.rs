use crate::{attributes::AttributeProvider, damage::DamageInstance};

pub trait Damageable: AttributeProvider {
	fn get_health(&self) -> Option<f32>;

	fn damage(&mut self, damage: &DamageInstance);
	fn kill(&mut self) { }
}