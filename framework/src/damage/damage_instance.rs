use std::ops::Deref;
use std::slice::Iter;
use std::sync::{Arc, Mutex};

use crate::damage::scale_damage;
use crate::{damage::{Damage, DamageDealer, DamageModifier, Damageable}, id::Id};

#[derive(Clone)]
pub struct DamageInstance {
	amount: f32,
	modifiers: Arc<[Arc<dyn DamageModifier>]>,
	target: Arc<Mutex<dyn Damageable>>,
	damage_dealer: Option<Arc<Mutex<dyn DamageDealer>>>,
}

impl DamageInstance {
	pub fn from(
		damage: &Damage,
		target: impl Damageable + 'static,
		damage_dealer: Option<impl DamageDealer + 'static>,
	) -> Self {
		Self {
			amount: damage.amount,
			modifiers: damage.modifiers.clone(),
			target: Arc::new(Mutex::new(target)),
			damage_dealer: damage_dealer.map(|dd| Arc::new(Mutex::new(dd)) as Arc<Mutex<dyn DamageDealer>>)
		}
	}

	pub fn inflict(self) {
		let arc = Arc::new(Mutex::new(self));

		let (
			modifiers,
			target,
			damage_dealer
		) = {
			let guard = arc.lock().unwrap();
			(guard.modifiers.clone(), guard.target.clone(), guard.damage_dealer.clone())
		};

		for modifier in modifiers.iter() {
			modifier.apply(arc.clone());
			modifier.add_effects(arc.clone());
		}

		target.lock().unwrap().damage(arc.lock().unwrap().deref());

		if let Some(dealer_arc) = damage_dealer {
			let mut dealer = dealer_arc.lock().unwrap();
			let target_ref = target.lock().unwrap();
			dealer.award_damage(arc.lock().unwrap().deref(), &*target_ref);
		}
	}

	pub fn amount(&self) -> f32 {
		self.amount
	}

	pub fn scale(
		&mut self,
		resistance_attribute: &Id,
		strength_attribute: &Id,
		allow_negative: bool,
	) {
		let target_ref = self.target.lock().unwrap();
		let dealer_ref = self.damage_dealer.as_ref().map(|d| d.lock().unwrap());

		self.amount = scale_damage(
			self.amount(),
			resistance_attribute,
			strength_attribute,
			target_ref.deref(),
			dealer_ref.as_ref().map(|d| d.deref()),
			allow_negative,
		);
	}

	pub fn modifiers<'a>(&'a self) -> Iter<'a, Arc<dyn DamageModifier>> {
		self.modifiers.iter()
	}

	pub fn target(&self) -> Arc<Mutex<dyn Damageable>> {
		self.target.clone()
	}

	pub fn damage_dealer(&self) -> Option<Arc<Mutex<dyn DamageDealer>>> {
		self.damage_dealer.clone()
	}
}