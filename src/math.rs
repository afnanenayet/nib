//! Utility math functions

use crate::{sampler::Sampler, types::GenFloat};
use cgmath::{prelude::*, Vector3};

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

/// Mirror a vector about a unit direction
///
/// `vector` is the incoming vector, and `normal` is the vector to mirror `vector` around. Returns
/// a mirrored vector. Note that `normal` must be a unit vector.
pub fn mirror<T: GenFloat>(vector: &Vector3<T>, normal: &Vector3<T>) -> Vector3<T> {
    vector - (normal * T::from(2).unwrap() * vector.dot(*normal))
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
