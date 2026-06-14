use std::sync::{Arc, Mutex};

use godot::{builtin::Array, classes::Resource, obj::{DynGd, Gd}, prelude::*};

use boundless::damage::{Damage, DamageDealer, DamageModifier, Damageable, flatten_damages};
use itertools::Itertools;

use crate::{DamageModifierWrapper, GodotDamageDealer, GodotDamageInstance, GodotDamageable};

#[derive(GodotClass)]
#[class(base=Resource, init, tool, rename = Damage)]
pub struct GodotDamage {
	#[export]
	#[init(val=1.0)]
	pub amount: f32,
	#[export]
	pub modifiers: Array<Option<DynGd<Resource, dyn DamageModifier>>>,
}

#[godot_api]
impl GodotDamage {
	pub fn build(&self) -> Damage {
		Damage::new(
			self.amount,
			self.modifiers.iter_shared()
				.flatten()
				.unique()
				.map(|modifier| Arc::<DamageModifierWrapper>::new(modifier.into()) as Arc<dyn DamageModifier>)
		)
	}

	#[func]
	pub fn instantiate(
		&mut self,
		target: DynGd<Node, dyn Damageable>,
		damage_dealer: Option<DynGd<Node, dyn DamageDealer>>,
	) -> Gd<GodotDamageInstance> {
		let damage = self.build();

		let damage_instance = damage.instantiate(
			GodotDamageable::from(target),
			damage_dealer.map(GodotDamageDealer::from)
		);

		GodotDamageInstance::gd_from(Arc::new(Mutex::new(damage_instance)))
	}

	#[func]
	pub fn inflict(
		&mut self,
		target: DynGd<Node, dyn Damageable>,
		damage_dealer: Option<DynGd<Node, dyn DamageDealer>>,
	) {
		let damage = self.build();

		let damage_instance = damage.instantiate(
			GodotDamageable::from(target),
			damage_dealer.map(GodotDamageDealer::from)
		);
		damage_instance.inflict();
	}
}

impl Into<Damage> for GodotDamage {
	fn into(self) -> Damage {
		self.build()
	}
}

pub fn flatten_damage_builders(
	damage_builders: impl IntoIterator<Item = Gd<GodotDamage>>,
) -> Vec<Damage> {
	let damages = damage_builders.into_iter().map(|builder| {
		let builder_ref = builder.bind();
		Damage::new(
			builder_ref.amount,
			builder_ref.modifiers.iter_shared()
				.flatten()
				.unique()
				.map(|modifier| Arc::<DamageModifierWrapper>::new(modifier.into()) as Arc<dyn DamageModifier>),
		)
	});

	flatten_damages(damages)
}
