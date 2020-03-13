//! A module defining an interface for objects in a scene that a ray can intersect with
//!
//! This module defines interfaces for all of the geometry in the renderer (which are all things
//! that a ray can intersect with), as well as interfaces for "infinite" objects that lie outside
//! the scene (like skyboxes and ambient lighting), as well as interfaces for acceleration
//! structures so that they can be used generically. It doesn't really matter, as long as you can
//! yield which object was hit.

use crate::{material::BSDF, ray::Ray, types::GenFloat};
use cgmath::Vector3;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod bounding_box;
mod sphere;

pub use sphere::Sphere;

/// An interface for any object that can intersect with a ray coming from the camera
///
/// NOTE: This method can be used with entire acceleration structures or individual geometric
/// objects. It doesn't matter, as long as you have some way to resolve which object was hit by an
/// outgoing ray.
#[enum_dispatch(SerializedHittable)]
pub trait Hittable<T: GenFloat>: Debug + Send + Sync {
    /// A method that returns a hit record if the object was hit
    fn hit(&self, ray: &Ray<T>) -> Option<HitRecord<T>>;
}

/// An interface for an object that is used as part of an acceleration structure
///
/// This method is automatically implemented for any struct that implements `Hittable`. This
/// differs from the `Hittable` trait in that these objects simply need to report whether they were
/// hit or not, not the particular point where the ray intersects the object.
pub trait AccelHittable<T: GenFloat>: Debug + Send + Sync {
    fn hit(&self, ray: &Ray<T>, inverse_dir: Vector3<T>) -> bool;
}

impl<T: GenFloat> AccelHittable<T> for dyn Hittable<T> {
    /// A blanket implementation of the `AccelHittable` trait for `Hittable` objects. It returns
    /// whether there was some hit record returned. Otherwise it will return `false`.
    fn hit(&self, ray: &Ray<T>, inverse_dir: Vector3<T>) -> bool {
        self.hit(ray).is_some()
    }
}

/// The different types of `Hittable` types that can be used as input objects
///
/// This is an enum type that exists for convenient use with serde, so we can create a serializable
/// struct to expose as a scene description to the user.
#[enum_dispatch]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SerializedHittable<T: GenFloat> {
    Sphere(Sphere<T>),
}

/// Information pertaining to a ray intersection
///
/// The hit record has information on where the object was hit and the normal for that hit. This is
/// the record struct specifically for geometric collisions.
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct HitRecord<T: GenFloat> {
    /// The point in space where the object was hit
    pub p: Vector3<T>,

    /// The normal vector for the intersection
    pub normal: Vector3<T>,

    /// The distance from the origin ray to the point of collision
    pub distance: T,
}

/// The struct for some object in the scene that can be intersected geometrically that also
/// provides a BSDF function for texture.
///
/// This struct pairs together geometry primitives and their corresponding BSDF functions so that
/// we can calculate the color value for a light bounce, and determine which direction the ray
/// should go next.
#[derive(Debug)]
pub struct Textured<'a, T: GenFloat> {
    /// The geometric primitive that might be hit by the light ray or path
    pub geometry: Box<dyn Hittable<T> + 'a>,

    /// A reference to the BSDF method for
    pub mat: Box<dyn BSDF<T> + 'a>,
}
