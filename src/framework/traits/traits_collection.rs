use std::collections::HashMap;

use godot::{meta::{Element, ToArg, conv::ByValue, shape::{GodotElementShape, GodotShape}}, prelude::*};

use crate::framework::{TraitsProvider, traits::game_trait::GameTrait};


#[derive(GodotClass, Default, Clone, PartialEq, Debug)]
#[class(init, tool)]
pub struct TraitsCollection {
	trait_values: HashMap<GameTrait, f32>,
}

#[godot_api]
impl TraitsCollection {
	pub const SHAPE: GodotShape = GodotShape::TypedDictionary {
		key: GameTrait::ELEMENT_SHAPE,
		value: GodotElementShape::Builtin {
			variant_type: VariantType::FLOAT
		},
	};

	pub fn set(&mut self, r#trait: GameTrait, value: f32) -> Option<f32> {
		self.trait_values.insert(r#trait, value)
	}
	#[func(rename=set)]
	pub fn gd_set(&mut self, r#trait: GameTrait, value: f32) -> f32 {
		self.set(r#trait, value)
			.unwrap_or_default()
	}

	pub fn get(&self, r#trait: &GameTrait) -> Option<&f32> {
		self.trait_values
			.get(&r#trait)
	}
	#[func(rename=get)]
	pub fn gd_get(&self, r#trait: GameTrait, default_value: f32) -> f32 {
		self.get(&r#trait)
			.copied()
			.unwrap_or(default_value)
	}

	#[func]
	pub fn remove(&mut self, r#trait: GameTrait) -> bool {
		self.trait_values
			.remove(&r#trait)
			.is_some()
	}

	#[func]
	pub fn clear(&mut self) {
		self.trait_values.clear();
	}

	#[func]
	pub fn contains_trait(&self, r#trait: GameTrait) -> bool {
		self.trait_values
			.contains_key(&r#trait)
	}
}


impl IntoIterator for TraitsCollection {
	type Item = (GameTrait, f32);
	type IntoIter = std::collections::hash_map::IntoIter<GameTrait, f32>;

	fn into_iter(self) -> Self::IntoIter {
		self.trait_values.into_iter()
	}
}

impl<'a> IntoIterator for &'a TraitsCollection {
	type Item = (&'a GameTrait, &'a f32);
	type IntoIter = std::collections::hash_map::Iter<'a, GameTrait, f32>;

	fn into_iter(self) -> Self::IntoIter {
		self.trait_values.iter()
	}
}

impl<'a> IntoIterator for &'a mut TraitsCollection {
	type Item = (&'a GameTrait, &'a mut f32);
	type IntoIter = std::collections::hash_map::IterMut<'a, GameTrait, f32>;

	fn into_iter(self) -> Self::IntoIter {
		self.trait_values.iter_mut()
	}
}

impl FromIterator<(GameTrait, f32)> for TraitsCollection {
	fn from_iter<T: IntoIterator<Item = (GameTrait, f32)>>(iter: T) -> Self {
		let trait_values = iter.into_iter().collect();
		Self { trait_values }
	}
}

impl Extend<(GameTrait, f32)> for TraitsCollection {
	fn extend<T: IntoIterator<Item = (GameTrait, f32)>>(&mut self, iter: T) {
		self.trait_values.extend(iter);
	}
}

impl TraitsProvider for TraitsCollection {
	fn get_value(&self, r#trait: &GameTrait) -> Option<f32> {
		self.get(r#trait)
			.copied()
	}
}

impl GodotConvert for TraitsCollection {
	type Via = Dictionary<GameTrait, f32>;

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
			.fold(Dictionary::new(), |mut dict, (key, value)| {
				let _ = dict.insert(key.clone(), *value);
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