//! The sampler interface
//!
//! This module defines a generic sampling interface that can be extended to various sampling
//! implementations.

use crate::types::GenFloat;
use std::fmt::Debug;
use thiserror::Error;

mod random;

pub use random::Random;

/// The possible errors that a `Sampler` can return
#[derive(Error, Debug, Eq, PartialEq)]
pub enum SamplerError<T: GenFloat> {
    /// The caller requested more dimensions than the sampler could provide
    ///
    /// Some samplers are limited in the amount of dimensions they can provide. This error
    /// indicates that the caller requested more dimensions than the sampler could provide. This
    /// error handles the case where the sampler is completely exhausted.
    ///
    /// In the event that the sampler is able to return a partial result, but not all of the
    /// requested dimensions, the `IncompleteDimensions` variant should be used.
    #[error("Too many dimensions were requested")]
    TooManyDims,

    /// The sampler has been exhausted, and cannot generate any more samples
    ///
    /// This error is incurred when the sample can't generate any more points. This is a separate
    /// issue from the `TooManyDims` error, which represents when the sampler cannot generate any
    /// more dimensional samples for a sample.
    #[error("The sampler has been exhausted and cannot generate any more samples.")]
    NoSamplesRemaining,

    /// The user requested more samples for an inded than the sampler was able to provide
    ///
    /// This error is used when the sampler is able to provide *some* of the dimensions, but not
    /// all of them. This error variant indicates that the sampler has been exhausted, but will
    /// provide the dimensions that the sampler had left.
    ///
    /// In the event that the sampler can't produce any more dimensions and has been completely
    /// exhausted for this index, use the `TooManyDims` variant.
    #[error(
        "The sampler was able to provide {provided:?} dimensions, but {requested:?} were requsted"
    )]
    IncompleteDimensions {
        /// The dimensions the sampler was able to supply
        sample: Vec<T>,

        /// The number of dimensions that were requested
        ///
        /// This number must always be greater than 0.
        requested: u32,

        /// The number of dimensions the sampler was able to provide
        ///
        /// This number will always be greater than 0.
        provided: u32,
    },
}

/// A convenient type alias for when a result can return a `SamplerError`
pub type SamplerResult<T, N> = Result<T, SamplerError<N>>;

/// The interface for a sampler that needs to be queried sequentially
///
/// The base sampler trait is the basic interface for samplers to implement, which give you all of
/// the methods from the `Sampler` interface for free.
///
/// The `Sampler` method defines a number of convenience methods that are repetitive that can be
/// derived from the methods of the `BaseSampler` trait.
pub trait Sequential<T: GenFloat>: Debug + Send + Sync
where
    T: GenFloat,
{
    /// Retrieve the next sample
    ///
    /// This retrieves number of dimensions specified for the next sample
    fn get_next(&mut self, dimensions: u32) -> SamplerResult<Vec<T>, T>;
}

/// The interface for an in-place sampler that can stochastically query samples
///
/// This is meant for samplers that can generate any sample in O(1)/on-the-fly with negligible
/// overhead without requiring information about previous samples. These types of samplers tend to
/// be the most performant.
pub trait InPlace<T: GenFloat>: Debug + Send + Sync
where
    T: GenFloat,
{
    /// Sample a particular dimension and index
    fn sample(&mut self, index: u32, dim: u32) -> SamplerResult<T, T>;
}

/// The generic interface for a rendering sampler
///
/// This method automatically derives these methods for any sampler that implements the
/// `BaseSampler` trait. Users should use this trait and *not* the `BaseSampler` trait as an
/// interface.
pub trait Sampler<T>: Debug + Send + Sync
where
    T: GenFloat,
{
    /// Sample all of the dimensions for a particular index
    // TODO(afnan) should we use `&mut self` instead?
    fn sample_idx(&mut self, index: u32) -> SamplerResult<T, T>;

    /// Request a certain number of dimensions for a particular index
    fn sampler_idx_dims(&mut self, index: u32, dimensions: u32) -> SamplerResult<Vec<T>, T>;

    /// Get the next `dimensions` samples.
    ///
    /// If there are no more dimensions remaining for this particular index, then this will return
    /// an error, or an incomplete
    fn next(&mut self, dimensions: u32) -> SamplerResult<Vec<T>, T>;
}
