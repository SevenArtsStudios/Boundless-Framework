use godot::{obj::Gd, prelude::GodotClass, builtin::Array};

use crate::framework::{GdTraitModifier, TraitModifierEntry};


#[derive(GodotClass, Clone)]
#[class(base = Resource, init, tool)]
pub struct TraitModifierEntries {
	#[export]
	modifiers: Array<Gd<TraitModifierEntry>>,
}
impl TraitModifierEntries {
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


	pub fn iter(&self) -> impl Iterator<Item = Gd<TraitModifierEntry>> + '_ {
		self.modifiers.iter_shared()
	}
}

impl IntoIterator for TraitModifierEntries {
	type Item = Gd<TraitModifierEntry>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers.iter_shared().collect::<Vec<_>>().into_iter()
	}
}

impl <'a> IntoIterator for &'a TraitModifierEntries {
	type Item = Gd<TraitModifierEntry>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers.iter_shared().collect::<Vec<_>>().into_iter()
	}
}

impl <'a> IntoIterator for &'a mut TraitModifierEntries {
	type Item = Gd<TraitModifierEntry>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers.iter_shared().collect::<Vec<_>>().into_iter()
	}
}

impl FromIterator<Gd<TraitModifierEntry>> for TraitModifierEntries {
	fn from_iter<T: IntoIterator<Item = Gd<TraitModifierEntry>>>(iter: T) -> Self {
		let modifiers = iter.into_iter().collect();
		Self { modifiers }
	}
}

impl Extend<Gd<TraitModifierEntry>> for TraitModifierEntries {
	fn extend<T: IntoIterator<Item = Gd<TraitModifierEntry>>>(&mut self, iter: T) {
		self.modifiers.extend(iter);
	}
}