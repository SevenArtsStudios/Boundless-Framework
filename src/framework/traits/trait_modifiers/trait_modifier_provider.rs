use std::collections::HashMap;

use godot::meta::{Element, ToArg};
use godot::meta::conv::ByValue;
use godot::meta::shape::{ClassHeritage, GodotElementShape, GodotShape};
use godot::prelude::*;

use crate::framework::{Id, GameTraitModifierOperation, BaseTraitModifier, TraitModifierEntries, TraitModifierEntry};


#[derive(GodotClass, Default, Clone)]
#[class(init, tool)]
pub struct TraitModifierCollection {
	pub modifiers_by_trait: HashMap<Id, Gd<TraitModifierEntries>>,
}

#[godot_api]
impl TraitModifierCollection {
	pub fn shape() -> GodotShape {
		GodotShape::TypedDictionary {
			key: Id::ELEMENT_SHAPE,
			value: GodotElementShape::Class {
				class_id: TraitModifierEntries::class_id(),
				heritage: ClassHeritage::Resource
			}
		}
	}

	pub fn apply_modifiers(&self, id: &Id, base_value: f32) -> Option<f32> {
		let Some(modifiers) = self.modifiers_by_trait.get(id) else {
			return None;
		};

		let mut sum = 0.0;
		let mut multiplier = 1.0;

		for entry in modifiers.bind().iter() {
			let entry_ref = entry.bind();
			if let Some(modifier) = &entry_ref.modifier {
				let modifier_ref = modifier.bind();
				match *modifier_ref {
					BaseTraitModifier { operation: GameTraitModifierOperation::Multiply, is_additive: true, ..} => {
						multiplier *= modifier_ref.apply_to(base_value, entry_ref.multiplier) / base_value
					},
					_ => sum += modifier_ref.apply_to(base_value, entry_ref.multiplier) - base_value,
				}
			}
		}

		Some((base_value + sum) * multiplier)
	}


	#[func]
	pub fn add(&mut self, id: Id, modifier: Gd<BaseTraitModifier>, multiplier: f32) {
		self.modifiers_by_trait
			.entry(id)
			.or_default()
			.bind_mut()
			.add(Gd::from_object(TraitModifierEntry {
				modifier: Some(modifier),
				multiplier,
			}));
	}

	#[func]
	pub fn remove(&mut self, modifier: Gd<BaseTraitModifier>) -> bool {
		for modifiers in self.modifiers_by_trait.values_mut() {
			let found = modifiers.bind_mut().remove_modifier(&modifier);
			if found {
				return true;
			}
		}

		false
	}

	pub fn remove_trait(&mut self, id: &Id) -> bool {
		self.modifiers_by_trait.remove(id).is_some()
	}

	pub fn set_multiplier(&mut self, modifier: Gd<BaseTraitModifier>, multiplier: f32) -> bool {
		for modifiers in self.modifiers_by_trait.values_mut() {

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
		self.modifiers_by_trait.clear();
	}
}


impl IntoIterator for TraitModifierCollection {
	type Item = (Id, Gd<TraitModifierEntries>);
	type IntoIter = std::collections::hash_map::IntoIter<Id, Gd<TraitModifierEntries>>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers_by_trait.into_iter()
	}
}

impl<'a> IntoIterator for &'a TraitModifierCollection {
	type Item = (&'a Id, &'a Gd<TraitModifierEntries>);
	type IntoIter = std::collections::hash_map::Iter<'a, Id, Gd<TraitModifierEntries>>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers_by_trait.iter()
	}
}

impl<'a> IntoIterator for &'a mut TraitModifierCollection {
	type Item = (&'a Id, &'a mut Gd<TraitModifierEntries>);
	type IntoIter = std::collections::hash_map::IterMut<'a, Id, Gd<TraitModifierEntries>>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers_by_trait.iter_mut()
	}
}

impl FromIterator<(Id, Gd<TraitModifierEntries>)> for TraitModifierCollection {
	fn from_iter<T: IntoIterator<Item = (Id, Gd<TraitModifierEntries>)>>(iter: T) -> Self {
		Self { modifiers_by_trait: iter.into_iter().collect() }
	}
}

impl Extend<(Id, Gd<TraitModifierEntries>)> for TraitModifierCollection {
	fn extend<T: IntoIterator<Item = (Id, Gd<TraitModifierEntries>)>>(&mut self, iter: T) {
		self.modifiers_by_trait.extend(iter);
	}
}


impl GodotConvert for TraitModifierCollection {
	type Via = Dictionary<Id, Gd<TraitModifierEntries>>;

	fn godot_shape() -> GodotShape {
		Self::shape()
	}
}

impl FromGodot for TraitModifierCollection {
	fn try_from_godot(via: Self::Via) -> Result<Self, godot::prelude::ConvertError> {
		Ok(
			Self::from_iter(&via)
		)
	}
}

impl ToGodot for TraitModifierCollection {
	type Pass = ByValue;

	fn to_godot(&self) -> ToArg<'_, Self::Via, Self::Pass> {
		self.into_iter()
			.fold(Dictionary::new(), |mut dict, (key, value)| {
				let _ = dict.insert(key.clone(), value);
				dict
			})
	}
}

impl Var for TraitModifierCollection {
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

impl Export for TraitModifierCollection {}

impl Element for TraitModifierCollection {}