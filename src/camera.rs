//! A module defining the generic interface for cameras and providing interfaces for various camera
//! types

use crate::types::Ray;

/// The generic interface for a camera type
///
/// A camera simply needs to convert u, v coordinates to a 3D ray.
pub trait Camera<T> {
    /// Convert (u, v) coordinates to a ray in 3D space
    fn to_ray(&self, u: T, v: T) -> Ray<T>;
}
