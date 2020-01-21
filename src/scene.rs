//! A module providing definitions and methods for a scene. A scene contains most of the relevant
//! information for an integrator, and it provides information such as lights, geometric objects,
//! camera information, amongst other things. The scene primarily interacts with the deserializer
//! and the integrator.

use crate::{
    accel::{self, Accel},
    camera::{self, SerializedCamera},
    hittable::{self, Hittable, Textured},
    material::{SerializedMaterial, BSDF},
    types::{GenFloat, PixelValue},
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

/// The different types of `Hittable` types that can be used as input objects
///
/// This is an enum type that exists for convenient use with serde, so we can create a serializable
/// struct to expose as a scene description to the user.
#[enum_dispatch]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SerializedHittable<T: GenFloat> {
    Sphere(hittable::Sphere<T>),
}

/// The different types of acceleration structures that can be used in the scene description
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SerializedAccelerationStruct {
    ObjectList,
}

/// A serializable wrapper for the
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedTextured<T>
where
    T: GenFloat,
{
    /// The geometric primitive that might be hit by the light ray or path
    pub geometry: SerializedHittable<T>,

    /// A reference to the BSDF method for
    pub mat: SerializedMaterial<T>,
}

/// A struct representing the scene description as the user will input it
///
/// This struct exists solely for serialization and deserialization
#[derive(Debug, Serialize, Deserialize)]
pub struct Scene<T: GenFloat> {
    /// A list of all of the geometric objects in the scene
    pub objects: Vec<SerializedTextured<T>>,

    /// The acceleration structure to use with the scene
    pub acceleration_structure: SerializedAccelerationStruct,

    /// The camera to use with the scene
    pub camera: SerializedCamera<T>,

    /// The background color to return when no objects are hit
    pub background: PixelValue,

    /// The number of samples to take per pixel. This is effectively the anti-aliasing factor.
    pub samples_per_pixel: u32,
}

/// A scene with objects, lighting information, and other configuration options for rendering
///
/// The `ProcessedScene` struct contains all of the information that an integrator needs to generate an
/// image. This is generated from the input `Scene` struct that is primarily used for serializing
/// and deserializing scene information from user input.
#[derive(Debug)]
pub struct ProcessedScene<'a, T: GenFloat> {
    pub accel: Box<dyn Accel<T> + 'a>,
    pub camera: Box<dyn camera::Camera<T> + 'a>,
    pub background: PixelValue,
    pub samples_per_pixel: u32,
}

impl<'a, T> From<Scene<T>> for ProcessedScene<'a, T>
where
    T: GenFloat + 'a,
{
    fn from(scene: Scene<T>) -> Self {
        // We just destructure the serialized struct and convert them to boxed dynamic
        // implementations
        let objects: Vec<Textured<'a, T>> = (&scene.objects)
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
            background: scene.background, // unknown pixels are black
            samples_per_pixel: scene.samples_per_pixel,
        }
    }
}

/// Generate a set of sample scenes for documentation and reference
///
/// The serialized scenes are both generated and parsed programatically, based off the underlying
/// types, with serde. T
macro_rules! sample_scenes {
    () => {
        let scenes: Vec<Scene<f32>> = vec![
            Scene {
                objects: vec![SerializedTextured {
                    geometry: SerializedHittable::Sphere(Sphere {
                        center: Vector3::new(0.0, 0.0, 0.0),
                        radius: 1.0,
                    }),
                    mat: material::Mirror::default().into(),
                }],
                camera: camera::Pinhole::default().into(),
                background: [0, 0, 0],
                samples_per_pixel: 100,
                acceleration_structure: SerializedAccelerationStruct::ObjectList,
            },
        ]
    };
}
