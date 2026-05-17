use crate::framework::{AsNode, Damage, Damageable, DamageableProvider, TraitsHolder};

pub trait DamageDealer: DamageableProvider + TraitsHolder + AsNode {
	fn award_damage(&mut self, _damage: &Damage, _target: &mut dyn Damageable) { }
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