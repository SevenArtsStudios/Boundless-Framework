use godot::{classes::IResource, obj::{Base, WithBaseField}, prelude::{GodotClass, Resource, godot_api}, register::info::{PropertyInfo, PropertyUsageFlags}};

use crate::framework::GameTraitModifierOperation;

pub trait TraitModifier {
	fn apply_modifiers(&self, base_value: f32) -> Option<f32>;
}

#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=TraitModifier)]
pub struct BaseTraitModifier {
	#[var(set = set_operation)]
	#[export]
	#[init(val=GameTraitModifierOperation::Multiply)]
	pub operation: GameTraitModifierOperation,
	#[export]
	#[init(val=1.0)]
	pub value: f32,
	#[export]
	pub is_additive: bool,

	#[base]
	base: Base<Resource>,
}

#[godot_api]
impl BaseTraitModifier {
	pub const IS_ADDITIVE_PROPERTY: &'static str = "is_additive";
	pub const OPERATION_PROPERTY: &'static str = "operation";

	pub fn apply_to(&self, base_value: f32, multiplier: f32) -> f32 {
		match self.operation {
			GameTraitModifierOperation::Set => self.value * multiplier,
			GameTraitModifierOperation::Multiply => base_value * self.value * multiplier,
			GameTraitModifierOperation::Add => base_value + self.value * multiplier,
		}
	}

	#[func]
	fn set_operation(&mut self, operation: GameTraitModifierOperation) {
		self.operation = operation;
		self.base().signals().property_list_changed().emit();
	}
}

#[godot_api]
impl IResource for BaseTraitModifier {
	// This is broken in gdext for now, see https://github.com/godot-rust/gdext/issues/1427

	fn on_validate_property(&self, property: &mut PropertyInfo) {
		if property.property_name != Self::IS_ADDITIVE_PROPERTY {
			return;
		}

		property.usage =
			if self.operation == GameTraitModifierOperation::Multiply {
				PropertyUsageFlags::DEFAULT
			} else {
				PropertyUsageFlags::NONE
			};
	}
}

impl TraitModifier for BaseTraitModifier {
	fn apply_modifiers(&self, base_value: f32) -> Option<f32> {
		Some(self.apply_to(base_value, 1.0))
	}
}