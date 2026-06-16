use crate::math::lerp;

#[derive(Clone, PartialEq, Debug)]
pub enum AttributeModifier {
	Set(f32),
	Multiply(f32, bool),
	Add(f32),
	MoreThan(f32, bool),
	LessThan(f32, bool),
}

impl AttributeModifier {
	#[must_use]
	pub fn apply_to(&self, base_value: f32) -> f32 {
		match self {
			Self::Set(value) => *value,
			Self::Multiply(value, _) => base_value * value,
			Self::Add(value) => base_value + *value,
			Self::MoreThan(value, _) => base_value.max(*value),
			Self::LessThan(value, _) => base_value.min(*value),
		}
	}
}

pub struct AttributeModifierEntry {
	modifier: AttributeModifier,
	strength: f32,
}

impl AttributeModifierEntry {
	#[must_use]
	pub const fn new(modifier: AttributeModifier, strength: f32) -> Self {
		Self { modifier, strength }
	}

	#[must_use]
	pub fn apply_to(&self, base_value: f32) -> f32 {
		lerp(base_value, self.modifier.apply_to(base_value), self.strength)
	}
}

pub fn apply_modifiers<I: IntoIterator<Item = AttributeModifierEntry>>(modifiers: I, base_value: f32) -> Option<f32> {
	let mut init = base_value;
	let mut add = 0.0;
	let mut mult = 1.0;

	for entry in modifiers {
		match entry.modifier {
			AttributeModifier::Multiply(value, is_additive) if is_additive => {
				mult *= lerp(1.0, value, entry.strength);
			},
			AttributeModifier::LessThan(value, is_initial) if is_initial => {
				init = lerp(init, init.min(value), entry.strength);
			},
			AttributeModifier::MoreThan(value, is_initial) if is_initial => {
				init = lerp(init, init.max(value), entry.strength);
			},
			_ => add += lerp(base_value, entry.apply_to(base_value), entry.strength) - base_value,
		}
	}

	Some((init + add) * mult)
}