use crate::{Id, InstantiablePersistentValue, Item, ItemDataProvider, Persistent, PersistentState, PersistentValue};


/// A persistence wrapper around an item and its persistent state.
pub struct PersistentItem {
	pub item: Box<dyn Item + Send + Sync>,
	pub persistence: Box<dyn PersistentState>,
}

impl PersistentItem {
	pub fn new(item: Box<dyn Item + Send + Sync>, persistence: Box<dyn PersistentState>) -> Self {
		Self { item, persistence }
	}
}

impl PersistentState for PersistentItem {
	fn update(&mut self, value: &dyn PersistentValue, registry: Option<&dyn ItemDataProvider>) {
		self.persistence.update(value, registry);
	}

	fn save(&self) -> Box<dyn PersistentValue> {
		Box::new(PersistentItemValue {
			item_key: self.item.item_key(),
			persistence_value: self.persistence.save(),
		})
	}
}

/// The persisted representation of an item and its nested state.
pub struct PersistentItemValue {
	pub item_key: Id,
	pub persistence_value: Box<dyn PersistentValue>,
}

impl InstantiablePersistentValue for PersistentItemValue {
	fn instantiate(&self, registry: &dyn ItemDataProvider) -> Option<Box<dyn Persistent>> {
		let mut item = registry.get_item(&self.item_key)?.instantiate().ok()?;
		item.persistence_mut().update(self.persistence_value.as_ref(), Some(registry));
		Some(item)
	}
}
