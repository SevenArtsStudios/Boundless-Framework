use godot::{obj::Gd, prelude::GodotClass};

use crate::framework::BaseAttributeModifier;


#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
pub struct AttributeModifierEntry {
	#[export]
	pub modifier: Option<Gd<BaseAttributeModifier>>,
	#[export]
	#[init(val=1.0)]
	pub multiplier: f32,
}