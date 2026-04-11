use godot::prelude::GodotClass;

use crate::framework::{Id, TraitModifierCollection, TraitsCollection};

pub trait TraitsProvider {
	fn get_value(&self, id: &Id) -> Option<f32>;
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
	fn get_value(&self, id: &Id) -> Option<f32> {
		if let Some(base_value) = self.traits.get_value(id) {
			self.modifiers.apply_modifiers(id, base_value)
		} else {
			None
		}
	}
}

impl IntoIterator for GdTraitsProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.traits
			.into_iter()
			.map(|(id, base_value)| {
				let modified_value = self.modifiers.apply_modifiers(&id, base_value)
					.unwrap_or(base_value);
				(id, modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl <'a> IntoIterator for &'a GdTraitsProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.traits)
			.into_iter()
			.map(|(id, base_value)| {
				let modified_value = self.modifiers.apply_modifiers(&id, *base_value)
					.unwrap_or(*base_value);
				(id.clone(), modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl <'a> IntoIterator for &'a mut GdTraitsProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.traits)
			.into_iter()
			.map(|(id, base_value)| {
				let modified_value = self.modifiers.apply_modifiers(&id, *base_value)
					.unwrap_or(*base_value);
				(id.clone(), modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl FromIterator<(Id, f32)> for GdTraitsProvider {
	fn from_iter<T: IntoIterator<Item = (Id, f32)>>(iter: T) -> Self {
		let traits = iter.into_iter().collect();
		Self {
			traits,
			modifiers: TraitModifierCollection::default(),
		}
	}
}

impl Extend<(Id, f32)> for GdTraitsProvider {
	fn extend<T: IntoIterator<Item = (Id, f32)>>(&mut self, iter: T) {
		self.traits.extend(iter);
	}
}