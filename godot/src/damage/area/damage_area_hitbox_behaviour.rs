use godot::{
	classes::{Area3D, CollisionShape3D, Resource},
	obj::{Base, Gd},
	prelude::{godot_api, GodotClass},
};

#[derive(GodotClass)]
#[class(base=Resource, init, tool)]
pub struct DamageAreaHitboxBehaviour {
	#[base]
	base: Base<Resource>,
}

#[godot_api]
impl DamageAreaHitboxBehaviour {
	#[func(virtual)]
	pub fn setup_hitbox(
		&self,
		damage_area: Option<Gd<Area3D>>,
		collision: Option<Gd<CollisionShape3D>>,
	) {
		let _ = (damage_area, collision);
	}
}
