use crate::framework::TraitsProvider;

pub trait TraitsHolder {
	fn traits(&self) -> Option<Box<dyn TraitsProvider>>;
}