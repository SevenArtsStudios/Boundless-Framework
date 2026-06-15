use std::{collections::HashMap};
use std::sync::Arc;

use crate::{Id, ItemData};

pub trait ItemDataProvider: Send + Sync {
	fn get_item(&self, key: &Id) -> Option<Arc<dyn ItemData>>;
}


impl ItemDataProvider for HashMap<Id, Arc<dyn ItemData>> {
	fn get_item(&self, key: &Id) -> Option<Arc<dyn ItemData>> {
		self.get(key).cloned()
	}
}

impl ItemDataProvider for Vec<Arc<dyn ItemDataProvider>> {
	fn get_item(&self, key: &Id) -> Option<Arc<dyn ItemData>> {
		for registry in self {
			let found = registry.get_item(key);
			if found.is_some() {
				return found;
			}
		}

		None
	}
}