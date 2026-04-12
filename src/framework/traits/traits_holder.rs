use std::ops::Deref;

use crate::framework::TraitsProvider;

pub trait TraitsHolder {
	fn traits(&self) -> Option<impl Deref<Target = impl TraitsProvider> + '_>
	where
		Self: TraitsProvider + Sized,
	{
		None::<&Self>
	}
}