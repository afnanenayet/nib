//! The "list" acceleration structure for computing intersections.

use crate::{
    accel::{AccelError, AccelResult},
    hittable::{HitRecord, Hittable},
    types::{GenFloat, Ray},
};
use cgmath::prelude::*;
use std::cmp::Ordering::Equal;

/// A naive list "acceleration structure" for computing ray intersections in a scene
///
/// This isn't an actual acceleration structure, and it's very slow. This is a very basic
/// structure so we can have a functioning renderer without needing to implement more complex data
/// structures off the bat. To compute the intersection, this will traverse every object in the
/// scene and check whether the object was hit. This will return the intersection point that is
/// closest to the origin point of the ray.
#[derive(Debug)]
pub struct ObjectList<'a, T: GenFloat> {
    /// A list of every object in the scene
    objects: Vec<Box<dyn Hittable<T> + 'a>>,
}

impl<'a, T: GenFloat> ObjectList<'a, T> {
    pub fn new(objects: Vec<Box<dyn Hittable<T> + 'a>>) -> AccelResult<Self> {
        if objects.is_empty() {
            return Err(AccelError::NoObjects);
        }
        Ok(ObjectList { objects })
    }
}

impl<'a, T: GenFloat> Hittable<T> for ObjectList<'a, T> {
    fn hit(&self, ray: &Ray<T>) -> Option<HitRecord<T>> {
        // Collect every object that was hit so we can sort them out and find the closest
        // intersection to the origin point of the ray after every object has been traversed.
        let mut intersections: Vec<HitRecord<T>> =
            self.objects.iter().filter_map(|obj| obj.hit(ray)).collect();

        // If the list is empty, then the sort method will be a no-op

        // we don't need to preserve the order of elements, so we can use the fast unstable sort
        intersections.sort_unstable_by(|&a, &b| {
            let a_dist: T = ray.origin.distance(a.p);
            let b_dist: T = ray.origin.distance(b.p);
            // We treat NaN values as equal. If we hit NaNs by this point the entire list is likely
            // useless anyway and there are other issues that have propagated to this point.
            a_dist.partial_cmp(&b_dist).unwrap_or(Equal)
        });
        // Option<&HitRecord> -> Option<HitRecord>
        intersections.first().map(|x| *x)
    }
}
