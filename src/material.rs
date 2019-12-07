//! Defines the interface for materials that are associated with geometric objects. This interface
//! gives us texture and lighting information by defining a BSDF function for a particular
//! geometric object.

use cgmath::Vector3;

/// This trait defines some sort of object that can specify how light is scattered when the
/// material is hit.
///
/// This interface provides one method: the `scatter` function, which will return a `BSDFRecord`
pub trait BSDF<T> {
    /// Return the result of a scattering function on an input ray
    fn scatter(&self, ray: &Ray<T>, hit_record: &HitRecord<T>) -> BSDFRecord<T>;
}

/// The result of the BSDF scatter function
///
/// A BSDF hit record entails an outgoing ray and the attenuation factor for that ray.
pub struct BSDFRecord<T> {
    pub out: Ray<T>,
    pub attenuation: Vector3<T>,
}
