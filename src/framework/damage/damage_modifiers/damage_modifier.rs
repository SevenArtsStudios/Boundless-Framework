use godot::{classes::{Node, Resource}, obj::{Base, DynGd, WithBaseField}, prelude::{GodotClass, Variant, godot_api}, register::godot_dyn};

use crate::framework::{DamageDealer, Damageable, Id};

pub trait DamageModifier {
	fn modify(
		&mut self,
		base_amount: f32,
		target: &dyn Damageable,
		damage_dealer: &Option<&dyn DamageDealer>,
	) -> f32 {
		let _ = (target, damage_dealer);
		base_amount
	}

	fn apply(
		&mut self,
		final_amount: f32,
		target: &mut dyn Damageable,
		damage_dealer: &Option<&mut dyn DamageDealer>,
	) {
		let _ = (final_amount, target, damage_dealer);
	}
}

#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=DamageModifier)]
pub struct BaseDamageModifier {
	#[base]
	base: Base<Resource>,
}

#[godot_api]
impl BaseDamageModifier {
	#[func(virtual)]
	pub fn modify_damage(
		&self,
		base_amount: f32,
		target: Option<DynGd<Node, dyn Damageable>>,
		damage_dealer: Option<DynGd<Node, dyn DamageDealer>>,
	) -> f32 {
		let _ = (target, damage_dealer);
		base_amount
	}

	#[func(virtual)]
	pub fn apply_damage(
		&self,
		final_amount: f32,
		target: Option<DynGd<Node, dyn Damageable>>,
		damage_dealer: Option<DynGd<Node, dyn DamageDealer>>,
	) {
		let _ = (final_amount, target, damage_dealer);
	}

	#[func]
	pub fn scale_damage(
		&self,
		base_amount: f32,
		resistance_trait: Id,
		power_trait: Id,
		target: DynGd<Node, dyn Damageable>,
		damage_dealer: Option<DynGd<Node, dyn DamageDealer>>,
		allow_negative: bool,
	) -> f32 {

		let damage_dealer_obj = damage_dealer;

		let target_guard = target.dyn_bind();
		let dealer_guard = damage_dealer_obj.as_ref().map(|d| d.dyn_bind());

		scale_damage(
			base_amount,
			&resistance_trait,
			&power_trait,
			&*target_guard,
			dealer_guard.as_ref().map(|g| &**g),
			allow_negative,
		)
	}
}

#[godot_dyn]
impl DamageModifier for BaseDamageModifier {
	fn modify(
		&mut self,
		base_amount: f32,
		target: &dyn Damageable,
		damage_dealer: &Option<&dyn DamageDealer>,
	) -> f32 {
		let node_target = target.as_node();
		let node_dealer = damage_dealer.as_ref().and_then(|dealer| dealer.as_node());

		let return_value = self.base_mut().call(
			"modify_damage",
			&[
				Variant::from(base_amount),
				Variant::from(node_target),
				Variant::from(node_dealer),
			],
		);

		return_value.try_to::<f32>().unwrap_or(base_amount)
	}

	fn apply(
		&mut self,
		final_amount: f32,
		target: &mut dyn Damageable,
		damage_dealer: &Option<&mut dyn DamageDealer>,
	) {
		let node_target = target.as_node();
		let node_dealer = damage_dealer.as_ref().and_then(|dealer| dealer.as_node());

		let _ = self.base_mut().call(
			"apply_damage",
			&[
				Variant::from(final_amount),
				Variant::from(node_target),
				Variant::from(node_dealer),
			],
		);
	}
}


pub fn scale_damage(
	base_amount: f32,
	resistance_trait: &Id,
	power_trait: &Id,
	target: &dyn Damageable,
	damage_dealer: Option<&dyn DamageDealer>,
	allow_negative: bool,
) -> f32 {
	let mut modified_amount: f32 = base_amount;

	if let Some(dealer_traits) = damage_dealer.and_then(|d| d.traits()) {
		let mut strength_value = dealer_traits.get_value(power_trait).unwrap_or(1.0);
		if !allow_negative {
			strength_value = strength_value.max(0.0);
		}
		modified_amount *= strength_value;
	}

	if let Some(target_traits) = target.traits() {
		let mut resistance_value = target_traits.get_value(resistance_trait).unwrap_or(1.0);
		if !allow_negative {
			resistance_value = resistance_value.max(0.0);
		}
		if resistance_value != 0.0 {
			modified_amount /= resistance_value;
		}
	}

	modified_amount
}