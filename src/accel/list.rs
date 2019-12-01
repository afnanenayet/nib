//! The "list" acceleration structure for computing intersections.

use super::{AccelError, AccelResult};
use crate::hittable::HitRecord;
use crate::types::Ray;
use crate::{hittable::Hittable, types::GenFloat};
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
    objects: Vec<&'a dyn Hittable<T>>,
}

impl<'a, T: GenFloat> ObjectList<'a, T> {
    pub fn new(objects: Vec<&'a dyn Hittable<T>>) -> AccelResult<Self> {
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

        // If there are no intersections, there is no hit. If there's only one, it is by definition
        // the closest intersection and we can return that directly without wasting extra CPU
        // cycles. Otherwise, we will need to find the closest intersection to the incoming ray.
        if intersections.is_empty() {
            return None;
        } else if intersections.len() == 1 {
            return Some(intersections[0]);
        }
        intersections.sort_by(|&a, &b| {
            let a_dist: T = ray.origin.distance(a.p);
            let b_dist: T = ray.origin.distance(b.p);
            a_dist.partial_cmp(&b_dist).unwrap_or(Equal)
        });
        Some(intersections[0])
    }
}
