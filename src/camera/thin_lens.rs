//! A model of a thin-lens camera

use crate::{
    camera::{BasicPinhole, Camera},
    ray::Ray,
    types::Float,
};
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

/// A thin lens camera model
///
/// This camera model uses an approximation of a lens, rather than simulating a full lens. This
/// allows for effects like depth of field and adjustable apertures, without the cost of simulating
/// light refracting through len(ses).
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct ThinLens {
    /// The internal basic pinhole camera that we keep around for precomputed values
    basic_pinhole: Option<BasicPinhole>,

    /// The target that the camera is pointing towards from the origin
    pub target: Vector3<Float>,
    /// The origin point of the camera
    pub origin: Vector3<Float>,
    /// The vertical field of view of the camera
    pub vfov: Float,
    /// Which direction you consider up for the camera
    pub up: Vector3<Float>,
    /// The aspect ratio of the camera
    pub aspect: Float,
}

impl Camera for ThinLens {
    fn to_ray(&self, _u: Float, _v: Float) -> Ray {
        unimplemented!()
    }
}
