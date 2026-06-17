use boundless::attributes::{AttributeModifierEntry};
use godot::{obj::{Gd, OnEditor}, prelude::GodotClass};

use crate::GodotAttributeModifier;


#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=AttributeModifierEntry)]
pub struct GodotAttributeModifierEntry {
	#[export]
	pub modifier: OnEditor<Gd<GodotAttributeModifier>>,
	#[export]
	#[init(val=1.0)]
	pub multiplier: f32,
}

impl GodotAttributeModifierEntry {
	pub fn from(modifier: Gd<GodotAttributeModifier>, multiplier: Option<f32>) -> Self {
		let mut sentinel: OnEditor<Gd<GodotAttributeModifier>> = OnEditor::default();
		sentinel.init(modifier);

		Self {
			modifier: sentinel,
			multiplier: multiplier.unwrap_or(1.0)
		}
	}
	pub fn as_entry(&self) -> AttributeModifierEntry {
		AttributeModifierEntry::new(self.modifier.bind().as_modifier(), self.multiplier)
	}
}

impl From<GodotAttributeModifierEntry> for AttributeModifierEntry {
	fn from(val: GodotAttributeModifierEntry) -> Self {
		val.as_entry()
	}
}