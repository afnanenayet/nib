//! A module providing definitions and methods for a scene. A scene contains most of the relevant
//! information for an integrator, and it provides information such as lights, geometric objects,
//! camera information, amongst other things. The scene primarily interacts with the deserializer
//! and the integrator.

use crate::{
    accel, camera,
    hittable::{self, Hittable, Textured},
    material::{self, BSDF},
    types::{GenFloat, PixelValue},
};
use rand;
use serde::{Deserialize, Serialize};

/// The different types of `Hittable` types that can be used as input objects
///
/// This is an enum type that exists for convenient use with serde, so we can create a serializable
/// struct to expose as a scene description to the user.
#[derive(Debug, Serialize, Deserialize, Clone)]
enum SerializedHittable<T: GenFloat> {
    Sphere(hittable::Sphere<T>),
}

/// The different types of `BSDF` types that can be used as input objects
#[derive(Debug, Clone, Serialize, Deserialize)]
enum SerializedMaterial<T: GenFloat> {
    Diffuse(material::Diffuse<T>),
    Mirror(material::Mirror),
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

/// A serializable wrapper for the
#[derive(Debug, Serialize, Deserialize)]
struct SerializedTextured<T: GenFloat> {
    /// The geometric primitive that might be hit by the light ray or path
    pub geometry: SerializedHittable<T>,

    /// A reference to the BSDF method for
    pub mat: SerializedMaterial<T>,
}

/// A struct representing the scene description as the user will input it
///
/// This struct exists solely for serialization and deserialization
#[derive(Debug, Serialize, Deserialize)]
struct Scene<T: GenFloat> {
    /// A list of all of the geometric objects in the scene
    pub objects: Vec<SerializedTextured<T>>,

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
pub struct ProcessedScene<'a, T: GenFloat> {
    /// An acceleration structure containing all of the visible objects in the scene that can be
    /// queried to calculate a ray intersecion. We don't need to store a list of objects in this
    /// struct because the acceleration structure is the only thing that will process objects that
    /// are visible and can be hit by rays.
    pub accel: Box<dyn Hittable<T> + 'a>,

    /// The camera being used to render the scene
    pub camera: Box<dyn camera::Camera<T> + 'a>,

    /// The background color to return when no objects are hit.
    ///
    /// This is a fallback method for when no other color can be computed.
    pub background: PixelValue,
}

impl<'a, T> From<Scene<T>> for ProcessedScene<'a, T>
where
    T: GenFloat + 'a,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn from(scene: Scene<T>) -> Self {
        // We just destructure the serialized struct and convert them to boxed dynamic
        // implementations
        let objects: Vec<Textured<T>> = (&scene.objects)
            .iter()
            .map(
                |SerializedTextured {
                     geometry: g,
                     mat: m,
                 }| {
                    let geometry: Box<dyn Hittable<T> + 'a> = match g {
                        SerializedHittable::Sphere(x) => Box::new(x.clone()),
                    };
                    let bsdf: Box<dyn BSDF<T> + 'a> = match m {
                        SerializedMaterial::Mirror(x) => Box::new(x.clone()),
                        SerializedMaterial::Diffuse(x) => Box::new(x.clone()),
                    };
                    Textured {
                        geometry,
                        mat: bsdf,
                    }
                },
            )
            .collect();
        let camera = match scene.camera {
            SerializedCamera::Pinhole(x) => Box::new(x),
        };
        ProcessedScene {
            camera,
            accel: Box::new(accel::ObjectList::new(objects).unwrap()),
            background: [0, 0, 0], // unknown pixels are black
        }
    }
}
