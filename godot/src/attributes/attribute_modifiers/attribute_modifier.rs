use boundless::attributes::AttributeModifier;
use godot::{classes::IResource, obj::{Base, WithBaseField}, prelude::{GodotClass, Resource, godot_api}, register::info::{PropertyInfo, PropertyUsageFlags}};

use crate::AttributeModifierOperation;

#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=AttributeModifier)]
pub struct GodotAttributeModifier {
	#[var(set = set_operation)]
	#[export]
	#[init(val=AttributeModifierOperation::Multiply)]
	pub operation: AttributeModifierOperation,
	#[export]
	#[init(val=1.0)]
	pub value: f32,
	#[export]
	pub is_additive: bool,
	#[export]
	pub is_initial: bool,

	#[base]
	base: Base<Resource>,
}

#[godot_api]
impl GodotAttributeModifier {
	pub const IS_ADDITIVE_PROPERTY: &'static str = "is_additive";
	pub const IS_INITIAL_PROPERTY: &'static str = "is_initial";
	pub const OPERATION_PROPERTY: &'static str = "operation";

	#[func]
	fn set_operation(&mut self, operation: AttributeModifierOperation) {
		self.operation = operation;
		self.base().signals().property_list_changed().emit();
	}

	pub fn as_modifier(&self) -> AttributeModifier {
		match self.operation {
			AttributeModifierOperation::Multiply => AttributeModifier::Multiply(self.value, self.is_additive),
			AttributeModifierOperation::Add => AttributeModifier::Add(self.value),
			AttributeModifierOperation::Set => AttributeModifier::Set(self.value),
			AttributeModifierOperation::MoreThan => AttributeModifier::MoreThan(self.value, self.is_initial),
			AttributeModifierOperation::LessThan => AttributeModifier::LessThan(self.value, self.is_initial),
		}
	}
}

impl Into<AttributeModifier> for GodotAttributeModifier {
	fn into(self) -> AttributeModifier {
		return self.as_modifier();
	}
}

#[godot_api]
impl IResource for GodotAttributeModifier {
	// This is broken in gdext for now, see https://github.com/godot-rust/gdext/issues/1427

	fn on_validate_property(&self, property: &mut PropertyInfo) {
		match property.property_name.to_string().as_str() {
			Self::IS_ADDITIVE_PROPERTY => {
				property.usage = match self.operation {
					AttributeModifierOperation::Multiply => PropertyUsageFlags::DEFAULT,
					_ => PropertyUsageFlags::NONE,
				}
			},
			Self::IS_INITIAL_PROPERTY => {
				property.usage = match self.operation {
					AttributeModifierOperation::MoreThan => PropertyUsageFlags::DEFAULT,
					AttributeModifierOperation::LessThan => PropertyUsageFlags::DEFAULT,
					_ => PropertyUsageFlags::NONE,
				};
			},
			_ => {},
		}
	}
}