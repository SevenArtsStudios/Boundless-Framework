use crate::id::Id;

pub trait AttributeProvider {
	fn get_value(&self, id: &Id) -> Option<f32>;
}