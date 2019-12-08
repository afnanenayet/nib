//! A basic random sampler

use crate::sampler::SamplerResult;
use crate::{
    sampler::{InPlace, Sequential},
    types::GenFloat,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

/// An implementation of a basic random sampler
///
/// This sampler uses the ISAAC64 algorithm under the hood and is deterministic. You can optionally
/// provide a seed.
#[derive(Debug, Clone)]
pub struct Random {
    /// The PRNG to use with the random sampler
    ///
    /// You can use the default implementation on this sampler if you don't want to construc the
    /// RNG struct yourself, or don't care about the seed.
    pub prng: StdRng,
}

impl Default for Random {
    fn default() -> Self {
        Self {
            prng: StdRng::seed_from_u64(42),
        }
    }
}

impl Random {
    /// Create a new random sampler with a given seed
    ///
    /// The seed can be 256 elements long - anything extra will be ignored.
    pub fn with_seed(seed: u64) -> Self {
        Self {
            prng: StdRng::seed_from_u64(seed),
        }
    }
}

impl<T> InPlace<T> for Random
where
    T: GenFloat,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn sample(&mut self, _index: u32, _dim: u32) -> SamplerResult<T, T> {
        Ok(self.prng.gen())
    }
}

impl<T> Sequential<T> for Random
where
    T: GenFloat,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn next(&mut self, dimensions: u32) -> SamplerResult<Vec<T>, T> {
        Ok((0..dimensions).map(|_| self.prng.gen()).collect())
    }
}
