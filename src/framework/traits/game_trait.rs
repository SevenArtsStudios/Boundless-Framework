use godot::{builtin::VariantType, meta::{Element, FromGodot, GodotConvert, ToArg, ToGodot, conv::ByValue, shape::{GodotElementShape, GodotShape}}, prelude::{Export, GString, GodotClass, Var}, register::info::ParamMetadata};
use string_cache::DefaultAtom;

pub(crate) fn normalize_trait_name(name: &str) -> String {
	name.to_lowercase().replace(' ', "_")
}

#[derive(GodotClass, Clone, Default, PartialEq, Eq, Hash, Debug)]
#[class(init, tool)]
pub struct GameTrait {
	id: DefaultAtom,
}

impl GameTrait {
	pub const ELEMENT_SHAPE: GodotElementShape = GodotElementShape::Builtin {
		variant_type: VariantType::STRING_NAME
	};
	pub const SHAPE: GodotShape = GodotShape::Builtin {
		variant_type: VariantType::STRING_NAME,
		metadata: ParamMetadata::NONE
	};

	pub fn from_unnormalized(name: impl AsRef<str>) -> Self {
		let normalized = normalize_trait_name(name.as_ref());
		Self {
			id: DefaultAtom::from(normalized),
		}
	}

	pub fn from_normalized(normalized_name: &str) -> Self {
		assert_eq!(normalized_name, normalize_trait_name(normalized_name), "Trait name must be normalized (lowercase, no spaces)");
		Self {
			id: DefaultAtom::from(normalized_name),
		}
	}

	pub fn id(&self) -> &str {
		&self.id
	}
}

impl GodotConvert for GameTrait {
	type Via = GString;

	fn godot_shape() -> GodotShape {
		Self::SHAPE
	}
}

impl FromGodot for GameTrait {
	fn try_from_godot(via: Self::Via) -> Result<Self, godot::prelude::ConvertError> {
		Ok(via.into())
	}
}

impl ToGodot for GameTrait {
	type Pass = ByValue;

	fn to_godot(&self) -> ToArg<'_, Self::Via, Self::Pass> {
		self.id().into()
	}
}

impl Var for GameTrait {
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

impl Export for GameTrait {}

impl Element for GameTrait {}


impl From<&str> for GameTrait {
	fn from(value: &str) -> Self {
		GameTrait::from_unnormalized(value)
	}
}

impl From<String> for GameTrait {
	fn from(value: String) -> Self {
		GameTrait::from_unnormalized(value)
	}
}

impl From<GString> for GameTrait {
	fn from(value: GString) -> Self {
		GameTrait::from_unnormalized(String::from(value))
	}
}

impl Into<String> for GameTrait {
	fn into(self) -> String {
		self.id().to_string()
	}
}