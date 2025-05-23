//! Contains reusable utility functions

use rand::Rng;

#[allow(dead_code)]
pub fn random_number(min: f32, max: f32) -> f32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}
