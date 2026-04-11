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
		let mut trait_values = HashMap::new();

		for (key, value) in via.into_iter() {
			trait_values.insert(key, value);
		}

		Ok(
			Self { trait_values }
		)
	}
}

impl ToGodot for TraitsCollection {
	type Pass = ByValue;

	fn to_godot(&self) -> ToArg<'_, Self::Via, Self::Pass> {
		let mut dict = Dictionary::new();

		for (key, value) in &self.trait_values {
			let _ = dict.insert(key.clone(), *value);
		}

		dict
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