//! A basic random sampler

use crate::{
    sampler::{InPlace, Sampler, SamplerResult, Sequential},
    types::GenFloat,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::marker::PhantomData;

/// An implementation of a basic random sampler
///
/// This sampler uses the ISAAC64 algorithm under the hood and is deterministic. You can optionally
/// provide a seed.
#[derive(Debug, Clone)]
pub struct Random<T>
where
    T: GenFloat,
{
    /// The PRNG to use with the random sampler
    ///
    /// You can use the default implementation on this sampler if you don't want to construc the
    /// RNG struct yourself, or don't care about the seed.
    pub prng: StdRng,

    /// A phantom data marker so we can use the `T` type parameter
    phantom: PhantomData<T>,
}

impl<T> Default for Random<T>
where
    T: GenFloat,
{
    fn default() -> Self {
        Self {
            prng: StdRng::seed_from_u64(42),
            phantom: PhantomData,
        }
    }
}

impl<T: GenFloat> Random<T>
where
    T: GenFloat,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    /// Create a new random sampler with a given seed
    ///
    /// The seed can be 256 elements long - anything extra will be ignored.
    pub fn with_seed(seed: u64) -> Self {
        Self {
            prng: StdRng::seed_from_u64(seed),
            phantom: PhantomData,
        }
    }
}

impl<T> InPlace<T> for Random<T>
where
    T: GenFloat,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn sample(&mut self, _index: u32, _dim: u32) -> SamplerResult<T, T> {
        Ok(self.prng.gen())
    }
}

impl<T> Sequential<T> for Random<T>
where
    T: GenFloat,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn get_next(&mut self, dimensions: u32) -> SamplerResult<Vec<T>, T> {
        Ok((0..dimensions).map(|_| self.prng.gen()).collect())
    }
}

impl<T> Sampler<T> for Random<T>
where
    T: GenFloat,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn sample_idx(&mut self, _index: u32) -> SamplerResult<T, T> {
        Ok(self.prng.gen())
    }

    fn sampler_idx_dims(&mut self, _: u32, dimensions: u32) -> SamplerResult<Vec<T>, T> {
        Ok((0..dimensions).map(|_| self.prng.gen()).collect())
    }

    fn next(&mut self, dimensions: u32) -> SamplerResult<Vec<T>, T> {
        Ok((0..dimensions).map(|_| self.prng.gen()).collect())
    }
}
