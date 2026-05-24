use crate::framework::{AsNode, DamageDealerProvider, AttributeHolder};

pub trait Damageable: DamageDealerProvider + AttributeHolder + AsNode {
	fn get_health(&self) -> Option<f32>;

	fn apply_damage(&mut self, amount: f32);
	fn kill(&mut self) { }
}

pub trait DamageableProvider {
	fn get_damageable(&mut self) -> Option<&mut dyn Damageable>;
}

impl<T> DamageableProvider for T
	where
		T: Damageable
{
	fn get_damageable(&mut self) -> Option<&mut dyn Damageable> {
		Some(self)
	}
}