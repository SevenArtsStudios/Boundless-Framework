use godot::prelude::GodotClass;

use crate::framework::{GameTrait, TraitModifierCollection, TraitsCollection};

pub trait TraitsProvider {
	fn get_value(&self, r#trait: &GameTrait) -> Option<f32>;
}

#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename=TraitsProvider)]
pub struct GdTraitsProvider {
	#[export]
	pub traits: TraitsCollection,
	#[export]
	pub modifiers: TraitModifierCollection,
}

impl TraitsProvider for GdTraitsProvider {
	fn get_value(&self, r#trait: &GameTrait) -> Option<f32> {
		if let Some(base_value) = self.traits.get_value(r#trait) {
			let modified_value = self.modifiers.apply_modifiers(r#trait, base_value);
			modified_value
		} else {
			None
		}
	}
}