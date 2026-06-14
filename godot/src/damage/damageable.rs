use godot::prelude::*;
use boundless::{attributes::{AttributeProvider}, damage::{DamageInstance, Damageable}};

#[derive(Clone)]
pub struct GodotDamageable(DynGd<Node, dyn Damageable>);

impl GodotDamageable {
	pub fn from(damageable: DynGd<Node, dyn Damageable>) -> Self {
		return Self {
			0: damageable
		}
	}
}

impl AttributeProvider for GodotDamageable {
	fn get_value(&self, id: &boundless::id::Id) -> Option<f32> {
		self.0.dyn_bind().get_value(id)
	}
}

impl Damageable for GodotDamageable {
	fn get_health(&self) -> Option<f32> {
		self.0.dyn_bind().get_health()
	}

	fn damage(&mut self, damage: &DamageInstance) {
		self.0.dyn_bind_mut().damage(damage);
	}

	fn kill(&mut self) {
		self.0.dyn_bind_mut().kill();
	}
}