use crate::{ItemDataProvider, PersistentState, PersistentValue};



/// A list-like persistence state.
pub struct PersistentCollection {
	state: Vec<Box<dyn PersistentState>>,
	on_deserialize_missing_index:
		Option<Box<dyn Fn(usize, &dyn PersistentValue, Option<&dyn ItemDataProvider>) + Send + Sync>>,
}

impl PersistentCollection {
	pub fn new() -> Self {
		Self {
			state: Vec::new(),
			on_deserialize_missing_index: None,
		}
	}

	pub fn with_state(state: Vec<Box<dyn PersistentState>>) -> Self {
		Self {
			state,
			on_deserialize_missing_index: None,
		}
	}

	pub fn on_deserialize_missing_index(
		&mut self,
		callback: Box<dyn Fn(usize, &dyn PersistentValue, Option<&dyn ItemDataProvider>) + Send + Sync>,
	) {
		self.on_deserialize_missing_index = Some(callback);
	}

	pub fn get_state(&self, index: usize) -> Option<&dyn PersistentState> {
		self.state.get(index).map(|state| state.as_ref())
	}

	pub fn get_state_mut(&mut self, index: usize) -> Option<&mut (dyn PersistentState + '_)> {
		match self.state.get_mut(index) {
			Some(state) => Some(&mut **state),
			None => None,
		}
	}

	pub fn push(&mut self, state: Box<dyn PersistentState>) {
		self.state.push(state);
	}

	pub fn insert(&mut self, index: usize, state: Box<dyn PersistentState>) {
		self.state.insert(index, state);
	}

	pub fn remove(&mut self, index: usize) -> Option<Box<dyn PersistentState>> {
		if index < self.state.len() {
			Some(self.state.remove(index))
		} else {
			None
		}
	}

	pub fn len(&self) -> usize {
		self.state.len()
	}
}

impl PersistentState for PersistentCollection {
	fn update(&mut self, value: &dyn PersistentValue, registry: Option<&dyn ItemDataProvider>) {
		if let Some(collection_value) = value.as_any().downcast_ref::<PersistentCollectionValue>() {
			let count = usize::min(self.state.len(), collection_value.values.len());
			for index in 0..count {
				if let Some(state) = self.get_state_mut(index) {
					state.update(collection_value.values[index].as_ref(), registry);
				}
			}
			for index in count..collection_value.values.len() {
				if let Some(callback) = &self.on_deserialize_missing_index {
					callback(index, collection_value.values[index].as_ref(), registry);
				}
			}
		}
	}

	fn save(&self) -> Box<dyn PersistentValue> {
		let values = self.state.iter().map(|state| state.save()).collect();
		Box::new(PersistentCollectionValue { values })
	}
}

/// Represents a persistent collection value.
pub struct PersistentCollectionValue {
	pub values: Vec<Box<dyn PersistentValue>>,
}

/// A list alias for persistence collections.
pub type PersistentList = PersistentCollection;
