use crate::damage::{DamageDealer, DamageInstance, Damageable};

pub trait DamageDealerProxy: DamageDealer {
	fn sender(&self) -> Option<impl DamageDealer>;

	fn award_proxy_damage(&mut self, damage: &DamageInstance, target: &dyn Damageable) {
		let (_, _) = (damage, target);
	}
}

impl<T: DamageDealerProxy> DamageDealer for T {
	fn award_damage(&mut self, damage: &DamageInstance, target: &dyn Damageable) {
		self.award_proxy_damage(damage, target);
		if let Some(mut sender) = self.sender() {
			sender.award_damage(damage, target);
		}
	}
}