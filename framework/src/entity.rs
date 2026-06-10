use crate::{attributes::AttributeHolder, damage::{DamageDealer, Damageable}};

pub trait Entity: Damageable + DamageDealer + AttributeHolder {

}