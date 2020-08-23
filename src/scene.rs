//! A module providing definitions and methods for a scene. A scene contains most of the relevant
//! information for an integrator, and it provides information such as lights, geometric objects,
//! camera information, amongst other things. The scene primarily interacts with the deserializer
//! and the integrator.

use crate::{
    accel,
    camera::{Camera, SerializedCamera},
    hittable::{Hittable, SerializedHittable, SerializedTextured, Textured},
    integrator::{Integrator, SerializedIntegrator},
    material::{SerializedMaterial, BSDF},
    renderer::{Arena, Renderer},
    types::{GenFloat, PixelValue},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// The different types of acceleration structures that can be used in the scene description
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SerializedAccelerationStruct {
    ObjectList,
}

/// A struct representing the scene description as the user will input it
///
/// This struct exists solely for serialization and deserialization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scene<T: GenFloat> {
    /// A list of all of the geometric objects in the scene
    pub objects: Vec<SerializedTextured<T>>,

    /// The acceleration structure to use with the scene
    pub acceleration_structure: SerializedAccelerationStruct,

    /// The camera to use with the scene
    pub camera: SerializedCamera<T>,

    /// The background color to return when no objects are hit
    pub background: PixelValue<T>,

    /// The number of samples to take per pixel. This is effectively the anti-aliasing factor.
    pub samples_per_pixel: u32,

    /// The integrator to use to render the scene
    pub integrator: SerializedIntegrator<T>,

    /// The vertical resolution of the scene, in pixels
    pub height: u32,

    /// The horizontal resolution of the scene, in pixels
    pub width: u32,
}

impl<'a, T> From<Scene<T>> for Renderer<'a, T>
where
    T: GenFloat + 'a,
{
    fn from(scene: Scene<T>) -> Self {
        let aspect_ratio = T::from(scene.height).unwrap() / T::from(scene.width).unwrap();
        // We just destructure the serialized struct and convert them to boxed dynamic
        // implementations
        let arena: Arena<'a, T> = Arc::new(scene.objects.iter().map(|&x| x.into()).collect());
        let camera: Box<dyn Camera<T>> = match scene.camera {
            SerializedCamera::Pinhole(x) => Box::new(x.init(aspect_ratio)),
            SerializedCamera::BasicPinhole(x) => Box::new(x),
            SerializedCamera::ThinLens(x) => Box::new(x),
        };
        let integrator: Box<dyn Integrator<T>> = Box::new(scene.integrator);
        let accel = match scene.acceleration_structure {
            SerializedAccelerationStruct::ObjectList => {
                Box::new(accel::ObjectList::new(arena.clone()).unwrap())
            }
        };
        Renderer {
            arena,
            camera,
            integrator,
            accel,
            background: scene.background,
            samples_per_pixel: scene.samples_per_pixel,
            height: scene.height,
            width: scene.width,
        }
    }
}
