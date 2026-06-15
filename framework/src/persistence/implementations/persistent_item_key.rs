use crate::{Id, ItemDataProvider, persistence::{PersistentState, PersistentValue}};

/// A persistence state for an item key.
pub struct PersistentItemKey {
	key: Id,
	on_changed: Option<Box<dyn Fn(Id) + Send + Sync>>,
}

impl PersistentItemKey {
	pub fn new(key: Id) -> Self {
		Self {
			key,
			on_changed: None,
		}
	}

	pub fn on_changed(&mut self, callback: Box<dyn Fn(Id) + Send + Sync>) {
		self.on_changed = Some(callback);
	}

	pub fn key(&self) -> &Id {
		&self.key
	}
}

impl PersistentState for PersistentItemKey {
	fn update(&mut self, value: &dyn PersistentValue, _registry: Option<&dyn ItemDataProvider>) {
		if let Some(key) = value.as_any().downcast_ref::<Id>() {
			self.key = key.clone();
			if let Some(callback) = &self.on_changed {
				callback(self.key.clone());
			}
		}
	}

	fn save(&self) -> Box<dyn PersistentValue> {
		Box::new(self.key.clone())
	}
}
