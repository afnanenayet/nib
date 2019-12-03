//! A module providing definitions and methods for a scene. A scene contains most of the relevant
//! information for an integrator, and it provides information such as lights, geometric objects,
//! camera information, amongst other things. The scene primarily interacts with the deserializer
//! and the integrator.

use crate::{
    accel, camera,
    hittable::{self, Hittable},
    types::GenFloat,
};
use serde::{Deserialize, Serialize};

/// The different types of `Hittable` types that can be as input objects
///
/// This is an enum type that exists for convenient use with serde, so we can create a serializable
/// struct to expose as a scene description to the user.
#[derive(Debug, Serialize, Deserialize, Clone)]
enum SerializedHittable<T: GenFloat> {
    Sphere(hittable::Sphere<T>),
}

/// The different types of acceleration structures that can be used in the scene description
#[derive(Debug, Serialize, Deserialize, Clone)]
enum SerializedAccelerationStruct {
    ObjectList,
}

/// The different types of cameras that can be used in the scene description
#[derive(Debug, Serialize, Deserialize)]
enum SerializedCamera<T: GenFloat> {
    Pinhole(camera::Pinhole<T>),
}

/// A struct representing the scene description as the user will input it
///
/// This struct exists solely for serialization and deserialization
#[derive(Debug, Serialize, Deserialize)]
struct Scene<T: GenFloat> {
    /// A list of all of the geometric objects in the scene
    pub objects: Vec<SerializedHittable<T>>,

    /// The acceleration structure to use with the scene
    pub acceleration_structure: SerializedAccelerationStruct,

    /// The camera to use with the scene
    pub camera: SerializedCamera<T>,
}

/// A scene with objects, lighting information, and other configuration options for rendering
///
/// The `ProcessedScene` struct contains all of the information that an integrator needs to generate an
/// image. This is generated from the input `Scene` struct that is primarily used for serializing
/// and deserializing scene information from user input.
#[derive(Debug)]
pub struct ProcessedScene<'a, T> {
    /// An acceleration structure containing all of the visible objects in the scene that can be
    /// queried to calculate a ray intersecion. We don't need to store a list of objects in this
    /// struct because the acceleration structure is the only thing that will process objects that
    /// are visible and can be hit by rays.
    pub acceleration_struct: Box<dyn Hittable<T> + 'a>,

    /// The camera being used to render the scene
    pub camera: Box<dyn camera::Camera<T> + 'a>,
}

impl<'a, T: GenFloat + 'a> From<Scene<T>> for ProcessedScene<'a, T> {
    fn from(scene: Scene<T>) -> Self {
        // We just destructure the serialized struct and convert them to boxed dynamic
        // implementations
        let objects: Vec<Box<dyn Hittable<T> + 'a>> = (&scene.objects)
            .iter()
            .map(|x| {
                let x: Box<dyn Hittable<T> + 'a> = match x {
                    SerializedHittable::Sphere(x) => Box::new(x.clone()),
                };
                x
            })
            .collect();
        let camera = match scene.camera {
            SerializedCamera::Pinhole(x) => Box::new(x),
        };
        ProcessedScene {
            camera,
            acceleration_struct: Box::new(accel::ObjectList::new(objects).unwrap()),
        }
    }
}
