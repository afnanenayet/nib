//! Sampling functions for various primitives
//!
//! This module implements sampling functions for different geometric primitives.

use crate::{sampler::Sampler, types::GenFloat};
use cgmath::{InnerSpace, Vector3};

/// Generate a random sample in the unit sphere
///
/// This generates a random sample in the unit sphere by generating random vectors in each axis
/// which generates a random sample in the unit cube, and then normalizing the vectors so that they
/// lie within the unit sphere.
///
/// We reject vectors with very small norms, as this can be an issue with floating point numbers.
pub fn sample_unit_sphere<T: GenFloat>(sampler: &mut dyn Sampler<T>) -> Vector3<T> {
    let mut v = Vector3::new(
        T::from(2).unwrap(),
        T::from(2).unwrap(),
        T::from(2).unwrap(),
    );
    let unit = Vector3::new(
        T::from(1).unwrap(),
        T::from(1).unwrap(),
        T::from(1).unwrap(),
    );
    let scaling_factor = T::from(2).unwrap();
    while v.magnitude2() >= T::from(1.0).unwrap() {
        let rs = sampler.next(3).unwrap();
        debug_assert_eq!(rs.len(), 3);
        v = (Vector3::new(rs[0], rs[1], rs[2]) * scaling_factor) - unit;
    }
    v
}

/// Generate a random sample in the unit disk
///
/// This method uses rejection sampling to generate the point.
pub fn sample_unit_disk<T: GenFloat>(sampler: &mut dyn Sampler<T>) -> Vector3<T> {
    let mut v = Vector3::new(
        T::from(2).unwrap(),
        T::from(2).unwrap(),
        T::from(2).unwrap(),
    );

    while v.magnitude2() >= T::from(1.0).unwrap() {
        let rs = sampler.next(2).unwrap();
        v = (Vector3::new(rs[0], rs[1], T::from(0).unwrap())
            - Vector3::new(
                T::from(1).unwrap(),
                T::from(1).unwrap(),
                T::from(0).unwrap(),
            ))
        .map(|x| x * T::from(2).unwrap());
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sampler::Random;

    // TODO add a test to check if the outputs are uniformly distributed within the sphere
    #[test]
    fn test_sample_unit_sphere() {
        let mut sampler = Random::default();

        for _ in 0..100 {
            let sphere_coordinates = sample_unit_sphere::<f32>(&mut sampler);

            // Basic sanity check to ensure that all of the coordinates are within the proper range
            assert!(sphere_coordinates[0] >= -1.0 && sphere_coordinates[0] <= 1.0);
            assert!(sphere_coordinates[1] >= -1.0 && sphere_coordinates[1] <= 1.0);
            assert!(sphere_coordinates[2] >= -1.0 && sphere_coordinates[2] <= 1.0);
        }
    }
}
