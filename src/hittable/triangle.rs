//! An implementation of the triangle primitive hittable
//!
//! The triangle is probably one of the most widely used geometric primitives in three dimensional
//! rendering and modeling, as most OBJ files are defined in terms of triangles.

use crate::{
    hittable::{HitRecord, Hittable},
    types::{eta, GenFloat},
};
use cgmath::{InnerSpace, Vector3};
use serde::{Deserialize, Serialize};

/// The "handedness" of the coordinate system used to define the triangle
///
/// The way we compute the normal of a triangle depends on the direction of the points used to
/// define the triangle.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TriangleHandedness {
    Clockwise,
    CounterClockwise,
}

/// A geometric triangle
///
/// These are the parameters for a triangle that may be input by a user. The initialization method
/// will convert it into the `Triangle` struct, which can be used by the renderer at runtime.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriangleParameters<T: GenFloat> {
    /// The coordinates defining the bounds of the triangle in real-world space
    pub vertices: [Vector3<T>; 3],

    /// The direction in which the vertices of the triangle are evaluated to compute the normal
    /// vector
    ///
    /// The points can be evaluated in either a clockwise or counterclockwise fashion to generate
    /// the normal vector. The direction will affect the orientation of the normal.
    ///
    /// The convention we use is to use the counterclockwise points, so serde will deserialize this
    #[serde(default = "default_handedness")]
    pub handedness: TriangleHandedness,
}

/// A helper method for serde to infer the default handedness of a triangle
///
/// We have decided that the convention for this renderer will be to use counterclockwise vertices
fn default_handedness() -> TriangleHandedness {
    TriangleHandedness::CounterClockwise
}

impl<T: GenFloat> Default for TriangleParameters<T> {
    /// The default implementation of a triangle defines the default handedness of the vertices
    ///
    /// If you use this method, you will need to define the vertices on your own, since this
    /// defines all of the vertices as `[0, 0, 0]`, which does not yield a valid triangle.
    ///
    /// Proper usage for this method:
    ///
    /// ```
    /// # use super::*;
    /// let triangle = TriangleParameters {
    ///     vertices: [
    ///         Vector3::new(1, 2, 1),
    ///         Vector3::new(2, 1, 1),
    ///         Vector3::new(4, 5, 2),
    ///     ],
    ///     ..default(),
    /// };
    /// ```
    fn default() -> Self {
        let zero = T::from(0).unwrap();
        let zeroes = Vector3::new(zero, zero, zero);
        TriangleParameters {
            vertices: [zeroes, zeroes, zeroes],
            handedness: TriangleHandedness::CounterClockwise,
        }
    }
}

impl<T: GenFloat> TriangleParameters<T> {
    /// Initialize a `Triangle` from its parameters
    ///
    /// This will compute the normal vector by getting two sides of the triangle and computing the
    /// cross product of the two vectors.
    pub fn init(self) -> Triangle<T> {
        let a = self.vertices[2] - self.vertices[0];
        let b = self.vertices[2] - self.vertices[0];
        let normal = match self.handedness {
            TriangleHandedness::Clockwise => a.cross(b).normalize(),
            TriangleHandedness::CounterClockwise => b.cross(a).normalize(),
        };
        Triangle {
            vertices: self.vertices,
            edges: [a, b],
            normal,
        }
    }
}

/// A geometric triangle
///
/// This is the triangle struct with cached computation information that can be used at runtime. We
/// compute the ray-triangle intersection using the Moller-Trumbore algorithm, which you can read
/// about here:
/// http://webserver2.tecgraf.puc-rio.br/~mgattass/cg/trbRR/Fast%20MinimumStorage%20RayTriangle%20Intersection.pdf
#[derive(Debug, Serialize, Deserialize)]
pub struct Triangle<T: GenFloat> {
    /// The coordinates defining the bounds of the triangle relative to the origin
    ///
    /// These are the "real-world" coordinates of the vertices
    pub vertices: [Vector3<T>; 3],

    /// The edges of the triangle
    ///
    /// This is precomputed for the intersection calculation, so we don't have to repeat it for
    /// every collision.
    pub edges: [Vector3<T>; 2],

    /// The normal vector relative to the supporting plane of the triangle
    ///
    /// Any collision of the triangle will yield the same normal, since the triangle lies on a
    /// normal plane. We can precompute this and avoid wasting CPU cycles on every collision.
    pub normal: Vector3<T>,
}

impl<T: GenFloat> Hittable<T> for Triangle<T> {
    /// An implementation of the Moller-Trumbore algorithm for ray-triangle intersection detection
    ///
    /// This is an implementation of the [Moller-Trumbore algorithm]
    /// (http://webserver2.tecgraf.puc-rio.br/~mgattass/cg/trbRR/Fast%20MinimumStorage%20RayTriangle%20Intersection.pdf).
    fn hit(&self, ray: &crate::ray::Ray<T>) -> Option<HitRecord<T>> {
        // begin calculating the determinant
        let p = ray.direction.cross(self.edges[1]);
        let determinant = self.edges[0].dot(p);

        // This means that the ray and the plane that the triangle lies on are parallel. We exit
        // early because we know that there's no possible intersection, and also to avoid a
        // division by zero error.
        if determinant < eta() {
            return None;
        }

        // Distance from vertex[0] to the ray's origin
        let t = ray.origin - self.vertices[0];

        // Get u, the first barycentric coordinate
        let u = t.dot(p);

        // Short circuit if u isn't within the bounds of the triangle
        if u < T::from(0).unwrap() || u > determinant {
            return None;
        }
        let q = t.cross(self.edges[0]);
        let v = ray.direction.dot(q);

        // Check it the barycentric coordinates are outside of the bounds of the triangle
        if v < T::from(0).unwrap() || u + v > determinant {
            return None;
        }

        // Now we know the ray intersects the triangle, and we can calculate `t`,
        let inverse_determinant = T::from(1).unwrap() / determinant;
        let distance = self.edges[1].dot(q) * inverse_determinant;

        // Scale the barycentric coordinates
        let u = u * inverse_determinant;
        let v = v * inverse_determinant;
        let w = T::from(1).unwrap() - u - v;

        // Convert the barycentric coordinates to a real world coordinate
        let intersection_point =
            (self.vertices[0] * u) + (self.vertices[1] * v) + (self.vertices[2] * w);
        Some(HitRecord {
            p: intersection_point,
            normal: self.normal,
            distance,
        })
    }
}
