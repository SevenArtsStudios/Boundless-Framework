use crate::attributes::AttributeProvider;

pub trait AttributeHolder {
	fn attributes(&self) -> Option<Box<dyn AttributeProvider>>;
}