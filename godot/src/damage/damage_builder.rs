use std::{sync::{Arc, Mutex}};

use godot::{builtin::Array, classes::Resource, obj::{DynGd, Gd}, prelude::GodotClass};

use boundless::damage::{Damage, DamageInstance, DamageModifier, flatten_damages, unique_modifiers};

#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
pub struct DamageBuilder {
	#[export]
	#[init(val=1.0)]
	pub amount: f32,
	#[export]
	pub modifiers: Array<Option<DynGd<Resource, dyn DamageModifier>>>,
}

struct ModifierWrapper {
	modifier: DynGd<Resource, dyn DamageModifier>,
}

impl DamageModifier for ModifierWrapper {
	fn modify(
		&mut self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		self.modifier.dyn_bind_mut().modify(damage)
	}

	fn apply(
		&mut self,
		damage: Arc<Mutex<DamageInstance>>
	) {
		self.modifier.dyn_bind_mut().apply(damage);
	}
}

impl DamageBuilder {
	pub fn build(&self) -> Damage {
		Damage::new(
			self.amount,
			unique_modifiers(
				self.modifiers.iter_shared()
					.flatten()
					.map(|modifier| Arc::new(Mutex::new(ModifierWrapper { modifier })) as Arc<Mutex<dyn DamageModifier>>),
			),
		)
	}
}

pub fn flatten_damage_builders(
	damage_builders: impl IntoIterator<Item = Gd<DamageBuilder>>,
) -> Vec<Damage> {
	let damages = damage_builders.into_iter().map(|builder| {
		let builder_ref = builder.bind();
		Damage::new(
			builder_ref.amount,
			unique_modifiers(
				builder_ref.modifiers.iter_shared()
					.flatten()
					.map(|modifier| Arc::new(Mutex::new(ModifierWrapper { modifier })) as Arc<Mutex<dyn DamageModifier>>),
			),
		)
	});

	flatten_damages(damages)
}
