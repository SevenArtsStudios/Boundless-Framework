use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::damage::scale_damage;
use crate::{damage::{Damage, DamageDealer, DamageModifier, Damageable}, id::Id};

#[derive(Clone)]
pub struct DamageInstance {
	amount: f32,
	modifiers: Arc<Mutex<Vec<Arc<Mutex<dyn DamageModifier>>>>>,
	target: Arc<Mutex<dyn Damageable>>,
	damage_dealer: Option<Arc<Mutex<dyn DamageDealer>>>,
}

impl DamageInstance {
	pub fn from(
		damage: &mut Damage,
		target: impl Damageable + 'static,
		damage_dealer: Option<Arc<Mutex<dyn DamageDealer>>>,
	) -> Self {
		Self::new(damage.amount, damage.modifiers.clone(), target, damage_dealer)
	}

	pub fn new(
		amount: f32,
		modifiers: Vec<Arc<Mutex<dyn DamageModifier>>>,
		target: impl Damageable + 'static,
		damage_dealer: Option<Arc<Mutex<dyn DamageDealer>>>,
	) -> Self {
		Self {
			amount,
			modifiers: Arc::new(Mutex::new(modifiers)),
			target: Arc::new(Mutex::new(target)),
			damage_dealer,
		}
	}

	pub fn inflict(self) {
		let mods = {
			let mut m = self.modifiers.lock().unwrap();
			std::mem::take(&mut *m)
		};

		let arc = Arc::new(Mutex::new(self));

		for modifier in mods.iter() {
			modifier.lock().unwrap().modify(arc.clone());
			modifier.lock().unwrap().apply(arc.clone());
		}

		let m = arc.lock().unwrap();
		m.modifiers.lock().expect("").extend(mods);

		if let Some(ref dealer_arc) = m.damage_dealer {
			let mut dealer = dealer_arc.lock().unwrap();
			let target_ref = m.target.lock().unwrap();
			dealer.award_damage(&arc, &*target_ref);
		}
	}

	pub fn amount(&self) -> f32 {
		self.amount
	}

	pub fn scale(
		&mut self,
		resistance_trait: &Id,
		power_trait: &Id,
		allow_negative: bool,
	) {
		let target_ref = self.target.lock().unwrap();
		let dealer_ref = self.damage_dealer.as_ref().map(|d| d.lock().unwrap());

		self.amount = scale_damage(
			self.amount(),
			resistance_trait,
			power_trait,
			target_ref.deref(),
			dealer_ref.as_ref().map(|d| d.deref()),
			allow_negative,
		);
	}

	pub fn modifiers(&self) -> Arc<Mutex<Vec<Arc<Mutex<dyn DamageModifier>>>>> {
		self.modifiers.clone()
	}

	pub fn target(&self) -> Arc<Mutex<dyn Damageable>> {
		self.target.clone()
	}

	pub fn damage_dealer(&self) -> Option<Arc<Mutex<dyn DamageDealer>>> {
		self.damage_dealer.clone()
	}
}