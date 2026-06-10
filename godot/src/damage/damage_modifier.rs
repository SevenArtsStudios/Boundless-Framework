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
	pub fn modify_damage(
		&self,
		damage: Gd<GodotDamageInstance>,
	) {
		let _ = damage;
	}

	#[func(virtual)]
	pub fn apply_damage(
		&self,
		damage: Gd<GodotDamageInstance>
	) {
		let _ = damage;
	}
}

#[godot_dyn]
impl DamageModifier for GodotDamageModifier {
	fn modify(
		&mut self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		let godot_damage = Gd::from_object(GodotDamageInstance::from(damage));

		let _ = self.base_mut().call(
			"modify_damage",
			&[
				Variant::from(godot_damage)
			],
		);
	}

	fn apply(
		&mut self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		let godot_damage = Gd::from_object(GodotDamageInstance::from(damage));

		let _ = self.base_mut().call(
			"apply_damage",
			&[
				Variant::from(godot_damage)
			],
		);
	}
}