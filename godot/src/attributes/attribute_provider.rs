use std::vec;

use godot::prelude::GodotClass;

use boundless::{attributes::AttributeProvider, id::Id};

use crate::{AttributeModifierCollection, attribute_collection::AttributeCollection};

#[derive(GodotClass, Clone)]
#[class(base=Resource, init, tool, rename=AttributeProvider)]
pub struct GodotAttributeProvider {
	#[export]
	pub attributes: AttributeCollection,
	#[export]
	pub modifiers: AttributeModifierCollection,
}

impl GodotAttributeProvider {
	#[must_use]
	pub fn iter(&self) -> vec::IntoIter<(Id, f32)> {
		<&Self as IntoIterator>::into_iter(self)
	}
	#[must_use]
	pub fn iter_mut(&mut self) -> vec::IntoIter<(Id, f32)> {
		<&mut Self as IntoIterator>::into_iter(self)
	}
}

impl AttributeProvider for GodotAttributeProvider {
	fn get_attribute(&self, id: &Id) -> Option<f32> {
		self.attributes.get_attribute(id)
			.and_then(|base_value|
				self.modifiers.apply_modifiers(id, base_value)
					.or(Some(base_value))
		)
	}
}

impl IntoIterator for GodotAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.attributes
			.into_iter()
			.map(|(gd_id, base_value)| {
				let id = gd_id.into();
				let modified_value = self.modifiers.apply_modifiers(&id, base_value)
					.unwrap_or(base_value);
				(id, modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl  IntoIterator for &GodotAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.attributes)
			.into_iter()
			.map(|(gd_id, base_value)| {
				let id = gd_id.as_id();

				let modified_value = self.modifiers.apply_modifiers(id, *base_value)
					.unwrap_or(*base_value);
				(id.clone(), modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl  IntoIterator for &mut GodotAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.attributes)
			.into_iter()
			.map(|(gd_id, base_value)| {
				let id = gd_id.as_id();

				let modified_value = self.modifiers.apply_modifiers(id, *base_value)
					.unwrap_or(*base_value);
				(id.clone(), modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl FromIterator<(Id, f32)> for GodotAttributeProvider {
	fn from_iter<T: IntoIterator<Item = (Id, f32)>>(iter: T) -> Self {
		let attributes = iter.into_iter().map(|(id, v)| (id.into(), v)).collect();
		Self {
			attributes,
			modifiers: AttributeModifierCollection::default(),
		}
	}
}

impl Extend<(Id, f32)> for GodotAttributeProvider {
	fn extend<T: IntoIterator<Item = (Id, f32)>>(&mut self, iter: T) {
		self.attributes.extend(iter.into_iter().map(|(id, v)| (id.into(), v)));
	}
}