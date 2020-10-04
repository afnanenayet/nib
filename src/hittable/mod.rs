//! A module defining an interface for objects in a scene that a ray can intersect with
//!
//! This module defines interfaces for all of the geometry in the renderer (which are all things
//! that a ray can intersect with), as well as interfaces for "infinite" objects that lie outside
//! the scene (like skyboxes and ambient lighting), as well as interfaces for acceleration
//! structures so that they can be used generically. It doesn't really matter, as long as you can
//! yield which object was hit.

use crate::{
    material::{SerializedMaterial, BSDF},
    ray::Ray,
    types::{approx_eq_vec, Float},
};
use cgmath::Vector3;
use float_cmp::approx_eq;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod sphere;
mod triangle;

pub use sphere::Sphere;
pub use triangle::Triangle;

/// An interface for any object that can intersect with a ray coming from the camera
///
/// NOTE: This method can be used with entire acceleration structures or individual geometric
/// objects. It doesn't matter, as long as you have some way to resolve which object was hit by an
/// outgoing ray.
pub trait Hittable: Debug + Send + Sync {
    /// A method that returns a hit record if the object was hit
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

/// The different types of `Hittable` types that can be used as input objects
///
/// This is an enum type that exists for convenient use with serde, so we can create a serializable
/// struct to expose as a scene description to the user.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SerializedHittable {
    Sphere(Sphere),
    Triangle(triangle::TriangleParameters),
}

/// Information pertaining to a ray intersection
///
/// The hit record has information on where the object was hit and the normal for that hit. This is
/// the record struct specifically for geometric collisions.
#[derive(Clone, Debug, Copy)]
pub struct HitRecord {
    /// The point in space where the object was hit
    pub p: Vector3<Float>,

    /// The normal vector for the intersection
    pub normal: Vector3<Float>,

    /// The distance from the origin ray to the point of collision
    pub distance: Float,
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        approx_eq_vec(&self.p, &other.p)
            && approx_eq_vec(&self.normal, &other.normal)
            && approx_eq!(Float, self.distance, other.distance)
    }
}

impl Eq for HitRecord {}

/// The struct for some object in the scene that can be intersected geometrically that also
/// provides a BSDF function for texture.
///
/// This struct pairs together geometry primitives and their corresponding BSDF functions so that
/// we can calculate the color value for a light bounce, and determine which direction the ray
/// should go next.
#[derive(Debug)]
pub struct Textured<'a> {
    /// The geometric primitive that might be hit by the light ray or path
    pub geometry: Box<dyn Hittable + 'a>,

    /// A reference to the BSDF function that corresponds to the geometry
    pub mat: Box<dyn BSDF + 'a>,
}

/// A serializable wrapper for the
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SerializedTextured {
    /// The geometric primitive that might be hit by the light ray or path
    pub geometry: SerializedHittable,

    /// A reference to the BSDF method for
    pub mat: SerializedMaterial,
}

impl<'a> From<SerializedTextured> for Textured<'a> {
    fn from(serialized: SerializedTextured) -> Self {
        let geometry: Box<dyn Hittable + 'a> = match serialized.geometry {
            SerializedHittable::Sphere(x) => Box::new(x.clone()),
            SerializedHittable::Triangle(x) => Box::new(x.init()),
        };
        let bsdf: Box<dyn BSDF + 'a> = match serialized.mat {
            SerializedMaterial::Mirror(x) => Box::new(x.clone()),
            SerializedMaterial::Diffuse(x) => Box::new(x.clone()),
            SerializedMaterial::Dielectric(x) => Box::new(x.clone()),
        };
        Textured {
            geometry,
            mat: bsdf,
        }
    }
}
