use std::fmt::Display;

use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[godot(via = GString)]
pub enum AttributeModifierOperation {
	Set,
	#[default]
	Multiply,
	Add,
	MoreThan,
	LessThan,
}

impl Display for AttributeModifierOperation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AttributeModifierOperation::Set => write!(f, "Set"),
			AttributeModifierOperation::Multiply => write!(f, "Multiply"),
			AttributeModifierOperation::Add => write!(f, "Add"),
			AttributeModifierOperation::MoreThan => write!(f, "More Than"),
			AttributeModifierOperation::LessThan => write!(f, "Less Than"),
		}
	}
}