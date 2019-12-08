//! Defines the interface for materials that are associated with geometric objects. This interface
//! gives us texture and lighting information by defining a BSDF function for a particular
//! geometric object.

use crate::{hittable::HitRecord, sampler::Sampler, types::GenFloat, types::Ray};
use cgmath::Vector3;
use std::fmt::Debug;

mod diffuse;
mod mirror;

pub use diffuse::Diffuse;
pub use mirror::Mirror;

/// This trait defines some sort of object that can specify how light is scattered when the
/// material is hit.
///
/// This interface provides one method: the `scatter` function, which will return a `BSDFRecord`
pub trait BSDF<T: GenFloat>: Debug {
    /// Return the result of a scattering function on an input ray
    fn scatter(
        &self,
        s: &mut dyn Sampler<T>,
        ray: &Ray<T>,
        hit_record: &HitRecord<T>,
    ) -> BSDFRecord<T>;
}

/// The result of the BSDF scatter function
///
/// A BSDF hit record entails an outgoing ray and the attenuation factor for that ray.
pub struct BSDFRecord<T: GenFloat> {
    /// The outgoing ray
    pub out: Ray<T>,

    /// The attenuation factor to apply to the outgoing ray
    pub attenuation: Vector3<T>,
}
