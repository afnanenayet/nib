//! A module defining an interface for objects in a scene that a ray can intersect with
//!
//! This module defines interfaces for all of the geometry in the renderer (which are all things
//! that a ray can intersect with), as well as interfaces for "infinite" objects that lie outside
//! the scene (like skyboxes and ambient lighting), as well as interfaces for acceleration
//! structures so that they can be used generically. It doesn't really matter, as long as you can
//! yield which object was hit.

use crate::{
    material::BSDF,
    types::{GenFloat, GenReal, Ray},
};
use cgmath::Vector3;
use rand::Rng;
use std::fmt::Debug;

mod sphere;

pub use sphere::Sphere;

/// An interface for any object that can intersect with a ray coming from the camera
///
/// NOTE: This method can be used with entire acceleration structures or individual geometric
/// objects. It doesn't matter, as long as you have some way to resolve which object was hit by an
/// outgoing ray.
pub trait Hittable<T: GenFloat>: Debug {
    /// A method that returns a hit record if the object was hit
    fn hit(&self, ray: &Ray<T>) -> Option<HitRecord<T>>;
}

/// Information pertaining to a ray intersection
///
/// The hit record has information on where the object was hit and the normal for that hit
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct HitRecord<T: GenReal> {
    /// The point in space where the object was hit
    pub p: Vector3<T>,

    /// The normal vector for the intersection
    pub normal: Vector3<T>,
}

/// The struct for some object in the scene that can be intersected geometrically that also
/// provides a BSDF function for texture.
///
/// This struct pairs together geometry primitives and their corresponding BSDF functions so that
/// we can calculate the color value for a light bounce, and determine which direction the ray
/// should go next.
#[derive(Debug)]
pub struct Textured<T: GenFloat, R: Rng + ?Sized> {
    /// The geometric primitive that might be hit by the light ray or path
    pub geometry: Box<dyn Hittable<T>>,

    /// A reference to the BSDF method for
    pub mat: Box<dyn BSDF<T, R>>,
}
