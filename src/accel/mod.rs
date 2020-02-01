//! A module defining interfaces for an acceleration structure
//!
//! This module provides the generic interface for acceleration structures as well as
//! implementations of various acceleration structures.

mod list;

pub use list::ObjectList;

use crate::{
    hittable::{HitRecord, Hittable, Textured},
    material::BSDF,
    ray::Ray,
    types::GenFloat,
};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
/// An error associated with acceleration structures
pub enum AccelError {
    #[error("There must be at least one object passed to the constructor")]
    NoObjects,
}

/// A result that can return an `AccelError`
pub type AccelResult<T> = Result<T, AccelError>;

/// The result of a query to an acceleration structure.
///
/// An acceleration structure computes a geometric intersection between some path/ray and an
/// object. This requires both the information about the geometric primitive and the physical
/// collision, as well as a reference to the actual object and the associated BSDF/material
/// function.
#[derive(Debug, Clone, Copy)]
pub struct AccelRecord<'a, T: GenFloat> {
    /// The details of the collision
    pub hit_record: HitRecord<T>,

    /// A reference to the textured object (a geometric primitive and a BSDF pairing)
    pub object: &'a Textured<'a, T>,
}

/// A reference to an object that consists of a geometric form and a texture
#[derive(Debug, Clone, Copy)]
pub struct TexturedRef<'a, T: GenFloat> {
    /// The geometric primitive that might be hit by the light ray or path
    pub geometry: &'a dyn Hittable<T>,

    /// A reference to the BSDF method for
    pub mat: &'a dyn BSDF<T>,
}

/// The `Accel` trait is a generic trait for acceleration structures in the renderer.
///
/// It provides a simple interface which allows the caller to determine if an incoming ray collided
/// with some object in the scene.
pub trait Accel<T: GenFloat>: Debug + Send + Sync {
    /// Return whether the incoming ray collided with any of the objects in the scene
    fn collision(&self, ray: &Ray<T>) -> Option<AccelRecord<T>>;
}
