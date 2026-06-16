use crate::id::Id;

pub trait AttributeProvider {
	fn get_attribute(&self, id: &Id) -> Option<f32>;
}