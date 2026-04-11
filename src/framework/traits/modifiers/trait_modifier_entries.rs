use godot::{obj::Gd, prelude::GodotClass, builtin::Array};

use crate::framework::{GdTraitModifier, TraitModifierEntry};


#[derive(GodotClass, Clone)]
#[class(base = Resource, init, tool)]
pub struct TraitModifierEntries {
	#[export]
	modifiers: Array<Gd<TraitModifierEntry>>,
}
impl TraitModifierEntries {
	pub fn from_iter<T: IntoIterator<Item = Gd<TraitModifierEntry>>>(modifiers: T) -> Self {
		Self {
			modifiers: modifiers.into_iter().collect(),
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = Gd<TraitModifierEntry>> + '_ {
		self.modifiers.iter_shared()
	}

	pub fn add(&mut self, entry: Gd<TraitModifierEntry>) {
		self.modifiers.push(&entry);
	}

	pub fn remove(&mut self, entry: &Gd<TraitModifierEntry>) -> bool {
		if let Some(index) = self.modifiers.iter_shared().position(|e| e == *entry) {
			self.modifiers.remove(index);
			true
		} else {
			false
		}
	}
	pub fn remove_modifier(&mut self, modifier: &Gd<GdTraitModifier>) -> bool {
		if let Some(index) = self.modifiers.iter_shared().position(|e| e.bind().modifier.as_ref().unwrap() == modifier) {
			self.modifiers.remove(index);
			true
		} else {
			false
		}
	}

	pub fn len(&self) -> usize {
		self.modifiers.len()
	}

	pub fn clear(&mut self) {
		self.modifiers.clear();
	}
}