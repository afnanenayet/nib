//! A module providing definitions and methods for a scene. A scene contains most of the relevant
//! information for an integrator, and it provides information such as lights, geometric objects,
//! camera information, amongst other things. The scene primarily interacts with the deserializer
//! and the integrator.

use crate::hittable::Hittable;

/// A scene with objects, lighting information, and other configuration options for rendering
///
/// The `Scene` struct contains all of the information that an integrator needs to generate an
/// image.
#[derive(Debug)]
pub struct Scene<T> {
    /// A list of all of the objects in the scene
    objects: Vec<Box<dyn Hittable<T>>>,

    /// An acceleration structure containing all of the visible objects in the scene that can be
    /// queried to calculate a ray intersecion.
    acceleration_struct: Box<dyn Hittable<T>>,
}
