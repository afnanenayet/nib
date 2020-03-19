//! Implementations of box primitives
//!
//! This module provides implementations of the `Hittable` interface for axis aligned bounding
//! boxes (AABB), and oriented bounding boxes (OBB).

use crate::{
    hittable::AccelHittable,
    ray::Ray,
    types::{Dimension, GenFloat},
};
use cgmath::Vector3;
use num_traits::{Float, ToPrimitive};
use serde::{Deserialize, Serialize};

/// The parameters for an axis aligned bounding box (AABB)
///
/// An AABB is defined by two points which represent the minimum and maximum extants of the box.
#[derive(Debug, Serialize, Deserialize)]
struct AxisAlignedBoundingBox<T: GenFloat> {
    /// The minimum extant of the bounding box
    pub min: Vector3<T>,

    /// The maximum extant of the bounding box
    pub max: Vector3<T>,
}

impl<T: GenFloat> AccelHittable<T> for AxisAlignedBoundingBox<T> {
    /// Compute whether a ray intersects with an axis aligned bounding box
    ///
    /// Note that this method will *not* count rays that originate from inside the box as valid
    /// intersections.
    fn hit(&self, ray: &Ray<T>, inverse_dir: Vector3<T>) -> bool {
        let mut t_min: T = Float::neg_infinity();
        let mut t_max: T = Float::infinity();

        // This can be done functionally but it's more efficient to calculate this in one pass
        for d in Dimension::iterator() {
            let index = d.to_usize().unwrap();

            // This corresponds to the line equation:
            // t = d_0 - O_x / D_x (w.r.t. for the dimension). Of course, we are using the inverse
            // direction here, which is why we multiply by \frac{1}{D_x}. We multiply by the
            // inverse because it's more efficient.
            let t0 = (self.min[index] - ray.origin[index]) * inverse_dir[index];
            let t1 = (self.max[index] - ray.origin[index]) * inverse_dir[index];

            // We want to get the min and the max from what we just computed so we can shrink the
            // bounds that we're comparing to determine whether the ray falls in the bounding box.
            let cand_t_min = t0.min(t1);
            let cand_t_max = t0.max(t1);
            t_min = t_min.max(cand_t_min);
            t_max = t_max.min(cand_t_max);
        }
        // we use the t_min > 0 check to negate any intersections that originate from inside of the
        // box
        t_max > t_min && t_min > T::from(0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// We point the ray away from the AABB and expect no intersections
    #[test]
    fn aabb_ray_no_intersection() {
        // basic case where the ray originates outside of the box and points away
        let aabb = AxisAlignedBoundingBox {
            min: Vector3::new(0.0, 0.0, 0.0),
            max: Vector3::new(1.0, 1.0, 1.0),
        };
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, -1.0),
            direction: Vector3::new(-1.0, -1.0, -1.0),
        };
        assert!(!aabb.hit(&ray, ray.inverse_dir()));

        let aabb = AxisAlignedBoundingBox {
            min: Vector3::new(0.0, 0.0, 0.0),
            max: Vector3::new(1.0, 1.0, 1.0),
        };
        let ray = Ray {
            origin: Vector3::new(5.0, 5.0, 5.0),
            direction: Vector3::new(1.0, 1.0, 1.0),
        };
        assert!(!aabb.hit(&ray, ray.inverse_dir()));

        // the ray originates from inside of the box
        let aabb = AxisAlignedBoundingBox {
            min: Vector3::new(0.0, 0.0, 0.0),
            max: Vector3::new(1.0, 1.0, 1.0),
        };
        let ray = Ray {
            origin: Vector3::new(0.5, 0.5, 0.5),
            direction: Vector3::new(-1.0, 0.0, 1.0),
        };
        assert!(!aabb.hit(&ray, ray.inverse_dir()));
    }

    /// We point the ray at the AABB and expect intersections
    #[test]
    fn aabb_ray_intersections() {
        let aabb = AxisAlignedBoundingBox {
            min: Vector3::new(0.0, 0.0, 0.0),
            max: Vector3::new(1.0, 1.0, 1.0),
        };
        let ray = Ray {
            origin: Vector3::new(0.5, 0.5, -1.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        assert!(aabb.hit(&ray, ray.inverse_dir()));

        let aabb = AxisAlignedBoundingBox {
            min: Vector3::new(0.0, 0.0, 0.0),
            max: Vector3::new(-1.0, -1.0, -1.0),
        };
        let ray = Ray {
            origin: Vector3::new(-0.5, -0.5, -2.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        assert!(aabb.hit(&ray, ray.inverse_dir()));
    }
}
