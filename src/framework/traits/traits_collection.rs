use std::collections::HashMap;

use godot::{meta::{Element, ToArg, conv::ByValue, shape::{GodotElementShape, GodotShape}}, prelude::*};

use crate::framework::Id;
use crate::framework::TraitsProvider;


#[derive(GodotClass, Default, Clone, PartialEq, Debug)]
#[class(init, tool)]
pub struct TraitsCollection {
	trait_values: HashMap<Id, f32>,
}

#[godot_api]
impl TraitsCollection {
	pub const SHAPE: GodotShape = GodotShape::TypedDictionary {
		key: Id::ELEMENT_SHAPE,
		value: GodotElementShape::Builtin {
			variant_type: VariantType::FLOAT
		},
	};

	pub fn set(&mut self, id: Id, value: f32) -> Option<f32> {
		self.trait_values.insert(id, value)
	}
	#[func(rename=set)]
	pub fn gd_set(&mut self, id: Id, value: f32) -> f32 {
		self.set(id, value)
			.unwrap_or_default()
	}

	pub fn get(&self, id: &Id) -> Option<&f32> {
		self.trait_values
			.get(&id)
	}
	#[func(rename=get)]
	pub fn gd_get(&self, id: Id, default_value: f32) -> f32 {
		self.get(&id)
			.copied()
			.unwrap_or(default_value)
	}

	#[func]
	pub fn remove(&mut self, id: Id) -> bool {
		self.trait_values
			.remove(&id)
			.is_some()
	}

	#[func]
	pub fn clear(&mut self) {
		self.trait_values.clear();
	}

	#[func]
	pub fn contains_trait(&self, id: Id) -> bool {
		self.trait_values
			.contains_key(&id)
	}
}


impl IntoIterator for TraitsCollection {
	type Item = (Id, f32);
	type IntoIter = std::collections::hash_map::IntoIter<Id, f32>;

	fn into_iter(self) -> Self::IntoIter {
		self.trait_values.into_iter()
	}
}

impl<'a> IntoIterator for &'a TraitsCollection {
	type Item = (&'a Id, &'a f32);
	type IntoIter = std::collections::hash_map::Iter<'a, Id, f32>;

	fn into_iter(self) -> Self::IntoIter {
		self.trait_values.iter()
	}
}

impl<'a> IntoIterator for &'a mut TraitsCollection {
	type Item = (&'a Id, &'a mut f32);
	type IntoIter = std::collections::hash_map::IterMut<'a, Id, f32>;

	fn into_iter(self) -> Self::IntoIter {
		self.trait_values.iter_mut()
	}
}

impl FromIterator<(Id, f32)> for TraitsCollection {
	fn from_iter<T: IntoIterator<Item = (Id, f32)>>(iter: T) -> Self {
		let trait_values = iter.into_iter().collect();
		Self { trait_values }
	}
}

impl Extend<(Id, f32)> for TraitsCollection {
	fn extend<T: IntoIterator<Item = (Id, f32)>>(&mut self, iter: T) {
		self.trait_values.extend(iter);
	}
}

impl TraitsProvider for TraitsCollection {
	fn get_value(&self, id: &Id) -> Option<f32> {
		self.get(id)
			.copied()
	}
}

impl GodotConvert for TraitsCollection {
	type Via = Dictionary<Id, f32>;

	fn godot_shape() -> GodotShape {
		Self::SHAPE
	}
}

impl FromGodot for TraitsCollection {
	fn try_from_godot(via: Self::Via) -> Result<Self, godot::prelude::ConvertError> {
		Ok(
			Self::from_iter(&via)
		)
	}
}

impl ToGodot for TraitsCollection {
	type Pass = ByValue;

	fn to_godot(&self) -> ToArg<'_, Self::Via, Self::Pass> {
		self.into_iter()
			.fold(Dictionary::new(), |mut dict, (id, value)| {
				let _ = dict.insert(id.clone(), *value);
				dict
			})
	}
}

impl Var for TraitsCollection {
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

impl Export for TraitsCollection {}

impl Element for TraitsCollection {}