use godot::{meta::GodotConvert, prelude::{Export, GString, Var}};
use std::fmt::Display;

#[derive(GodotConvert, Var, Export, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[godot(via = GString)]
pub enum GameTraitModifierOperation {
	Set,
	#[default]
	Multiply,
	Add,
}

impl Display for GameTraitModifierOperation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			GameTraitModifierOperation::Set => write!(f, "Set"),
			GameTraitModifierOperation::Multiply => write!(f, "Multiply"),
			GameTraitModifierOperation::Add => write!(f, "Add"),
		}
	}
}