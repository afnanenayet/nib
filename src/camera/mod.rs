//! A module defining the generic interface for cameras and providing interfaces for various camera
//! types

use crate::{ray::Ray, types::Float};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod pinhole;
mod thin_lens;

pub use pinhole::BasicPinhole;
pub use pinhole::Pinhole;
pub use thin_lens::ThinLens;

/// The generic interface for a camera type
///
/// A camera simply needs to convert u, v coordinates to a 3D ray.
pub trait Camera: Debug + Send + Sync {
    /// Convert (u, v) pixel coordinates to a ray in 3D space
    ///
    /// This method expects (u, v) coordinates that lie in the unit plane [0, 1]. It also expects
    /// an aspect ratio, which is just nx / ny, where nx and ny are the horizontal and vertical
    /// pixels, respectively.
    fn to_ray(&self, u: Float, v: Float) -> Ray;
}

/// The different types of cameras that can be used in the scene description
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SerializedCamera {
    BasicPinhole(BasicPinhole),
    Pinhole(Pinhole),
    ThinLens(ThinLens),
}
