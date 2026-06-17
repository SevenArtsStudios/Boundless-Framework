use crate::{attributes::ModifiedAttributeValue::Modified, math::lerp};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModifiedAttributeValue {
	Base(f32),
	Modified(f32)
}

impl ModifiedAttributeValue {
	#[must_use]
	const fn value(self) -> f32 {
		match self {
			Self::Base(val) | Self::Modified(val) => val,
		}
	}
}

pub trait AttributeModifier {
	#[must_use]
	fn apply_to(&self, base_value: f32) -> ModifiedAttributeValue;

	#[must_use]
	fn operation(&self) -> AttributeModifierOperation;

	#[inline]
	#[must_use]
	fn strength(&self) -> f32 { 1.0 }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AttributeModifierOperation {
	Set { to: f32 },
	Multiply { with: f32, stacking: bool },
	Add { value: f32 },
	MoreThan { minimum: f32, deferred: bool },
	LessThan { maximum: f32, deferred: bool },
}

impl AttributeModifier for AttributeModifierOperation {
	fn apply_to(&self, base_value: f32) -> ModifiedAttributeValue {
		Modified(match self {
			Self::Set { to } => *to,
			Self::Multiply { with, .. } => base_value * with,
			Self::Add { value } => base_value + *value,
			Self::MoreThan { minimum, .. } => base_value.max(*minimum),
			Self::LessThan { maximum, .. } => base_value.min(*maximum),
		})
	}

	fn operation(&self) -> AttributeModifierOperation {
		*self
	}
}

pub fn apply_modifiers<'a, I, T>(modifiers: I, base_value: f32) -> f32
where
	I: IntoIterator<Item = T>,
	T: AttributeModifier + 'a,
{
	let mut init = base_value;
	let mut max = f32::INFINITY;
	let mut min = f32::NEG_INFINITY;
	let mut add = 0.0;
	let mut mult = 1.0;

	for entry in modifiers {
		match entry.operation() {
			AttributeModifierOperation::Multiply { with, stacking: true } => { // Stacking Multiply STACKS with other Multiplies
				mult *= lerp(mult, with, entry.strength());
			},
			AttributeModifierOperation::MoreThan { minimum, deferred: false } => { // NON-DEFERRED MoreThan = calculation starts HIGHER THAN minimum
				init = lerp(init, init.max(minimum), entry.strength());
			},
			AttributeModifierOperation::MoreThan { minimum, deferred: true } => { // DEFERRED MoreThan = calculation result CANNOT GO LOWER than minimum
				let strength = entry.strength();
				if min.is_finite() {
					min = lerp(min, min.max(minimum), strength);
				} else if strength > 0.0 {
					min = minimum;
				}
			},
			AttributeModifierOperation::LessThan { maximum, deferred: false } => { // NON-DEFERRED LessThan = calculation starts LOWER THAN maximum
				init = lerp(init, init.min(maximum), entry.strength());
			},
			AttributeModifierOperation::LessThan { maximum, deferred: true } => { // DEFERRED LessThan = calculation result CANNOT GO HIGHER than maximum
				let strength = entry.strength();
				if max.is_finite() {
					max = lerp(max, max.min(maximum), strength);
				} else if strength > 0.0 {
					max = maximum;
				}
			},
			_ => add += lerp(base_value, entry.apply_to(base_value).value(), entry.strength()) - base_value, // Add the 'SUM' of the operation, meaning nothing stacks
		}
	}

	((init + add) * mult).clamp(min, max)
}

pub trait TraitModifierIterator {
	fn apply_to(self, base_value: f32) -> f32;
}


impl< I, T> TraitModifierIterator for I
where
	I: IntoIterator<Item = T>,
	T: AttributeModifier,
{
	fn apply_to(self, base_value: f32) -> f32 {
		apply_modifiers(self, base_value)
	}
}