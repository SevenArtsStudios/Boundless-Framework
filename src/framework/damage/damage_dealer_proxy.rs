use std::ops::DerefMut;

use crate::framework::{Damage, DamageDealer, Damageable};

pub trait DamageDealerProxy: DamageDealer {
	fn sender(&self) -> Option<impl DerefMut<Target = impl DamageDealer> + '_>;

	fn award_proxy_damage<T: Damageable>(&mut self, _damage: &Damage, _target: &mut T) { }

	fn award_damage<T: Damageable>(&mut self, damage: &Damage, target: &mut T) {
		self.award_proxy_damage(damage, target);
		if let Some(mut sender) = self.sender() {
			sender.award_damage(damage, target);
		}
	}
}
