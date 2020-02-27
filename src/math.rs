//! Utility math functions

use crate::{sampler::Sampler, types::GenFloat};
use cgmath::{prelude::*, Vector3};
use num::pow;

/// Mirror a vector about a unit direction
///
/// `vector` is the incoming vector, and `normal` is the vector to mirror `vector` around. Returns
/// a mirrored vector. Note that `normal` must be a unit vector.
pub fn mirror<T: GenFloat>(vector: &Vector3<T>, normal: &Vector3<T>) -> Vector3<T> {
    vector - (normal * T::from(2).unwrap() * vector.dot(*normal))
}

/// Schlick's algorithm for computing a reflection coefficient
///
/// An implementation of Schlick's algorithm for approximating the contribution of the Fresnel
/// factor in a specular reflection.
pub fn schlick<T: GenFloat>(cosine: T, ref_idx: T) -> T {
    let r0 = (T::from(1).unwrap() - ref_idx) / (T::from(1).unwrap() + ref_idx);
    let r0 = r0 * r0;
    r0 + (T::from(1).unwrap() - r0) * pow(T::from(1).unwrap() - cosine, 5)
}
