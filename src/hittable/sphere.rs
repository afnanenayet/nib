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
        let discriminant = (b * b) - c;

        if discriminant < T::from(0).unwrap() {
            return None;
        }
        let t = -b - discriminant.sqrt();
        let p = ray.origin + (ray.direction * t);
        let normal = ((p - self.center) / self.radius).normalize();
        Some(HitRecord {
            distance: t,
            p,
            normal,
        })
    }
}
