use crate::{attributes::AttributeHolder};

pub trait Damageable: AttributeHolder {
	fn get_health(&self) -> Option<f32>;

	fn apply_damage(&mut self, amount: f32);
	fn kill(&mut self) { }
}

pub trait DamageableProvider {
	fn get_damageable<'a>(&'a mut self) -> Option<&'a mut dyn Damageable>;
}

impl<T> DamageableProvider for T
	where
		T: Damageable
{
	fn get_damageable<'a>(&'a mut self) -> Option<&'a mut dyn Damageable> {
		Some(self)
	}
}