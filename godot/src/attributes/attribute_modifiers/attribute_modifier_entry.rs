use boundless::attributes::{AttributeModifier, AttributeModifierEntry};
use godot::{obj::Gd, prelude::GodotClass};

use crate::GodotAttributeModifier;


#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=AttributeModifierEntry)]
pub struct GodotAttributeModifierEntry {
	#[export]
	pub modifier: Option<Gd<GodotAttributeModifier>>,
	#[export]
	#[init(val=1.0)]
	pub multiplier: f32,
}

impl GodotAttributeModifierEntry {
	pub fn as_entry(&self) -> AttributeModifierEntry {
		if let Some(modifier) = &self.modifier {
			AttributeModifierEntry::new(modifier.bind().as_modifier(), self.multiplier)
		} else {
			AttributeModifierEntry::new(AttributeModifier::Multiply(1.0, false), self.multiplier)
		}
	}
}

impl Into<AttributeModifierEntry> for GodotAttributeModifierEntry {
	fn into(self) -> AttributeModifierEntry {
		self.as_entry()
	}
}