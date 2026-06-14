use std::sync::{Arc, Mutex};

use boundless::damage::{DamageInstance};
use godot::register::{GodotClass, godot_api};

use crate::GodotId;

#[derive(GodotClass, Clone)]
#[class(base=RefCounted, no_init, rename=DamageInstance)]
pub struct GodotDamageInstance {
	damage_instance: Arc<Mutex<DamageInstance>>,
}

#[godot_api]
impl GodotDamageInstance {
	pub fn from(damage_instance: Arc<Mutex<DamageInstance>>) -> Self {
		Self {
			damage_instance,
		}
	}

	#[func]
	pub fn get_amount(&self) -> f32 {
		self.damage_instance.lock().unwrap().amount()
	}

	#[func]
	pub fn scale_damage(
		&mut self,
		resistance_attribute: GodotId,
		strength_attribute: GodotId,
		allow_negative: bool,
	) {
		self.damage_instance.lock().unwrap()
			.scale(
				&resistance_attribute.into(),
				&strength_attribute.into(),
				allow_negative
			);
	}
}