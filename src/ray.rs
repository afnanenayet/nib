//! Defines a `Ray` type with convenience methods

use cgmath::Vector3;
use serde::{Deserialize, Serialize};

/// A standard ray with an origin point and a directional vector
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ray<T> {
    /// The origin point of the ray in three-dimensional space
    pub origin: Vector3<T>,

    /// The normalized direction of the ray
    ///
    /// The direction of the ray is represented as a normalized 3D vector, which means that every
    /// component of the vector must be between 0 and 1.
    pub direction: Vector3<T>,
}

impl<T> Ray<T> {
    /// A convenience method to create a new ray given an origin and direction
    pub fn new(origin: Vector3<T>, direction: Vector3<T>) -> Self {
        Self { origin, direction }
    }
}
