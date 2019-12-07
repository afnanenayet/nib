//! Utility math functions  

use crate::types::{GenFloat, ETA};
use cgmath::{prelude::*, Vector3};
use rand::prelude::*;

/// Generate a random sample in the unit sphere
///
/// This generates a random sample in the unit sphere by generating random vectors in each axis
/// which generates a random sample in the unit cube, and then normalizing the vectors so that they
/// lie within the unit sphere.
///
/// We reject vectors with very small norms, as this can be an issue with floating point numbers.
pub fn sample_unit_sphere<T, R>(r: &mut R) -> Vector3<T>
where
    T: GenFloat,
    R: Rng + ?Sized,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    let mut v = Vector3::new(
        T::from(0).unwrap(),
        T::from(0).unwrap(),
        T::from(0).unwrap(),
    );
    while v.magnitude() < T::from(ETA).unwrap() {
        v = Vector3::new(r.gen(), r.gen(), r.gen());
    }
    // Normalize the vector so it has a magnitude of one
    v.normalize()
}
