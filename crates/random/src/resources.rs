use bevy::prelude::*;
use rand::{rng, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Resource)]
pub struct SeededRng {
    seed: u64,
    rng: ChaCha8Rng,
}

impl Default for SeededRng {
    fn default() -> Self {
        let seed = rng().random();
        Self::new(seed)
    }
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        SeededRng {
            seed,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    // -- Getters --

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn rng(&mut self) -> &mut ChaCha8Rng {
        &mut self.rng
    }
}
