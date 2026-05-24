use godot::prelude::GodotClass;

use crate::framework::{Id, AttributeModifierCollection, AttributeCollection};

pub trait AttributeProvider {
	fn get_value(&self, id: &Id) -> Option<f32>;
}

#[derive(GodotClass, Clone)]
#[class(base=Resource, init, tool, rename=AttributeProvider)]
pub struct BaseAttributeProvider {
	#[export]
	pub attributes: AttributeCollection,
	#[export]
	pub modifiers: AttributeModifierCollection,
}

impl AttributeProvider for BaseAttributeProvider {
	fn get_value(&self, id: &Id) -> Option<f32> {
		if let Some(base_value) = self.attributes.get_value(id) {
			self.modifiers.apply_modifiers(id, base_value)
		} else {
			None
		}
	}
}

impl<'a> AttributeProvider for &'a BaseAttributeProvider {
	fn get_value(&self, id: &Id) -> Option<f32> {
		if let Some(base_value) = self.attributes.get_value(id) {
			self.modifiers.apply_modifiers(id, base_value)
		} else {
			None
		}
	}
}

impl IntoIterator for BaseAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.attributes
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

impl <'a> IntoIterator for &'a BaseAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.attributes)
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

impl <'a> IntoIterator for &'a mut BaseAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.attributes)
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

impl FromIterator<(Id, f32)> for BaseAttributeProvider {
	fn from_iter<T: IntoIterator<Item = (Id, f32)>>(iter: T) -> Self {
		let attributes = iter.into_iter().collect();
		Self {
			attributes,
			modifiers: AttributeModifierCollection::default(),
		}
	}
}

impl Extend<(Id, f32)> for BaseAttributeProvider {
	fn extend<T: IntoIterator<Item = (Id, f32)>>(&mut self, iter: T) {
		self.attributes.extend(iter);
	}
}