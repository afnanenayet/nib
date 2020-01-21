//! An implementation of the sphere primitive

use crate::{
    hittable::{HitRecord, Hittable},
    types::GenFloat,
    types::Ray,
};
use cgmath::{prelude::*, Vector3};
use serde::{Deserialize, Serialize};

/// A sphere primitive
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sphere<T: GenFloat> {
    /// The center of the sphere in spatial coordinates
    pub center: Vector3<T>,

    /// The radius of the sphere
    pub radius: T,
}

impl<T: GenFloat> Hittable<T> for Sphere<T> {
    fn hit(&self, ray: &Ray<T>) -> Option<HitRecord<T>> {
        let oc = ray.origin - self.center;
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - (self.radius * self.radius);

        if c > T::from(0).unwrap() && b > T::from(0).unwrap() {
            return None;
        }

        let discriminant = b * b - c;

        // A negative discriminant means the ray missed the sphere
        if discriminant < T::from(0).unwrap() {
            return None;
        }

        // Now we have established the ray hit the sphere. Calculate the smallest t-value of the
        // intersection (aka find the closest hit point, since we can intersect a sphere at two
        // different points).
        let mut t = (T::from(-1).unwrap() * b) - (discriminant.sqrt());

        // If t is less than 0, it means the ray started inside of the sphere, so we just clamp it
        // to 0
        if t < T::from(0).unwrap() {
            t = T::from(0).unwrap();
        }
        let p = ray.origin + (ray.direction * t);
        let normal = (p - self.center).normalize();
        Some(HitRecord { p, normal })
    }
}
