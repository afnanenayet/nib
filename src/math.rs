//! Utility math functions

use crate::{
    sampler::Sampler,
    types::{eta, GenFloat, GenReal},
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
    let mut v = Vector3::new(
        T::from(0).unwrap(),
        T::from(0).unwrap(),
        T::from(0).unwrap(),
    );
    while v.magnitude() < eta() {
        let rs = sampler.next(3).unwrap();
        debug_assert_eq!(rs.len(), 1);
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
