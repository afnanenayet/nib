//! Defines a `Ray` type with convenience methods

use crate::types::{approx_eq_vec, Float};
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

/// A standard ray with an origin point and a directional vector
#[derive(Debug, Serialize, Deserialize)]
pub struct Ray {
    /// The origin point of the ray in three-dimensional space
    pub origin: Vector3<Float>,

    /// The normalized direction of the ray
    ///
    /// The direction of the ray is represented as a normalized 3D vector, which means that every
    /// component of the vector must be between 0 and 1.
    pub direction: Vector3<Float>,
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        approx_eq_vec(&self.origin, &other.origin)
            && approx_eq_vec(&self.direction, &other.direction)
    }
}

impl Ray {
    /// A convenience method to create a new ray given an origin and direction
    pub fn new(origin: Vector3<Float>, direction: Vector3<Float>) -> Self {
        Self { origin, direction }
    }
}
