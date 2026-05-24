use std::collections::HashMap;

use godot::meta::{Element, ToArg};
use godot::meta::conv::ByValue;
use godot::meta::shape::{ClassHeritage, GodotElementShape, GodotShape};
use godot::prelude::*;

use crate::framework::{Id, AttributeModifierOperation, BaseAttributeModifier, AttributeModifierEntries, AttributeModifierEntry};


#[derive(GodotClass, Default, Clone)]
#[class(init, tool)]
pub struct AttributeModifierCollection {
	pub modifiers_by_attribute: HashMap<Id, Gd<AttributeModifierEntries>>,
}

#[godot_api]
impl AttributeModifierCollection {
	pub fn shape() -> GodotShape {
		GodotShape::TypedDictionary {
			key: Id::ELEMENT_SHAPE,
			value: GodotElementShape::Class {
				class_id: AttributeModifierEntries::class_id(),
				heritage: ClassHeritage::Resource
			}
		}
	}

	pub fn apply_modifiers(&self, id: &Id, base_value: f32) -> Option<f32> {
		let Some(modifiers) = self.modifiers_by_attribute.get(id) else {
			return None;
		};

		let mut sum = 0.0;
		let mut multiplier = 1.0;

		for entry in modifiers.bind().iter() {
			let entry_ref = entry.bind();
			if let Some(modifier) = &entry_ref.modifier {
				let modifier_ref = modifier.bind();
				match *modifier_ref {
					BaseAttributeModifier { operation: AttributeModifierOperation::Multiply, is_additive: true, ..} => {
						multiplier *= modifier_ref.apply_to(base_value, entry_ref.multiplier) / base_value
					},
					_ => sum += modifier_ref.apply_to(base_value, entry_ref.multiplier) - base_value,
				}
			}
		}

		Some((base_value + sum) * multiplier)
	}


	#[func]
	pub fn add(&mut self, id: Id, modifier: Gd<BaseAttributeModifier>, multiplier: f32) {
		self.modifiers_by_attribute
			.entry(id)
			.or_default()
			.bind_mut()
			.add(Gd::from_object(AttributeModifierEntry {
				modifier: Some(modifier),
				multiplier,
			}));
	}

	#[func]
	pub fn remove(&mut self, modifier: Gd<BaseAttributeModifier>) -> bool {
		for modifiers in self.modifiers_by_attribute.values_mut() {
			let found = modifiers.bind_mut().remove_modifier(&modifier);
			if found {
				return true;
			}
		}

		false
	}

	pub fn remove_attribute(&mut self, id: &Id) -> bool {
		self.modifiers_by_attribute.remove(id).is_some()
	}

	pub fn set_multiplier(&mut self, modifier: Gd<BaseAttributeModifier>, multiplier: f32) -> bool {
		for modifiers in self.modifiers_by_attribute.values_mut() {

			for mut entry in modifiers.bind_mut().iter() {
				let is_match = entry.bind().modifier.as_ref().unwrap() == &modifier;
				if is_match {
					entry.bind_mut().multiplier = multiplier;
					return true;
				}
			}
		}

		false
	}


	#[func]
	pub fn clear(&mut self) {
		self.modifiers_by_attribute.clear();
	}
}


impl IntoIterator for AttributeModifierCollection {
	type Item = (Id, Gd<AttributeModifierEntries>);
	type IntoIter = std::collections::hash_map::IntoIter<Id, Gd<AttributeModifierEntries>>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers_by_attribute.into_iter()
	}
}

impl<'a> IntoIterator for &'a AttributeModifierCollection {
	type Item = (&'a Id, &'a Gd<AttributeModifierEntries>);
	type IntoIter = std::collections::hash_map::Iter<'a, Id, Gd<AttributeModifierEntries>>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers_by_attribute.iter()
	}
}

impl<'a> IntoIterator for &'a mut AttributeModifierCollection {
	type Item = (&'a Id, &'a mut Gd<AttributeModifierEntries>);
	type IntoIter = std::collections::hash_map::IterMut<'a, Id, Gd<AttributeModifierEntries>>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers_by_attribute.iter_mut()
	}
}

impl FromIterator<(Id, Gd<AttributeModifierEntries>)> for AttributeModifierCollection {
	fn from_iter<T: IntoIterator<Item = (Id, Gd<AttributeModifierEntries>)>>(iter: T) -> Self {
		Self { modifiers_by_attribute: iter.into_iter().collect() }
	}
}

impl Extend<(Id, Gd<AttributeModifierEntries>)> for AttributeModifierCollection {
	fn extend<T: IntoIterator<Item = (Id, Gd<AttributeModifierEntries>)>>(&mut self, iter: T) {
		self.modifiers_by_attribute.extend(iter);
	}
}


impl GodotConvert for AttributeModifierCollection {
	type Via = Dictionary<Id, Gd<AttributeModifierEntries>>;

	fn godot_shape() -> GodotShape {
		Self::shape()
	}
}

impl FromGodot for AttributeModifierCollection {
	fn try_from_godot(via: Self::Via) -> Result<Self, godot::prelude::ConvertError> {
		Ok(
			Self::from_iter(&via)
		)
	}
}

impl ToGodot for AttributeModifierCollection {
	type Pass = ByValue;

	fn to_godot(&self) -> ToArg<'_, Self::Via, Self::Pass> {
		self.into_iter()
			.fold(Dictionary::new(), |mut dict, (key, value)| {
				let _ = dict.insert(key.clone(), value);
				dict
			})
	}
}

impl Var for AttributeModifierCollection {
	type PubType = Self::Via;

	fn var_get(field: &Self) -> Self::Via {
		Self::to_godot(field)
	}

	fn var_set(field: &mut Self, value: Self::Via) {
		*field = Self::from_godot(value);
	}

	fn var_pub_get(field: &Self) -> Self::PubType {
		Self::to_godot(field)
	}

	fn var_pub_set(field: &mut Self, value: Self::PubType) {
		*field = Self::from_godot(value);
	}
}

impl Export for AttributeModifierCollection {}

impl Element for AttributeModifierCollection {}