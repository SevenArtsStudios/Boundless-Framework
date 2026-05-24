use godot::{meta::GodotConvert, prelude::{Export, GString, Var}};
use std::fmt::Display;

#[derive(GodotConvert, Var, Export, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[godot(via = GString)]
pub enum AttributeModifierOperation {
	Set,
	#[default]
	Multiply,
	Add,
}

impl Display for AttributeModifierOperation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AttributeModifierOperation::Set => write!(f, "Set"),
			AttributeModifierOperation::Multiply => write!(f, "Multiply"),
			AttributeModifierOperation::Add => write!(f, "Add"),
		}
	}
}