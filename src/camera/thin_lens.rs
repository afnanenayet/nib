//! A model of a thin-lens camera

use crate::{
    camera::{BasicPinhole, Camera},
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
    /// The internal basic pinhole camera that we keep around for precomputed values
    basic_pinhole: Option<BasicPinhole<T>>,

    /// The target that the camera is pointing towards from the origin
    pub target: Vector3<T>,
    /// The origin point of the camera
    pub origin: Vector3<T>,
    /// The vertical field of view of the camera
    pub vfov: T,
    /// Which direction you consider up for the camera
    pub up: Vector3<T>,
    /// The aspect ratio of the camera
    pub aspect: T,
}

impl<T: GenFloat> Camera<T> for ThinLens<T> {
    fn to_ray(&self, u: T, v: T) -> crate::ray::Ray<T> {
        unimplemented!()
    }
}
