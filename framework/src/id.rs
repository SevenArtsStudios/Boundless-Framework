use string_cache::DefaultAtom;

pub(crate) fn normalize_id_name(name: &str) -> String {
	name.to_lowercase().replace(' ', "_")
}

#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct Id {
	id: DefaultAtom,
}

impl Id {
	pub fn from_unnormalized(name: impl AsRef<str>) -> Self {
		let normalized = normalize_id_name(name.as_ref());
		Self {
			id: DefaultAtom::from(normalized),
		}
	}

	pub fn from_normalized(normalized_name: &str) -> Self {
		assert_eq!(normalized_name, normalize_id_name(normalized_name), "Id name must be normalized (lowercase, no spaces)");
		Self {
			id: DefaultAtom::from(normalized_name),
		}
	}

	pub fn id(&self) -> &str {
		&self.id
	}
}


impl From<&str> for Id {
	fn from(value: &str) -> Self {
		Id::from_unnormalized(value)
	}
}

impl From<String> for Id {
	fn from(value: String) -> Self {
		Id::from_unnormalized(value)
	}
}