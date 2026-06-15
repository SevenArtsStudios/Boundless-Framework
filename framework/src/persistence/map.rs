use std::collections::HashMap;

use crate::{Id, ItemDataProvider, PersistentState, PersistentValue};

/// A map-like persistence state keyed by item keys.
pub struct PersistentMap {
	state: HashMap<Id, Box<dyn PersistentState>>,
	on_deserialize_missing_key: Option<Box<dyn Fn(&Id, &dyn PersistentValue, Option<&dyn ItemDataProvider>) + Send + Sync>>,
}

impl PersistentMap {
	pub fn new() -> Self {
		Self {
			state: HashMap::new(),
			on_deserialize_missing_key: None,
		}
	}

	pub fn with_state(state: HashMap<Id, Box<dyn PersistentState>>) -> Self {
		Self {
			state,
			on_deserialize_missing_key: None,
		}
	}

	pub fn on_deserialize_missing_key(
		&mut self,
		callback: Box<dyn Fn(&Id, &dyn PersistentValue, Option<&dyn ItemDataProvider>) + Send + Sync>,
	) {
		self.on_deserialize_missing_key = Some(callback);
	}

	pub fn get_state(&self, key: &Id) -> Option<&dyn PersistentState> {
		self.state.get(key).map(|state| state.as_ref())
	}

	pub fn get_state_mut(&mut self, key: &Id) -> Option<&mut (dyn PersistentState + '_)> {
		match self.state.get_mut(key) {
			Some(state) => Some(&mut **state),
			None => None,
		}
	}

	pub fn insert(&mut self, key: Id, value: Box<dyn PersistentState>) {
		self.state.insert(key, value);
	}

	pub fn contains_key(&self, key: &Id) -> bool {
		self.state.contains_key(key)
	}

	pub fn len(&self) -> usize {
		self.state.len()
	}
}

impl PersistentState for PersistentMap {
	fn update(&mut self, value: &dyn PersistentValue, registry: Option<&dyn ItemDataProvider>) {
		if let Some(map_value) = value.as_any().downcast_ref::<PersistentMapValue>() {
			for (key, item_value) in &map_value.data {
				if let Some(state) = self.get_state_mut(key) {
					state.update(item_value.as_ref(), registry);
				} else if let Some(callback) = &self.on_deserialize_missing_key {
					callback(key, item_value.as_ref(), registry);
				}
			}
		}
	}

	fn save(&self) -> Box<dyn PersistentValue> {
		let data = self
			.state
			.iter()
			.map(|(key, state)| (key.clone(), state.save()))
			.collect();

		Box::new(PersistentMapValue { data })
	}
}

/// Represents a persistent map value.
pub struct PersistentMapValue {
	pub data: HashMap<Id, Box<dyn PersistentValue>>,
}

/// A dictionary alias for persistence maps.
pub type PersistentDictionary = PersistentMap;
