use std::sync::{Arc, Mutex};
use godot::prelude::*;

use boundless::{damage::{DamageInstance, DamageModifier}};

use crate::{GodotDamageInstance};

#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=DamageModifier)]
pub struct GodotDamageModifier {
	#[base]
	base: Base<Resource>,
}

#[godot_api]
impl GodotDamageModifier {
	#[func(virtual)]
	pub fn apply(
		&self,
		damage: Gd<GodotDamageInstance>,
	) {
		let _ = damage;
	}

	#[func(virtual)]
	pub fn add_effects(
		&self,
		damage: Gd<GodotDamageInstance>
	) {
		let _ = damage;
	}
}

#[godot_dyn]
impl DamageModifier for GodotDamageModifier {
	fn apply(
		&self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		let godot_damage = GodotDamageInstance::gd_from(damage);
		let _ = self.apply(godot_damage);
	}

	fn add_effects(
		&self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		let godot_damage = GodotDamageInstance::gd_from(damage);
		let _ = self.add_effects(godot_damage);
	}
}

#[derive(Hash, Eq, PartialEq)]
pub struct DamageModifierWrapper {
	modifier: DynGd<Resource, dyn DamageModifier>,
}

impl DamageModifier for DamageModifierWrapper {
	fn apply(
		&self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		self.modifier.dyn_bind().apply(damage)
	}

	fn add_effects(
		&self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		self.modifier.dyn_bind().add_effects(damage);
	}
}

impl From<DynGd<Resource, dyn DamageModifier>> for DamageModifierWrapper {
	fn from(value: DynGd<Resource, dyn DamageModifier>) -> Self {
		Self {
			modifier: value
		}
	}
}