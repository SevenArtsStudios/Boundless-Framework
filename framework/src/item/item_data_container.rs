use std::{collections::HashMap, sync::Arc};

use crate::{Id, ItemData, ItemDataProvider};



pub trait ItemDataContainer: ItemDataProvider {
	fn register(&mut self, data: Arc<dyn ItemData>, overwrite: bool) -> Result<(), &'static str>;
	fn unregister(&mut self, data: &Arc<dyn ItemData>) -> bool;
	fn clear(&mut self) -> bool;
}

impl ItemDataContainer for HashMap<Id, Arc<dyn ItemData>> {
	fn register(&mut self, data: Arc<dyn ItemData>, overwrite: bool) -> Result<(), &'static str> {
		let key = data.item_key();
		let exists = self.contains_key(&key);

		if exists && !overwrite {
			return Err("Key already present");
		}

		self.insert(key.clone(), data);

		return Ok(());
	}

	fn unregister(&mut self, data: &Arc<dyn ItemData>) -> bool {
		let key = data.item_key();
		self.remove(&key).is_some()
	}

	fn clear(&mut self) -> bool {
		self.clear();
		true
	}
}
