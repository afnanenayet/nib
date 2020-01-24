//! A module defining the generic interface for cameras and providing interfaces for various camera
//! types

use crate::types::{GenFloat, Ray};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod pinhole;

pub use pinhole::Pinhole;

/// The generic interface for a camera type
///
/// A camera simply needs to convert u, v coordinates to a 3D ray.
#[enum_dispatch(SerializedCamera)]
pub trait Camera<T: GenFloat>: Debug + Send + Sync {
    /// Convert (u, v) pixel coordinates to a ray in 3D space
    fn to_ray(&self, u: T, v: T) -> Ray<T>;
}

/// The different types of cameras that can be used in the scene description
#[enum_dispatch]
#[derive(Debug, Serialize, Deserialize)]
pub enum SerializedCamera<T: GenFloat> {
    Pinhole(Pinhole<T>),
}
