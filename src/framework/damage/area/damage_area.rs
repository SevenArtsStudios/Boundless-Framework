use std::collections::{HashMap, HashSet};

use godot::{
	builtin::Array,
	classes::{Area3D, IArea3D, Node, Node3D},
	obj::{Base, Gd, WithBaseField},
	prelude::{GodotClass, godot_api},
};

use crate::framework::{
	Damage, DamageAreaHitboxBuilder, DamageBuilder, DamageDealer, Damageable, GdObjectHandle, ObjectHandle, flatten_damage_builders
};

type DamageableHandle = ObjectHandle<dyn Damageable, Node3D>;
// type DamageDealerHandle = ObjectHandle<dyn DamageDealer, Node3D>;


#[derive(GodotClass)]
#[class(base=Area3D, init, tool)]
pub struct DamageArea {
	#[base]
	pub base: Base<Area3D>,

	#[init(val = 0)]
	#[export]
	pub flags: i64,

	#[init(val = 0.0)]
	#[export]
	pub life_time: f32,

	#[init(val = 0)]
	#[export]
	pub max_impacts: i32,

	#[init(val = 1.0)]
	#[export]
	pub damage_multiplier: f32,

	#[export]
	pub damage_builders: Array<Gd<DamageBuilder>>,

	#[export]
	pub damage_dealer: GdObjectHandle<dyn DamageDealer, Node>,

	impacts_history: HashMap<DamageableHandle, i32>,
	impacts_buffer: HashSet<DamageableHandle>,

	buffered_impacts_flush: bool,
}

#[godot_api]
impl DamageArea {
	pub fn new(
		base: Base<Area3D>,
		flags: i64,
		life_time: f32,
		max_impacts: i32,
		damage_multiplier: f32,
	) -> Self {
		Self {
			base,
			flags: flags,
			life_time: life_time,
			max_impacts: max_impacts,
			damage_multiplier: damage_multiplier,
			damage_builders: Default::default(),
			damage_dealer: Default::default(),
			impacts_history: Default::default(),
			impacts_buffer: Default::default(),
			buffered_impacts_flush: Default::default(),
		}
	}

	pub fn with_dealer(mut self, dealer: impl Into<GdObjectHandle<dyn DamageDealer, Node>>) -> Self {
		self.damage_dealer = dealer.into();
		self
	}

	pub fn with_hitbox(mut self, hitbox_builder: Gd<DamageAreaHitboxBuilder>) -> Self {
		hitbox_builder.bind().add_to(&mut self.base_mut());
		self
	}
	pub fn with_hitboxes(mut self, hitbox_builders: impl IntoIterator<Item = Gd<DamageAreaHitboxBuilder>>) -> Self {
		for hitbox_builder in hitbox_builders {
			hitbox_builder.bind().add_to(&mut self.base_mut());
		}

		self
	}

	pub fn build_damages(&self) -> Vec<Damage> {
		let mut damages = flatten_damage_builders(self.damage_builders.iter_shared());

		for damage in &mut damages {
			damage.amount *= self.damage_multiplier;
		}

		damages
	}

	pub fn inflict_upon(
		&mut self,
		target: &mut dyn Damageable
	) {
		let mut damages = self.build_damages();

		match &mut self.damage_dealer {
			GdObjectHandle::Some(dealer) => {
				for damage in &mut damages {
					damage.inflict_upon(target, Some(&mut *dealer.bind_mut()));
				}
			},
			_ => {
				for damage in &mut damages {
					damage.inflict_upon(target, None);
				}
			},
		}
	}

	pub fn get_impact_count(&self, target_handle: &DamageableHandle) -> i32 {
		self.impacts_history.get(target_handle).cloned().unwrap_or(0)
	}

	fn buffer_impact(&mut self, target_handle: DamageableHandle) {
		if self.max_impacts >= 0 && self.get_impact_count(&target_handle) >= self.max_impacts {
			return;
		}

		self.impacts_buffer.insert(target_handle);
		if ! self.buffered_impacts_flush {
			self.run_deferred(DamageArea::flush_buffered_impacts);
			self.buffered_impacts_flush = true;
		}
	}

	pub fn flush_buffered_impacts(&mut self) {
		let _buffered = std::mem::take(&mut self.impacts_buffer);
		for mut handle in _buffered {
			self.inflict_upon(&mut *handle.bind_mut());
			*self.impacts_history.entry(handle).or_insert(0) += 1;
		}

		self.buffered_impacts_flush = false;
		self.reset_impacts();
	}

	pub fn reset_impacts(&mut self) {
		self.impacts_buffer.clear();
	}


	#[func]
	pub fn on_area_entered(&mut self, area: Gd<Area3D>) {
		if let Ok(damageable) = area.upcast::<Node3D>().try_dynify::<dyn Damageable>() {
			self.buffer_impact(damageable.into());
		}
	}

	#[func]
	pub fn on_body_entered(&mut self, body: Gd<Node3D>) {
		if let Ok(damageable) = body.try_dynify::<dyn Damageable>() {
			self.buffer_impact(damageable.into());
		}
	}
}

#[godot_api]
impl IArea3D for DamageArea {
	fn ready(&mut self) {
		// We use untyped signals to avoid crashes when hot-reloading
		let mut gd_self = self.to_gd();

		let body_entered_callable = gd_self.callable("on_body_entered");
		gd_self.connect("body_entered", &body_entered_callable);

		let area_entered_callable = gd_self.callable("on_area_entered");
		gd_self.connect("area_entered", &area_entered_callable);


		// self.base()
		// 	.signals()
		// 	.body_entered()
		// 	.connect_other(self, DamageArea::on_body_entered);

		// self.base()
		// 	.signals()
		// 	.area_entered()
		// 	.connect_other(self, DamageArea::on_area_entered);
	}
}
