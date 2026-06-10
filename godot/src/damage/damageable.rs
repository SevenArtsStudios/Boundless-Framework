use godot::prelude::*;
use boundless::{attributes::{AttributeHolder, AttributeProvider}, damage::{Damageable}};

#[derive(Clone)]
pub struct GodotDamageable(DynGd<Node, dyn Damageable>);

impl GodotDamageable {
	pub fn from(item: DynGd<Node, dyn Damageable>) -> Self {
		return Self {
			0: item
		}
	}
}

impl AttributeHolder for GodotDamageable {
	fn attributes(&self) -> Option<Box<dyn AttributeProvider>> {
		self.0.dyn_bind().attributes()
	}
}

impl Damageable for GodotDamageable {
	fn get_health(&self) -> Option<f32> {
		self.0.dyn_bind().get_health()
	}

	fn apply_damage(&mut self, amount: f32) {
		self.0.dyn_bind_mut().apply_damage(amount);
	}

	fn kill(&mut self) {
		self.0.dyn_bind_mut().kill();
	}
}