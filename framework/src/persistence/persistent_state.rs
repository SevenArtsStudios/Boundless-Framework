use crate::{ItemDataProvider, PersistentValue};


/// A persistent state that can save and restore values.
pub trait PersistentState: Send + Sync {
	/// Applies the given persistent value to this state.
	fn update(&mut self, value: &dyn PersistentValue, registry: Option<&dyn ItemDataProvider>);

	/// Saves the current state as a persistent value.
	fn save(&self) -> Box<dyn PersistentValue>;
}
