use crate::{ItemDataProvider, persistence::{PersistentState, PersistentValue}};

/// A persistence state representing a null value.
pub struct PersistentNull;

impl PersistentNull {
	pub fn new() -> Self {
		Self
	}
}

impl PersistentState for PersistentNull {
	fn update(&mut self, _value: &dyn PersistentValue, _registry: Option<&dyn ItemDataProvider>) {}

	fn save(&self) -> Box<dyn PersistentValue> {
		Box::new(PersistentNull)
	}
}

