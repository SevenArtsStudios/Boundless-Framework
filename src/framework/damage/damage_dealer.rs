use std::ops::Deref;

use crate::framework::{Damageable, TraitsHolder};

pub trait DamageDealer: TraitsHolder {
	fn get_damageable(&self) -> Option<impl Deref<Target = impl Damageable> + '_>
	where
		Self: Damageable +Sized,
	{
		None::<&Self>
	}
}