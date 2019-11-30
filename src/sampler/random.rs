//! A basic random sampler

use crate::sampler::SamplerResult;
use crate::{sampler::InPlace, types::GenFloat};
use rand::{isaac::Isaac64Rng, Rng, SeedableRng};

/// An implementation of a basic random sampler
///
/// This sampler uses the ISAAC64 algorithm under the hood and is deterministic. You can optionally
/// provide a seed.
#[derive(Debug)]
pub struct Random {
    /// The RNG to use with the random sampler
    ///
    /// You can use the default implementation on this sampler if you don't want to construc the
    /// RNG struct yourself, or don't care about the seed.
    pub rand: Isaac64Rng,

    /// An optional seed for the RNG
    ///
    /// This is part of the sampler struct because the RNG expects a reference to an array of 64
    /// bit integers, and this struct needs to own the references.
    pub seed: Vec<u64>,
}

impl Default for Random {
    fn default() -> Self {
        Self {
            rand: Isaac64Rng::new_unseeded(),
            // The empty vector does not allocate, so there are no performance concerns here.
            seed: Vec::new(),
        }
    }
}

impl Random {
    /// Create a new random sampler with a given seed
    ///
    /// The seed can be 256 elements long - anything extra will be ignored.
    pub fn with_seed(seed: Vec<u64>) -> Self {
        Self {
            rand: Isaac64Rng::from_seed(&seed[..]),
            seed,
        }
    }
}

impl<T: GenFloat> InPlace<T> for Random {
    fn sample(&mut self, _index: u32, _dim: u32) -> SamplerResult<T> {
        Ok(self.rand.gen())
    }
}
