//! The "list" acceleration structure for computing intersections.

use crate::hittable::HitRecord;
use crate::math::vec3_dist;
use crate::types::Ray;
use crate::{hittable::Hittable, types::GenFloat};

/// A naive list "acceleration structure" for computing ray intersections in a scene
///
/// This isn't an actual acceleration structure, and it's very slow. This is a very basic
/// structure so we can have a functioning renderer without needing to implement more complex data
/// structures off the bat. To compute the intersection, this will traverse every object in the
/// scene and check whether the object was hit. This will return the intersection point that is
/// closest to the origin point of the ray.
pub struct ObjectList<'a, T: GenFloat> {
    /// A list of every object in the scene
    objects: Vec<&'a dyn Hittable<NumType = T>>,
}

impl<'a, T: GenFloat> Hittable for ObjectList<'a, T> {
    type NumType = T;
    fn hit(&self, ray: &Ray<Self::NumType>) -> Option<HitRecord<Self::NumType>> {
        // Collect every object that was hit so we can sort them out and find the closest
        // intersection to the origin point of the ray after every object has been traversed.
        let mut intersections: Vec<HitRecord<Self::NumType>> =
            self.objects.iter().filter_map(|obj| obj.hit(ray)).collect();

        // If there are no intersections, there is no hit. If there's only one, it is by definition
        // the closest intersection and we can return that directly. Otherwise, we will need to
        // find the closest intersection to the incoming ray.
        if intersections.is_empty() {
            return None;
        } else if intersections.len() == 1 {
            return Some(intersections[0]);
        }
        intersections.sort_by(|a, b| {
            let a_dist: T = vec3_dist(&ray.origin, &a.p);
            let b_dist: T = vec3_dist(&ray.origin, &b.p);
            a_dist.cmp(&b_dist)
        });
        Some(intersections[0])
    }
}
