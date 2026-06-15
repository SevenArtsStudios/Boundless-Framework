use crate::{ItemDataProvider, persistence::{PersistentState, PersistentValue}};

/// A persistent toggle state.
pub struct PersistentToggle {
	value: bool,
	on_update: Option<Box<dyn Fn(bool) + Send + Sync>>,
}

impl PersistentToggle {
	pub fn new(default_value: bool) -> Self {
		Self {
			value: default_value,
			on_update: None,
		}
	}

	pub fn on_update(&mut self, callback: Box<dyn Fn(bool) + Send + Sync>) {
		self.on_update = Some(callback);
	}

	pub fn value(&self) -> bool {
		self.value
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PersistentToggleValue(pub bool);


impl PersistentState for PersistentToggle {
	fn update(&mut self, value: &dyn PersistentValue, _registry: Option<&dyn ItemDataProvider>) {
		if let Some(toggle) = value.as_any().downcast_ref::<PersistentToggleValue>() {
			self.value = toggle.0;
			if let Some(callback) = &self.on_update {
				callback(self.value);
			}
		}
	}

	fn save(&self) -> Box<dyn PersistentValue> {
		Box::new(PersistentToggleValue(self.value))
	}
}
