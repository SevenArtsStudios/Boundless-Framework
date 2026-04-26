use std::ops::{Deref, DerefMut};

use godot::{
	classes::Object,
	obj::{DynGd, DynGdMut, DynGdRef, GodotClass, InstanceId},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectKey {
	Owned(usize),
	Godot(InstanceId),
}

pub enum ObjectDynBoundRef<'a, T: ?Sized + 'static> {
	Owned(&'a T),
	Godot(DynGdRef<'a, T>),
}

impl<T: ?Sized + 'static> Deref for ObjectDynBoundRef<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Owned(value) => value,
			Self::Godot(value) => value,
		}
	}
}

pub enum ObjectDynBoundMut<'a, T: ?Sized + 'static> {
	Owned(&'a mut T),
	Godot(DynGdMut<'a, T>),
}

impl<T: ?Sized + 'static> Deref for ObjectDynBoundMut<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Owned(value) => value,
			Self::Godot(value) => value,
		}
	}
}

impl<T: ?Sized + 'static> DerefMut for ObjectDynBoundMut<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::Owned(value) => value,
			Self::Godot(value) => value,
		}
	}
}


pub enum ObjectHandle<T: ?Sized + 'static, G: GodotClass = Object> {
	Owned(Box<T>),
	Godot(DynGd<G, T>),
}

impl<T: ?Sized + 'static, G: GodotClass> ObjectHandle<T, G> {
	pub fn key(&self) -> ObjectKey {
		match self {
			Self::Owned(modifier) => {
				ObjectKey::Owned(modifier.as_ref() as *const T as *const () as usize)
			}
			Self::Godot(object) => ObjectKey::Godot(object.instance_id()),
		}
	}

	pub fn bind(&self) -> ObjectDynBoundRef<'_, T> {
		match self {
			Self::Owned(value) => ObjectDynBoundRef::Owned(value),
			Self::Godot(value) => ObjectDynBoundRef::Godot(value.dyn_bind()),
		}
	}

	pub fn bind_mut(&mut self) -> ObjectDynBoundMut<'_, T> {
		match self {
			Self::Owned(value) => ObjectDynBoundMut::Owned(value),
			Self::Godot(value) => ObjectDynBoundMut::Godot(value.dyn_bind_mut()),
		}
	}
}

impl<T: ?Sized + 'static, G: GodotClass> From<Box<T>> for ObjectHandle<T, G> {
	fn from(value: Box<T>) -> Self {
		Self::Owned(value)
	}
}

impl<T: ?Sized + 'static, G: GodotClass> From<DynGd<G, T>> for ObjectHandle<T, G> {
	fn from(value: DynGd<G, T>) -> Self {
		Self::Godot(value)
	}
}