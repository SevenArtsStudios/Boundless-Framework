use std::fmt::Display;

use boundless::id::Id;
use godot::{meta::{Element, ToArg, conv::ByValue, shape::{GodotElementShape, GodotShape}}, prelude::*, register::info::ParamMetadata};


#[derive(GodotClass, Clone, Default, PartialEq, Eq, Hash, Debug)]
#[class(init, tool, rename=Id)]
pub struct GodotId {
	value: Id
}

impl GodotId {
	pub const ELEMENT_SHAPE: GodotElementShape = GodotElementShape::Builtin {
		variant_type: VariantType::STRING_NAME
	};
	pub const SHAPE: GodotShape = GodotShape::Builtin {
		variant_type: VariantType::STRING_NAME,
		metadata: ParamMetadata::NONE
	};

	pub fn from_unnormalized(name: impl AsRef<str>) -> Self {
		Self {
			value: Id::from_unnormalized(name)
		}
	}

	pub fn from_normalized(normalized_name: &str) -> Self {
		Self {
			value: Id::from_normalized(normalized_name)
		}
	}

	pub fn as_id(&self) -> &Id {
		&self.value
	}

	pub fn id(&self) -> &str {
		&self.value.id()
	}
}

impl From<Id> for GodotId {
	fn from(value: Id) -> Self {
		Self { value }
	}
}

impl Into<Id> for GodotId {
	fn into(self) -> Id {
		self.value
	}
}

impl Display for GodotId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.as_id().fmt(f)
	}
}

impl GodotConvert for GodotId {
	type Via = StringName;

	fn godot_shape() -> GodotShape {
		Self::SHAPE
	}
}

impl FromGodot for GodotId {
	fn try_from_godot(via: Self::Via) -> Result<Self, godot::prelude::ConvertError> {
		Ok(via.into())
	}
}

impl ToGodot for GodotId {
	type Pass = ByValue;

	fn to_godot(&self) -> ToArg<'_, Self::Via, Self::Pass> {
		self.id().into()
	}
}

impl Var for GodotId {
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

impl Export for GodotId {}

impl Element for GodotId {}

impl From<GString> for GodotId {
	fn from(value: GString) -> Self {
		GodotId::from_unnormalized(String::from(value))
	}
}

impl From<StringName> for GodotId {
	fn from(value: StringName) -> Self {
		GodotId::from_unnormalized(String::from(value))
	}
}

impl Into<String> for GodotId {
	fn into(self) -> String {
		self.id().to_string()
	}
}