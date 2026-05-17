use std::{hash::Hash, ops::{Deref, DerefMut}};

use godot::{
	classes::{Node, Object, Resource}, meta::{GodotConvert, shape::{ClassHeritage, GodotShape}}, obj::{Bounds, DynGd, DynGdMut, DynGdRef, Gd, GodotClass, InstanceId, bounds::DeclEngine}, register::property::{Export, Var}
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


#[derive(Clone, Debug)]
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

impl <T: ?Sized + 'static, G: GodotClass> PartialEq for ObjectHandle<T, G> {
	fn eq(&self, other: &Self) -> bool {
		self.key() == other.key()
	}
}

impl <T: ?Sized + 'static, G: GodotClass> Eq for ObjectHandle<T, G> { }

impl<T: ?Sized + 'static, G: GodotClass> Hash for ObjectHandle<T, G> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.key().hash(state);
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

#[derive(Clone, Debug, Default)]
pub enum GdObjectHandle<T: ?Sized + 'static, G: GodotClass> {
	Some(ObjectHandle<T, G>),
	#[default]
	None,
}

impl<T: ?Sized + 'static, G: GodotClass> GodotConvert for GdObjectHandle<T, G> {
	type Via = Option<Gd<G>>;

	fn godot_shape() -> GodotShape {
		// Note: `get_dyn_implementor_class_ids` is not accessible, so we settle for Resource
		let heritage = if G::inherits::<Resource>() {
			ClassHeritage::Resource
		} else if G::inherits::<Node>() {
			ClassHeritage::Node
		} else {
			ClassHeritage::Other
		};

		let class_id = G::class_id();
		GodotShape::Class {
			class_id,
			heritage,
			is_nullable: false,
		}
	}
}

impl<T: ?Sized + 'static, G: GodotClass + Bounds<Declarer = DeclEngine>> Var for GdObjectHandle<T, G> {
	type PubType = Self;

	fn var_get(field: &Self) -> Self::Via {
		match field {
			Self::Some(ObjectHandle::Owned(_)) => None,
			Self::Some(ObjectHandle::Godot(object)) => Some(object.clone().into_gd()),
			Self::None => None,
		}
	}

	fn var_set(field: &mut Self, value: Self::Via) {
		match value {
			Some(gd_object) => {
				let dyn_gd = gd_object.upcast().try_dynify::<T>();
				match dyn_gd {
					Ok(dyn_gd) => *field = Self::Some(ObjectHandle::Godot(dyn_gd.clone())),
					Err(_) => *field = Self::None
				}
			},
			None => *field = Self::None,
		}
	}

	fn var_pub_get(field: &Self) -> Self::PubType {
		match field {
			Self::Some(ObjectHandle::Godot(object)) => Self::Some(ObjectHandle::Godot(object.clone())),
			_ => Self::None,
		}
	}

	fn var_pub_set(field: &mut Self, value: Self::PubType) {
		*field = value;
	}
}

impl<T: ?Sized + 'static, G: GodotClass + Bounds<Declarer = DeclEngine>> Export for GdObjectHandle<T, G> { }