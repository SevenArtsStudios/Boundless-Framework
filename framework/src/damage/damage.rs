use std::{collections::HashSet, sync::{Arc, Mutex}};

use crate::damage::{DamageDealer, DamageInstance, DamageModifier, Damageable};

#[derive(Default)]
pub struct Damage {
	pub modifiers: Vec<Arc<Mutex<dyn DamageModifier>>>,
	pub amount: f32,
}

impl Damage {
	pub fn new(
		amount: f32,
		modifiers: impl IntoIterator<Item = Arc<Mutex<dyn DamageModifier>>>,
	) -> Self {
		Self {
			amount,
			modifiers: modifiers.into_iter().collect(),
		}
	}


	pub fn inflicted_upon(
		&mut self,
		target: impl Damageable + 'static,
		damage_dealer: Option<Arc<Mutex<dyn DamageDealer>>>,
	) -> DamageInstance {
		DamageInstance::from(self, target, damage_dealer)
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

pub fn unique_modifiers(
	modifiers: impl IntoIterator<Item = Arc<Mutex<dyn DamageModifier>>>,
) -> Vec<Arc<Mutex<dyn DamageModifier>>> {
	let mut unique: Vec<Arc<Mutex<dyn DamageModifier>>> = Vec::new();
	let mut seen: HashSet<*const dyn DamageModifier> = HashSet::new();

	for modifier in modifiers {
		let modifier_ptr = {
			let borrow = modifier.lock().unwrap();
			&*borrow as *const dyn DamageModifier
		};
		if seen.insert(modifier_ptr) {
			unique.push(modifier);
		}
	}

	unique
}

pub fn modifier_set_equals<'a>(
	x: impl IntoIterator<Item = &'a Arc<Mutex<dyn DamageModifier>>>,
	y: impl IntoIterator<Item = &'a Arc<Mutex<dyn DamageModifier>>>,
) -> bool {
	let x_modifiers: HashSet<*const dyn DamageModifier> = x
		.into_iter()
		.map(|modifier| &*modifier.lock().unwrap() as *const dyn DamageModifier)
		.collect();
	let y_modifiers: HashSet<*const dyn DamageModifier> = y
		.into_iter()
		.map(|modifier| &*modifier.lock().unwrap() as *const dyn DamageModifier)
		.collect();

	x_modifiers == y_modifiers
}
