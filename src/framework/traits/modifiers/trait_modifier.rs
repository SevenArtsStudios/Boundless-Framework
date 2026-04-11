use godot::{classes::IResource, obj::Base, prelude::{GodotClass, Resource, godot_api}};

use crate::framework::GameTraitModifierOperation;

pub trait TraitModifier {
	fn apply_modifiers(&self, base_value: f32) -> Option<f32>;
}

#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=TraitModifier)]
pub struct GdTraitModifier {
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

impl GdTraitModifier {
	pub const IS_ADDITIVE_PROPERTY: &'static str = "is_additive";
	pub const OPERATION_PROPERTY: &'static str = "operation";

	pub fn apply_to(&self, base_value: f32, multiplier: f32) -> f32 {
		match self.operation {
			GameTraitModifierOperation::Set => self.value * multiplier,
			GameTraitModifierOperation::Multiply => base_value * self.value * multiplier,
			GameTraitModifierOperation::Add => base_value + self.value * multiplier,
		}
	}
}

#[godot_api]
impl IResource for GdTraitModifier {
	// This is broken in gdext for now, see https://github.com/godot-rust/gdext/issues/1427

	// fn on_validate_property(&self, property: &mut PropertyInfo) {
	// 	if property.variant_type != VariantType::BOOL {
	// 		return;
	// 	}

	// 	property.usage =
	// 		if self.operation == GameTraitModifierOperation::Multiply {
	// 			PropertyUsageFlags::DEFAULT
	// 		} else {
	// 			PropertyUsageFlags::NONE
	// 		};
	// }
}

impl TraitModifier for GdTraitModifier {
	fn apply_modifiers(&self, base_value: f32) -> Option<f32> {
		Some(self.apply_to(base_value, 1.0))
	}
}