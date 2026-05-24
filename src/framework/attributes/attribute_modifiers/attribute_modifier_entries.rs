use godot::{obj::Gd, prelude::GodotClass, builtin::Array};

use crate::framework::{BaseAttributeModifier, AttributeModifierEntry};


#[derive(GodotClass, Clone)]
#[class(base = Resource, init, tool)]
pub struct AttributeModifierEntries {
	#[export]
	modifiers: Array<Gd<AttributeModifierEntry>>,
}
impl AttributeModifierEntries {
	pub fn add(&mut self, entry: Gd<AttributeModifierEntry>) {
		self.modifiers.push(&entry);
	}

	pub fn remove(&mut self, entry: &Gd<AttributeModifierEntry>) -> bool {
		if let Some(index) = self.modifiers.iter_shared().position(|e| e == *entry) {
			self.modifiers.remove(index);
			true
		} else {
			false
		}
	}
	pub fn remove_modifier(&mut self, modifier: &Gd<BaseAttributeModifier>) -> bool {
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


	pub fn iter(&self) -> impl Iterator<Item = Gd<AttributeModifierEntry>> + '_ {
		self.modifiers.iter_shared()
	}
}

impl IntoIterator for AttributeModifierEntries {
	type Item = Gd<AttributeModifierEntry>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers.iter_shared().collect::<Vec<_>>().into_iter()
	}
}

impl <'a> IntoIterator for &'a AttributeModifierEntries {
	type Item = Gd<AttributeModifierEntry>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers.iter_shared().collect::<Vec<_>>().into_iter()
	}
}

impl <'a> IntoIterator for &'a mut AttributeModifierEntries {
	type Item = Gd<AttributeModifierEntry>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.modifiers.iter_shared().collect::<Vec<_>>().into_iter()
	}
}

impl FromIterator<Gd<AttributeModifierEntry>> for AttributeModifierEntries {
	fn from_iter<T: IntoIterator<Item = Gd<AttributeModifierEntry>>>(iter: T) -> Self {
		let modifiers = iter.into_iter().collect();
		Self { modifiers }
	}
}

impl Extend<Gd<AttributeModifierEntry>> for AttributeModifierEntries {
	fn extend<T: IntoIterator<Item = Gd<AttributeModifierEntry>>>(&mut self, iter: T) {
		self.modifiers.extend(iter);
	}
}