use macroquad::prelude::*;

use crate::Asset;

pub struct Country {
  pub position: Vec2,
  pub asset: Asset,
  pub name: String,
  pub data: CountryData
}
#[derive(Clone, Default)]
pub struct CountryData {
  pub influence: f32
}

