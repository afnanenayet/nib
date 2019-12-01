//! An implementation of a basic pinhole camera

use crate::{
    camera::Camera,
    types::{GenFloat, Ray},
};
use cgmath::Vector3;

/// The classic pinhole camera
///
/// No bells, no whistles, just projected rays.
#[derive(Debug)]
pub struct Pinhole<T: GenFloat> {
    /// The origin point of the camera's field of view
    pub origin: Vector3<T>,
    /// The horizontal span of the camera's field of view
    pub horizontal: Vector3<T>,
    /// The vertical span of the camera's field of view
    pub vertical: Vector3<T>,
    /// The lower left corner of the camera's field of view
    pub lower_left: Vector3<T>,
}

impl<T: GenFloat> Camera<T> for Pinhole<T> {
    fn to_ray(&self, u: T, v: T) -> Ray<T> {
        Ray {
            origin: self.origin,
            direction: self.lower_left + (self.horizontal * u) + (self.vertical * v) - self.origin,
        }
    }
}

impl<T: GenFloat> Default for Pinhole<T> {
    /// Return the standard camera parameters as defined in page 20 of "Ray Tracing in One Weekend"
    fn default() -> Self {
        // So we don't have to type this repeatedly
        let zero = T::from(0).unwrap();
        Self {
            origin: Vector3::new(zero, zero, zero),
            horizontal: Vector3::new(T::from(4).unwrap(), zero, zero),
            vertical: Vector3::new(zero, T::from(2).unwrap(), zero),
            lower_left: Vector3::new(
                T::from(-2.0).unwrap(),
                T::from(-1.0).unwrap(),
                T::from(-1.0).unwrap(),
            ),
        }
    }
}