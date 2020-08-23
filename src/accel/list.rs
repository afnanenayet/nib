//! The "list" acceleration structure for computing intersections.

use crate::{
    accel::{Accel, AccelRecord, AccelResult},
    hittable::Textured,
    ray::Ray,
    renderer::Arena,
    types::{eta, GenFloat},
};
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
    objects: Arena<'a, T>,
}

impl<'a, T: GenFloat> ObjectList<'a, T> {
    pub fn new(objects: Arena<'a, T>) -> AccelResult<Self> {
        Ok(ObjectList { objects })
    }
}

impl<'a, T: GenFloat> Accel<T> for ObjectList<'a, T> {
    fn collision(&self, ray: &Ray<T>) -> Option<AccelRecord<T>> {
        // Collect every object that was hit so we can sort them out and find the closest
        // intersection to the origin point of the ray after every object has been traversed. We
        // also filter out any collisions that are less than the margin of error.
        let mut intersections: Vec<AccelRecord<T>> = self
            .objects
            .iter()
            .filter_map(|obj| {
                if let Some(hit_record) = obj.geometry.hit(ray) {
                    Some(AccelRecord {
                        object: obj,
                        hit_record,
                    })
                } else {
                    None
                }
            })
            .filter(|x| x.hit_record.distance >= eta())
            .collect();

        // If the list is empty, then the sort method will be a no-op. We don't need to preserve
        // the order of elements, so we can use the fast unstable sort.
        intersections.sort_unstable_by(|a, b| {
            let a_dist: T = a.hit_record.distance;
            let b_dist: T = b.hit_record.distance;
            // We treat NaN values as equal. If we hit NaNs by this point the entire list is likely
            // useless anyway and there are other issues that have propagated to this point.
            a_dist.partial_cmp(&b_dist).unwrap_or(Equal)
        });
        // Convert `Option<&AccelRecord>` to `Option<AccelRecord>`
        intersections.first().map(|&x| x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        hittable::{HitRecord, Sphere},
        material::Mirror,
    };
    use cgmath::Vector3;
    use std::sync::Arc;

    // A convenience method to help create an ObjectList of references
    fn create_list<'a>(objects: Vec<Sphere<f32>>) -> ObjectList<'a, f32> {
        let box_objects = objects
            .into_iter()
            .map(|geom| Textured {
                geometry: Box::new(geom),
                mat: Box::new(Mirror::default()),
            })
            .collect();
        ObjectList::new(Arc::new(box_objects)).unwrap()
    }

    // Basic case where there are no objects, so we expect no collision
    #[test]
    fn no_objects() {
        let list = create_list(vec![]);
        let ray = Ray::<f32>::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        assert!(list.collision(&ray).is_none());
    }

    // The ray misses all of the objects in the acceleration structure, we expect no collision
    #[test]
    fn no_collisions() {
        let list = create_list(vec![
            Sphere {
                center: Vector3::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            Sphere {
                center: Vector3::new(0.0, 5.0, 0.0),
                radius: 1.0,
            },
            Sphere {
                center: Vector3::new(0.0, -5.0, 0.0),
                radius: 1.0,
            },
        ]);
        let ray = Ray::<f32>::new(Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 0.0, -1.0));
        assert!(list.collision(&ray).is_none());
    }

    // This is testing that a collision can be detected at all, with the ray intersecting only one
    // geometric primitive
    #[test]
    fn one_possible_collision() {
        let list = create_list(vec![Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }]);
        let ray = Ray::<f32>::new(Vector3::new(0.0, -2.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        assert!(list.collision(&ray).is_some());
    }

    // Testing a ray that intersects with multiple spheres. This should return the closest hit.
    #[test]
    fn multiple_collisions() {
        let list = create_list(vec![
            Sphere {
                center: Vector3::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            Sphere {
                center: Vector3::new(0.0, 2.0, 0.0),
                radius: 1.0,
            },
            Sphere {
                center: Vector3::new(-5.0, -5.0, -5.0),
                radius: 2.0,
            },
            Sphere {
                center: Vector3::new(5.0, 5.0, 5.0),
                radius: 2.0,
            },
        ]);
        let ray = Ray::<f32>::new(Vector3::new(0.0, -2.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let expected = HitRecord {
            p: Vector3::new(0.0, -1.0, 0.0),
            distance: 1.0,
            normal: Vector3::new(0.0, -1.0, 0.0),
        };
        assert_eq!(list.collision(&ray).unwrap().hit_record, expected);
    }
}
