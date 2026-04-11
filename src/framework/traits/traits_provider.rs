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
			self.modifiers.apply_modifiers(r#trait, base_value)
		} else {
			None
		}
	}
}

impl IntoIterator for GdTraitsProvider {
	type Item = (GameTrait, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.traits
			.into_iter()
			.map(|(r#trait, base_value)| {
				let modified_value = self.modifiers.apply_modifiers(&r#trait, base_value)
					.unwrap_or(base_value);
				(r#trait, modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl <'a> IntoIterator for &'a GdTraitsProvider {
	type Item = (GameTrait, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.traits)
			.into_iter()
			.map(|(r#trait, base_value)| {
				let modified_value = self.modifiers.apply_modifiers(&r#trait, *base_value)
					.unwrap_or(*base_value);
				(r#trait.clone(), modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl <'a> IntoIterator for &'a mut GdTraitsProvider {
	type Item = (GameTrait, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.traits)
			.into_iter()
			.map(|(r#trait, base_value)| {
				let modified_value = self.modifiers.apply_modifiers(&r#trait, *base_value)
					.unwrap_or(*base_value);
				(r#trait.clone(), modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl FromIterator<(GameTrait, f32)> for GdTraitsProvider {
	fn from_iter<T: IntoIterator<Item = (GameTrait, f32)>>(iter: T) -> Self {
		let traits = iter.into_iter().collect();
		Self {
			traits,
			modifiers: TraitModifierCollection::default(),
		}
	}
}

impl Extend<(GameTrait, f32)> for GdTraitsProvider {
	fn extend<T: IntoIterator<Item = (GameTrait, f32)>>(&mut self, iter: T) {
		self.traits.extend(iter);
	}
}