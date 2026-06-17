use boundless::attributes::{AttributeModifier, AttributeModifierOperation, ModifiedAttributeValue};
use godot::{classes::IResource, obj::{Base, WithBaseField}, prelude::*, register::info::{PropertyInfo, PropertyUsageFlags}};

use crate::AttributeModifierOperator;

#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=AttributeModifier)]
pub struct GodotAttributeModifier {
	#[var(set = set_operation)]
	#[export]
	#[init(val=AttributeModifierOperator::Multiply)]
	pub operation: AttributeModifierOperator,
	#[export]
	#[init(val=1.0)]
	pub value: f32,
	#[export]
	pub is_stacking: bool,
	#[export]
	pub is_deferred: bool,

	#[base]
	base: Base<Resource>,
}

#[godot_api]
impl GodotAttributeModifier {
	pub const IS_STACKING_PROPERTY: &'static str = "is_stacking";
	pub const IS_DEFERRED_PROPERTY: &'static str = "is_deferred";
	pub const OPERATION_PROPERTY: &'static str = "operation";

	#[func]
	fn set_operation(&mut self, operation: AttributeModifierOperator) {
		self.operation = operation;
		self.base().signals().property_list_changed().emit();
	}
}

impl AttributeModifier for GodotAttributeModifier {
	fn apply_to(&self, base_value: f32) -> ModifiedAttributeValue {
		ModifiedAttributeValue::Modified(match self.operation {
			AttributeModifierOperator::Set => self.value,
			AttributeModifierOperator::Multiply => self.value * base_value,
			AttributeModifierOperator::Add => base_value + self.value,
			AttributeModifierOperator::MoreThan => base_value.max(self.value),
			AttributeModifierOperator::LessThan => base_value.min(self.value),
		})
	}

	fn operation(&self) -> AttributeModifierOperation {
		match self.operation {
			AttributeModifierOperator::Set => AttributeModifierOperation::Set { to: self.value },
			AttributeModifierOperator::Multiply => AttributeModifierOperation::Multiply { with: self.value, stacking: self.is_stacking },
			AttributeModifierOperator::Add => AttributeModifierOperation::Add { value: self.value },
			AttributeModifierOperator::MoreThan => AttributeModifierOperation::MoreThan { minimum: self.value, deferred: self.is_deferred },
			AttributeModifierOperator::LessThan => AttributeModifierOperation::LessThan { maximum: self.value, deferred: self.is_deferred },
		}
	}
}

#[godot_api]
impl IResource for GodotAttributeModifier {
	// This is broken in gdext for now, see https://github.com/godot-rust/gdext/issues/1427

	fn on_validate_property(&self, property: &mut PropertyInfo) {
		match property.property_name.to_string().as_str() {
			Self::IS_STACKING_PROPERTY => {
				property.usage = match self.operation {
					AttributeModifierOperator::Multiply => PropertyUsageFlags::DEFAULT,
					_ => PropertyUsageFlags::NONE,
				}
			},
			Self::IS_DEFERRED_PROPERTY => {
				property.usage = match self.operation {
					AttributeModifierOperator::MoreThan | AttributeModifierOperator::LessThan => PropertyUsageFlags::DEFAULT,
					_ => PropertyUsageFlags::NONE,
				};
			},
			_ => {},
		}
	}
}

pub struct GodotAttributeModifierWrapper(Gd<GodotAttributeModifier>);

impl GodotAttributeModifierWrapper {
	pub const fn wrap(item: Gd<GodotAttributeModifier>) -> Self {
		Self(item)
	}
}

impl AttributeModifier for GodotAttributeModifierWrapper {
	fn apply_to(&self, base_value: f32) -> ModifiedAttributeValue {
		self.0.bind().apply_to(base_value)
	}
	fn operation(&self) -> AttributeModifierOperation {
		self.0.bind().operation()
	}
	fn strength(&self) -> f32 {
		self.0.bind().strength()
	}
}

impl From<Gd<GodotAttributeModifier>> for GodotAttributeModifierWrapper {
	fn from(value: Gd<GodotAttributeModifier>) -> Self {
		Self::wrap(value)
	}
}