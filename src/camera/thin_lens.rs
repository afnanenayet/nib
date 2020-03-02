//! A model of a thin-lens camera

use crate::{
    camera::Camera,
    ray::Ray,
    sampler::{primitives::sample_unit_disk, Sampler},
    types::GenFloat,
};
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

/// A thin lens camera model
///
/// This camera model uses an approximation of a lens, rather than simulating a full lens. This
/// allows for effects like depth of field and adjustable apertures, without the cost of simulating
/// light refracting through len(ses).
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ThinLens<T: GenFloat> {
    pub lens_radius: T,
    pub origin: Vector3<T>,
    pub u: Vector3<T>,
    pub v: Vector3<T>,
    pub w: Vector3<T>,
    pub lower_left: Vector3<T>,
    pub horizontal: Vector3<T>,
    pub vertical: Vector3<T>,
}

impl<T: GenFloat> Camera<T> for ThinLens<T> {
    fn to_ray(&self, x: T, y: T, s: &mut dyn Sampler<T>) -> Ray<T> {
        let rd = sample_unit_disk(s).map(|x| x * self.lens_radius);
        // random offset calculated by sampling the unit disk
        let offset = self.u.map(|i| i * rd.x) + self.v.map(|i| i * rd.y);
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left
                + self.horizontal.map(|i| i * x)
                + self.vertical.map(|i| i * y)
                - self.origin
                - offset,
        }
    }
}

/// The parameters for a thin lens camera model
///
/// This struct contains the data that a user will input, which will then be processed and
/// converted into a `ThinLens` struct, which has the actual runtime information for the renderer.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ThinLensParameters<T: GenFloat> {
    /// The origin of the camera, where the camera is placed
    pub look_from: Vector3<T>,

    /// The target position for the camera.
    ///
    /// The camera is looking at this point from the `look_from` position.
    pub look_at: Vector3<T>,

    /// Which orientation is "up" to the camera.
    ///
    /// This parameter can be used to rotate the camera.
    pub up: Vector3<T>,

    /// The field of view for the camera, in degrees
    pub fov: Vector3<T>,

    /// The aperture size of the camera
    ///
    /// This dictates how strong the depth of field effect is going to be (or as Pete Shirley
    /// calls it, "defocus blur").
    pub aperture: T,

    /// The focus distance for the camera
    ///
    /// Everything around this plane will be blurred. The strength of the blur is dictated by the
    /// aperture size.
    pub focus_distance: T,
}

impl<T: GenFloat> ThinLensParameters<T> {
    /// Initialize a `ThinLens` runtime struct from user-provided input
    pub fn init(self) -> ThinLens<T> {
        todo!()
    }
}
