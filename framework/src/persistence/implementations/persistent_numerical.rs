use crate::{Clamped, ItemDataProvider, persistence::{PersistentState, PersistentValue}};

/// A persistent numerical state.
pub struct PersistentNumerical<T: Send + Sync + Clone + Clamped + 'static> {
	min: Option<T>,
	max: Option<T>,
	default_value: T,
	value: T,
	on_update: Option<Box<dyn Fn(T) + Send + Sync>>,
}

impl<T: Send + Sync + Clone + Clamped + 'static> PersistentNumerical<T> {
	pub fn new(min: Option<T>, max: Option<T>, default_value: T) -> Self {
		let value = default_value.clone();
		Self {
			min,
			max,
			default_value,
			value,
			on_update: None,
		}
	}

	pub fn on_update(&mut self, callback: Box<dyn Fn(T) + Send + Sync>) {
		self.on_update = Some(callback);
	}

	pub fn value(&self) -> T {
		self.value.clone()
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PersistentNumericalValue<T: Send + Sync + Clone + Clamped + 'static>(T);


impl<T: Send + Sync + Clone + Clamped + 'static> PersistentState for PersistentNumerical<T> {
	fn update(&mut self, value: &dyn PersistentValue, _registry: Option<&dyn ItemDataProvider>) {
		self.value = value
			.as_any()
			.downcast_ref::<PersistentNumericalValue<T>>()
			.map(|value| value.0.clamped(self.min.clone(), self.max.clone()))
			.unwrap_or_else(|| self.default_value.clone());

		if let Some(callback) = &self.on_update {
			callback(self.value.clone());
		}
	}

	fn save(&self) -> Box<dyn PersistentValue> {
		Box::new(PersistentNumericalValue::<T>(self.value.clone()))
	}
}

pub type PersistentInteger = PersistentNumerical<i32>;
pub type PersistentIntegerValue = PersistentNumericalValue<i32>;

pub type PersistentDecimal = PersistentNumerical<f32>;
pub type PersistentDecimalValue = PersistentNumericalValue<f32>;