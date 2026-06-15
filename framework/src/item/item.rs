use crate::{Persistent, Id};

pub trait Item {
	fn item_key(&self) -> Id;
}

pub trait ItemData: Send + Sync {
	fn item_key(&self) -> Id;
	fn instantiate(&self) -> Result<Box<dyn Persistent>, ()>;
}