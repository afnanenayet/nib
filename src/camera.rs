//! A module defining the generic interface for cameras and providing interfaces for various camera
//! types

use crate::types::{GenFloat, Ray};

mod pinhole;

pub use pinhole::Pinhole;

/// The generic interface for a camera type
///
/// A camera simply needs to convert u, v coordinates to a 3D ray.
pub trait Camera<T: GenFloat> {
    /// Convert (u, v) pixel coordinates to a ray in 3D space
    fn to_ray(&self, u: T, v: T) -> Ray<T>;
}
