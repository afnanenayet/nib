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
    fn hit(&self, ray: &Ray<T>, inverse_dir: Vector3<T>) -> bool {
        let mut t_min: T = Float::neg_infinity();
        let mut t_max: T = Float::infinity();
        let dim = Dimension::X; // the starting dimension to iterate through

        for d in dim {
            let index = d.to_usize().unwrap();
            let t1 = (self.min[index] - ray.origin[index]) * inverse_dir[index];
            let t2 = (self.max[index] - ray.origin[index]) * inverse_dir[index];
            t_min = t_min.max(t1.min(t2));
            t_max = t_min.min(t1.max(t2));
        }
        t_max > t_min.max(T::from(0).unwrap())
    }
}
