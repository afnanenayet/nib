//! Utility math functions

use crate::{
    sampler::Sampler,
    types::{GenFloat, GenReal, ETA},
};
use cgmath::{prelude::*, Vector3};
use rand;

/// Generate a random sample in the unit sphere
///
/// This generates a random sample in the unit sphere by generating random vectors in each axis
/// which generates a random sample in the unit cube, and then normalizing the vectors so that they
/// lie within the unit sphere.
///
/// We reject vectors with very small norms, as this can be an issue with floating point numbers.
pub fn sample_unit_sphere<T: GenFloat>(sampler: &mut dyn Sampler<T>) -> Vector3<T> {
    let rs = sampler.next(3).unwrap();
    let mut v = Vector3::new(
        T::from(0).unwrap(),
        T::from(0).unwrap(),
        T::from(0).unwrap(),
    );
    while v.magnitude() < T::from(ETA).unwrap() {
        v = Vector3::new(rs[0], rs[1], rs[2]);
    }
    // Normalize the vector so it has a magnitude of one
    v.normalize()
}

/// Mirror a vector about a unit direction
///
/// `vector` is the incoming vector, and `normal` is the vector to mirror `vector` around. Returns
/// a mirrored vector. Note that `normal` must be a unit vector.
pub fn mirror<T: GenFloat>(vector: &Vector3<T>, normal: &Vector3<T>) -> Vector3<T> {
    vector - (normal * T::from(2).unwrap() * vector.dot(*normal))
}

/// Multiply two vectors together in a component wise fashion
///
/// This is the equivalent of A' * B, or (A transpose) multiplied with B. This method will multiply
/// each corresponding component of the vectors together.
pub fn component_mul<T: GenReal>(a: Vector3<T>, b: Vector3<T>) -> Vector3<T> {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}
