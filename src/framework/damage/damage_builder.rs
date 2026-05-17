use godot::{builtin::Array, classes::Resource, obj::{DynGd, Gd}, prelude::GodotClass};

use crate::framework::{Damage, DamageModifier, flatten_damages, unique_modifiers};

#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
pub struct DamageBuilder {
	#[export]
	#[init(val=1.0)]
	pub amount: f32,
	#[export]
	pub modifiers: Array<Option<DynGd<Resource, dyn DamageModifier>>>,
}

impl DamageBuilder {
	pub fn build(&self) -> Damage {
		Damage::new(
			self.amount,
			self.modifiers.iter_shared().flatten(),
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
			unique_modifiers(builder_ref.modifiers.iter_shared().flatten()),
		)
	});

	flatten_damages(damages)
}
