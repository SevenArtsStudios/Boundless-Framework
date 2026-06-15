use std::any::Any;

use crate::{Id, ItemDataProvider, PersistentState};

/// A marker trait for values that can be serialized by the persistence system.
pub trait PersistentValue: Any + Send + Sync {
	fn as_any(&self) -> &dyn Any;
}

impl<T: Any + Send + Sync> PersistentValue for T {
	fn as_any(&self) -> &dyn Any {
		self
	}
}

/// A persistent value that can recreate a persisted object using an item provider.
pub trait InstantiablePersistentValue: PersistentValue {
	fn instantiate(&self, registry: &dyn ItemDataProvider) -> Option<Box<dyn Persistent>>;
}

/// An object that can expose persistence state.
pub trait Persistent: Send + Sync {
	/// The key that identifies this persistent object.
	fn identifier(&self) -> Id;

	/// The root persistence state for this object.
	fn persistence(&self) -> &dyn PersistentState;

	/// The root persistence state for this object, mutable.
	fn persistence_mut(&mut self) -> &mut dyn PersistentState;
}
