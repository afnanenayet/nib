//! A module providing definitions and methods for a scene. A scene contains most of the relevant
//! information for an integrator, and it provides information such as lights, geometric objects,
//! camera information, amongst other things. The scene primarily interacts with the deserializer
//! and the integrator.

use crate::{
    accel::SerializedAccelerationStruct,
    camera::{Camera, SerializedCamera},
    hittable::SerializedTextured,
    integrator::{Integrator, SerializedIntegrator},
    renderer::{Arena, Renderer},
    types::{Float, PixelValue},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A struct representing the scene description as the user will input it
///
/// This struct exists solely for serialization and deserialization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scene {
    /// A list of all of the geometric objects in the scene
    pub objects: Vec<SerializedTextured>,

    /// The acceleration structure to use with the scene
    pub acceleration_structure: SerializedAccelerationStruct,

    /// The camera to use with the scene
    pub camera: SerializedCamera,

    /// The background color to return when no objects are hit
    pub background: PixelValue<Float>,

    /// The number of samples to take per pixel. This is effectively the anti-aliasing factor.
    pub samples_per_pixel: u32,

    /// The integrator to use to render the scene
    pub integrator: SerializedIntegrator,

    /// The vertical resolution of the scene, in pixels
    pub height: u32,

    /// The horizontal resolution of the scene, in pixels
    pub width: u32,
}

impl From<Scene> for Renderer {
    fn from(scene: Scene) -> Self {
        let aspect_ratio = (scene.height as Float) / (scene.width as Float);
        // We just destructure the serialized struct and convert them to boxed dynamic
        // implementations
        let arena: Arena = Arc::new(scene.objects.iter().map(|&x| x.into()).collect());
        let camera: Box<dyn Camera> = match scene.camera {
            SerializedCamera::Pinhole(x) => Box::new(x.init(aspect_ratio)),
            SerializedCamera::BasicPinhole(x) => Box::new(x),
            SerializedCamera::ThinLens(x) => Box::new(x),
        };
        let integrator: Box<dyn Integrator> = Box::new(scene.integrator);
        let accel = scene
            .acceleration_structure
            .to_accel(arena.clone())
            .expect("Unable to construct acceleration structure");
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
