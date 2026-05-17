use godot::{
	builtin::Array,
	prelude::GodotClass,
	obj::Gd,
};

use crate::framework::{
	DamageArea, DamageAreaHitboxBuilder
};

#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
pub struct DamageAreaBuilder {
	#[export]
	pub hitbox_builders: Array<Gd<DamageAreaHitboxBuilder>>,

	#[init(val = 0)]
	#[export]
	pub flags: i64,

	#[init(val = -1.0)]
	#[export]
	pub life_time: f32,

	#[init(val = -1)]
	#[export]
	pub max_impacts: i32,

	#[init(val = 1.0)]
	#[export]
	pub damage_multiplier: f32,
}

impl DamageAreaBuilder {
	pub fn build(&self) -> Gd<DamageArea> {
		Gd::from_init_fn(|base| {
			DamageArea::new(base, self.flags, self.life_time, self.max_impacts, self.damage_multiplier)
				.with_hitboxes(self.hitbox_builders.iter_shared())
		})
	}
}
