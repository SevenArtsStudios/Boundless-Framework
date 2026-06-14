use std::{collections::HashSet, sync::Arc};

use crate::damage::{DamageDealer, DamageInstance, DamageModifier, Damageable};

#[derive(Default)]
pub struct Damage {
	pub amount: f32,
	pub modifiers: Arc<[Arc<dyn DamageModifier>]>,
}

impl Damage {
	pub fn new(
		amount: f32,
		modifiers: impl IntoIterator<Item = Arc<dyn DamageModifier>>,
	) -> Self {
		Self {
			amount,
			modifiers: modifiers.into_iter().collect(),
		}
	}

	pub fn instantiate(
		&self,
		target: impl Damageable + 'static,
		damage_dealer: Option<impl DamageDealer + 'static>,
	) -> DamageInstance {
		DamageInstance::from(self, target, damage_dealer)
	}

	pub fn inflict(
		&self,
		target: impl Damageable + 'static,
		damage_dealer: Option<impl DamageDealer + 'static>,
	) {
		let instance = self.instantiate(target, damage_dealer);
		instance.inflict();
	}
}


pub fn flatten_damages(damages: impl IntoIterator<Item = Damage>) -> Vec<Damage> {
	let mut flattened: Vec<Damage> = Vec::new();

	for damage in damages {
		if let Some(existing) = flattened.iter_mut().find(|entry| {
			modifier_set_equals(entry.modifiers.iter(), damage.modifiers.iter())
		}) {
			existing.amount += damage.amount;
		} else {
			flattened.push(damage);
		}
	}

	flattened
}

pub fn modifier_set_equals<'a>(
	x: impl IntoIterator<Item = &'a Arc<dyn DamageModifier>>,
	y: impl IntoIterator<Item = &'a Arc<dyn DamageModifier>>,
) -> bool {
	let x_modifiers: HashSet<*const dyn DamageModifier> = x
		.into_iter()
		.map(|modifier| &*modifier.as_ref() as *const dyn DamageModifier)
		.collect();
	let y_modifiers: HashSet<*const dyn DamageModifier> = y
		.into_iter()
		.map(|modifier| &*modifier.as_ref() as *const dyn DamageModifier)
		.collect();

	x_modifiers == y_modifiers
}
