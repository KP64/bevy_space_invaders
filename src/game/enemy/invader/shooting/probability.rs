use bevy::prelude::*;

pub(super) const INCREMENT_RATE: f64 = 0.1;
pub(super) const LOWEST: f64 = 0.3;
pub(super) const HIGHEST: f64 = 1.0;

#[derive(Resource, Deref, DerefMut)]
pub struct Probability(pub f64);

impl Default for Probability {
    fn default() -> Self {
        Self(LOWEST)
    }
}

impl Probability {
    pub fn increase(&mut self) {
        let new_prob = self.0 + INCREMENT_RATE;
        self.0 = new_prob.clamp(LOWEST, HIGHEST);
    }
}
