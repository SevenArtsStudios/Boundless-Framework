use crate::{damage::{DamageDealer, Damageable}};

pub trait Entity: Damageable + DamageDealer {

}