//! An implementation of the triangle primitive hittable
//!
//! The triangle is probably one of the most widely used geometric primitives in three dimensional
//! rendering and modeling, as most OBJ files are defined in terms of triangles.

use crate::types::GenFloat;
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

/// A geometric triangle
///
/// These are the parameters for a triangle that may be input by a user. The initialization method
/// will convert it into the `Triangle` struct, which can be used by the renderer at runtime.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriangleParameters<T: GenFloat> {
    /// The center of the triangle, expressed in real-world coordinates
    pub origin: Vector3<T>,

    /// The coordinates defining the bounds of the triangle relative to the origin (barycentric
    /// coordinates)
    pub vertices: [Vector3<T>; 3],
}

impl<T: GenFloat> TriangleParameters<T> {
    /// Initialize a `Triangle` from its parameters
    ///
    /// This will compute the normal vector by getting two sides of the triangle and computing the
    /// cross product of the two vectors.
    pub fn init(self) -> Triangle<T> {
        let a = self.vertices[2] - self.vertices[0];
        let b = self.vertices[2] - self.vertices[0];
        let normal = a.cross(b);
        Triangle {
            origin: self.origin,
            vertices: self.vertices,
            normal,
        }
    }
}

/// A geometric triangle
///
/// This is the triangle struct with cached computation information that can be used at runtime
#[derive(Debug, Serialize, Deserialize)]
pub struct Triangle<T: GenFloat> {
    /// The center of the triangle, expressed in real-world coordinates
    pub origin: Vector3<T>,

    /// The normal of the triangle
    ///
    /// The normal of the triangle is computed as a cross product of AB and BC, which computes the
    /// normal of the plane that the triangle lies on, which is identical to the normal of the
    /// triangle.
    pub normal: Vector3<T>,

    /// The coordinates defining the bounds of the triangle relative to the origin
    pub vertices: [Vector3<T>; 3],
}
