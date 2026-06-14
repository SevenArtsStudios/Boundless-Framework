use crate::{attributes::AttributeHolder};

pub trait Damageable: AttributeHolder {
	fn get_health(&self) -> Option<f32>;

	fn apply_damage(&mut self, amount: f32);
	fn kill(&mut self) { }
}