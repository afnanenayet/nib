//! Utility math functions

use crate::types::Float;
use cgmath::{prelude::*, Vector3};
use num::pow;

/// Mirror a vector about a unit direction
///
/// `vector` is the incoming vector, and `normal` is the vector to mirror `vector` around. Returns
/// a mirrored vector. Note that `normal` must be a unit vector.
pub fn mirror(vector: &Vector3<Float>, normal: &Vector3<Float>) -> Vector3<Float> {
    vector - (normal * 2.0 * vector.dot(*normal))
}

/// Schlick's algorithm for computing a reflection coefficient
///
/// An implementation of Schlick's algorithm for approximating the contribution of the Fresnel
/// factor in a specular reflection.
pub fn schlick(cosine: Float, ref_idx: Float) -> Float {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * pow(1.0 - cosine, 5)
}
