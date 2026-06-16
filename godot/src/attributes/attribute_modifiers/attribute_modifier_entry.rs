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
		self.modifier.as_ref()
			.map_or_else(
				|| AttributeModifierEntry::new(AttributeModifier::Multiply(1.0, false), self.multiplier),
				|modifier| AttributeModifierEntry::new(modifier.bind().as_modifier(), self.multiplier)
			)
	}
}

impl From<GodotAttributeModifierEntry> for AttributeModifierEntry {
	fn from(val: GodotAttributeModifierEntry) -> Self {
		val.as_entry()
	}
}