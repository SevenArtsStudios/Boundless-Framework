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

impl AttributeProvider for GodotAttributeProvider {
	fn get_value(&self, id: &Id) -> Option<f32> {
		if let Some(base_value) = self.attributes.get_value(id) {
			self.modifiers.apply_modifiers(id, base_value)
				.or(Some(base_value))
		} else {
			None
		}
	}
}

impl IntoIterator for GodotAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

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

impl <'a> IntoIterator for &'a GodotAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.attributes)
			.into_iter()
			.map(|(gd_id, base_value)| {
				let id = gd_id.as_id();

				let modified_value = self.modifiers.apply_modifiers(&id, *base_value)
					.unwrap_or(*base_value);
				(id.clone(), modified_value)
			})
			.collect::<Vec<_>>()
			.into_iter()
	}
}

impl <'a> IntoIterator for &'a mut GodotAttributeProvider {
	type Item = (Id, f32);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		(&self.attributes)
			.into_iter()
			.map(|(gd_id, base_value)| {
				let id = gd_id.as_id();

				let modified_value = self.modifiers.apply_modifiers(&id, *base_value)
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