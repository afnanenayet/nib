//! A module defining an interface for objects in a scene that a ray can intersect with
//!
//! This module defines interfaces for all of the geometry in the renderer (which are all things
//! that a ray can intersect with), as well as interfaces for "infinite" objects that lie outside
//! the scene (like skyboxes and ambient lighting), as well as interfaces for acceleration
//! structures so that they can be used generically.

use cgmath::Vector3;

/// An interface for any object that can intersect with a ray coming from the camera
///
/// NOTE: This method can be used with entire acceleration structures or individual hittable
/// objects. It doesn't matter, as long as you have some way to resolve which object was hit by an
/// outgoing ray.
pub trait Hittable {
    /// Having an associated type allows for different numeric types to be easily swapped in an
    /// out. This defines the numeric type of the ray and consequently, the hit record.
    type NumType: Sync;

    /// A method that returns a hit record if the object was hit
    fn hit(&self, ray: &Ray<Self::Numtype>) -> Option<HitRecord<Self::NumType>>;
}

/// Information pertaining to a ray intersection
///
/// The hit record has information on where the object was hit and the normal for that hit
pub struct HitRecord<T> {
    /// The point in space where the object was hit
    p: Vector3<T>,

    /// The normal vector for the intersection
    normal: Vector3<T>,
}
