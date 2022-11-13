extern crate serde;
use serde::Serialize;

#[derive(Serialize)]
pub struct Movie {
  pub title: String,
  pub description: String,
  pub ratings: f32
}