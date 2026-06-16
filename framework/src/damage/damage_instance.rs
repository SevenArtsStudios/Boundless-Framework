use std::ops::Deref;
use std::rc::Rc;
use std::slice::Iter;
use std::sync::Mutex;

use crate::damage::scale_damage;
use crate::{damage::{Damage, DamageDealer, DamageModifier, Damageable}, id::Id};

#[derive(Clone)]
pub struct DamageInstance {
	amount: f32,
	modifiers: Rc<[Rc<dyn DamageModifier>]>,
	target: Rc<Mutex<dyn Damageable>>,
	damage_dealer: Option<Rc<Mutex<dyn DamageDealer>>>,
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
			target: Rc::new(Mutex::new(target)),
			damage_dealer: damage_dealer.map(|dd| Rc::new(Mutex::new(dd)) as Rc<Mutex<dyn DamageDealer>>)
		}
	}

	pub fn inflict(self) {
		let rc = Rc::new(Mutex::new(self));

		let (
			modifiers,
			target,
			damage_dealer
		) = {
			let guard = rc.lock().unwrap();
			(guard.modifiers.clone(), guard.target.clone(), guard.damage_dealer.clone())
		};

		for modifier in modifiers.iter() {
			modifier.apply(rc.clone());
			modifier.add_effects(rc.clone());
		}

		target.lock().unwrap().damage(rc.lock().unwrap().deref());

		if let Some(dealer_rc) = damage_dealer {
			let mut dealer = dealer_rc.lock().unwrap();
			let target_ref = target.lock().unwrap();
			dealer.award_damage(rc.lock().unwrap().deref(), &*target_ref);
		}
	}

	#[must_use]
	pub const fn amount(&self) -> f32 {
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
			&*target_ref,
			dealer_ref.as_deref(),
			allow_negative,
		);
	}

	pub fn modifiers(& self) -> Iter<'_, Rc<dyn DamageModifier>> {
		self.modifiers.iter()
	}

	#[must_use]
	pub fn target(&self) -> Rc<Mutex<dyn Damageable>> {
		self.target.clone()
	}

	#[must_use]
	pub fn damage_dealer(&self) -> Option<Rc<Mutex<dyn DamageDealer>>> {
		self.damage_dealer.clone()
	}
}

unsafe impl Sync for DamageInstance {}