use std::collections::HashSet;

use godot::classes::Resource;

use crate::framework::{DamageDealer, DamageModifier, Damageable, ObjectHandle, ObjectKey};

type DamageModifierHandle = ObjectHandle<dyn DamageModifier, Resource>;

#[derive(Default)]
pub struct Damage {
	pub modifiers: Vec<DamageModifierHandle>,
	pub amount: f32,
}

impl Damage {
	pub fn new(
		amount: f32,
		modifiers: impl IntoIterator<Item = impl Into<DamageModifierHandle>>,
	) -> Self {
		Self {
			amount,
			modifiers: modifiers.into_iter().map(Into::into).collect(),
		}
	}

	pub fn compute_damage_amount(
		&mut self,
		target: &dyn Damageable,
		damage_dealer: Option<&dyn DamageDealer>,
	) -> f32 {
		let mut total_damage_amount = self.amount;

		for modifier in &mut self.modifiers {
			let modified_amount = modifier
				.bind_mut()
				.modify(self.amount, target, damage_dealer);

			total_damage_amount += modified_amount - self.amount;
		}

		total_damage_amount
	}

	pub fn inflict_upon(
		&mut self,
		target: &mut dyn Damageable,
		mut damage_dealer: Option<&mut dyn DamageDealer>,
	) {
		let amount = self.compute_damage_amount(target, damage_dealer.as_deref());
		target.apply_damage(amount);

		if let Some(dealer) = damage_dealer.as_deref_mut() {
			for modifier in &mut self.modifiers {
				modifier
					.bind_mut()
					.apply(amount, target, Some(&mut *dealer));
			}

			dealer.award_damage(self, target);
		} else {
			for modifier in &mut self.modifiers {

				modifier.bind_mut().apply(amount, target, None);
			}
		}
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

pub(crate) fn unique_modifiers(
	modifiers: impl IntoIterator<Item = impl Into<DamageModifierHandle>>,
) -> Vec<DamageModifierHandle> {
	let mut unique: Vec<DamageModifierHandle> = Vec::new();
	let mut seen: HashSet<ObjectKey> = HashSet::new();

	for modifier in modifiers {
		let modifier = modifier.into();
		if seen.insert(modifier.key()) {
			unique.push(modifier);
		}
	}

	unique
}

pub(crate) fn modifier_set_equals<'a>(
	x: impl IntoIterator<Item = &'a DamageModifierHandle>,
	y: impl IntoIterator<Item = &'a DamageModifierHandle>,
) -> bool {
	let x_modifiers: HashSet<ObjectKey> = x.into_iter().map(DamageModifierHandle::key).collect();
	let y_modifiers: HashSet<ObjectKey> = y.into_iter().map(DamageModifierHandle::key).collect();

	x_modifiers == y_modifiers
}
