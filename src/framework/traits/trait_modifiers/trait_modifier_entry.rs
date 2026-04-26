use godot::{obj::Gd, prelude::GodotClass};

use crate::framework::GdTraitModifier;


#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
pub struct TraitModifierEntry {
	#[export]
	pub modifier: Option<Gd<GdTraitModifier>>,
	#[export]
	#[init(val=1.0)]
	pub multiplier: f32,
}