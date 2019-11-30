//! The sampler interface
//!
//! This module defines a generic sampling interface that can be extended to various sampling
//! implementations.

use crate::types::GenFloat;
use thiserror::Error;

mod random;

pub use random::Random;

/// The possible errors that a `Sampler` can return
#[derive(Error, Debug, Eq, PartialEq)]
pub enum SamplerError {
    /// The caller requested more dimensions than the sampler could provide
    ///
    /// Some samplers are limited in the amount of dimensions they can provide. This error
    /// indicates that the caller requested more dimensions than the sampler could provide. The
    /// sampler can indicate how many dimensions can be provided for this sampler, if the sampler
    /// can provide some dimensions, but less than the amount that was requested.
    #[error(
        "Too many dimensions were requested, but you can request {dimensions_left:?} dimensions."
    )]
    TooManyDims {
        /// The number of dimensions remaining for the sampler, for this index. There may be no
        /// dimensions remaining
        dimensions_left: u32,
    },

    /// The sampler has been exhausted, and cannot generate any more samples
    ///
    /// This error is incurred when the sample can't generate any more points. This is a separate
    /// issue from the `TooManyDims` error, which represents when the sampler cannot generate any
    /// more dimensional samples for a sample.
    #[error("The sampler has been exhausted and cannot generate any more samples.")]
    NoSamplesRemaining,
}

/// A convenient type alias for when a result can return a `SamplerError`
pub type SamplerResult<T> = Result<T, SamplerError>;

/// The interface for a sampler that needs to be queried sequentially
///
/// The base sampler trait is the basic interface for samplers to implement, which give you all of
/// the methods from the `Sampler` interface for free.
///
/// The `Sampler` method defines a number of convenience methods that are repetitive that can be
/// derived from the methods of the `BaseSampler` trait.
pub trait Sequential<T: GenFloat> {
    /// Retrieve the next sample
    ///
    /// This retrieves the next sample with all of the dimensions.
    // TODO(afnan) should we use `&mut self` instead?
    fn next(&self) -> SamplerResult<Vec<T>>;
}

/// The interface for an in-place sampler that can stochastically query samples
///
/// This is meant for samplers that can generate any sample in O(1)/on-the-fly with negligible
/// overhead without requiring information about previous samples. These types of samplers tend to
/// be the most performant.
pub trait InPlace<T: GenFloat> {
    /// Sample a particular dimension and index
    fn sample(&mut self, index: u32, dim: u32) -> SamplerResult<T>;
}

/// The generic interface for a rendering sampler
///
/// This method automatically derives these methods for any sampler that implements the
/// `BaseSampler` trait. Users should use this trait and *not* the `BaseSampler` trait as an
/// interface.
pub trait Sampler<T: GenFloat> {
    /// Sample all of the dimensions for a particular index
    // TODO(afnan) should we use `&mut self` instead?
    fn sample_idx(&self, index: u32) -> SamplerResult<Vec<T>>;
}
