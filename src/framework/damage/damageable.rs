use std::ops::DerefMut;

use crate::framework::{AsNode, DamageDealerProvider, TraitsHolder};

pub trait Damageable: DamageDealerProvider + TraitsHolder + AsNode {
	fn get_health(&self) -> Option<f32>;

	fn apply_damage(&mut self, amount: f32);
	fn kill(&mut self) { }
}

pub trait DamageableProvider {
	fn get_damageable(&mut self) -> Option<impl DerefMut<Target = impl Damageable> + '_>
	where
		Self: Sized;
}

impl<T> DamageableProvider for T
	where
		T: Damageable
{
	fn get_damageable(&mut self) -> Option<impl DerefMut<Target = impl Damageable> + '_>
	{
		Some::<&mut Self>(self)
	}
}