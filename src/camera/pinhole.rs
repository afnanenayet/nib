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
    pub origin: Vector3<T>,
    pub horizontal: Vector3<T>,
    pub vertical: Vector3<T>,
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
