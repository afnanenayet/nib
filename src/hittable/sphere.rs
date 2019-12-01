//! An implementation of the sphere primitive

use crate::hittable::HitRecord;
use crate::types::Ray;
use crate::{hittable::Hittable, types::GenFloat};
use cgmath::{prelude::*, Vector3};

/// A sphere primitive
#[derive(Debug)]
pub struct Sphere<T: GenFloat> {
    /// The center of the sphere in spatial coordinates
    pub center: Vector3<T>,

    /// The radius of the sphere
    pub radius: T,
}

impl<T: GenFloat> Hittable<T> for Sphere<T> {
    fn hit(&self, ray: &Ray<T>) -> Option<HitRecord<T>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = T::from(2).unwrap() * oc.dot(ray.direction);
        let c = oc.dot(oc) - (self.radius * self.radius);

        // If c <= 0 then the ray missed the sphere, so we don't have to calculate the intersection
        // point and normal.
        if c <= T::from(0).unwrap() {
            return None;
        }
        let discriminant = (b * b) - (T::from(4).unwrap() * a * c);
        // The distance from the origin of the ray where the intersection happened
        let isect_dist =
            (T::from(-1).unwrap() * b) - discriminant.sqrt() / (T::from(2).unwrap() * a);
        let isect_point = ray.origin + (ray.direction * isect_dist);
        let normal = (isect_point - self.center).normalize();
        Some(HitRecord {
            p: isect_point,
            normal,
        })
    }
}
