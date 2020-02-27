use crate::{
    hittable::HitRecord,
    material::{BSDFRecord, BSDF},
    ray::Ray,
    sampler::{primitives::sample_unit_sphere, Sampler},
    types::GenFloat,
};
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

/// A diffuse BSDF function
///
/// This BSDF models a typical matte, or non-glossy surface. The user can specify the albedo of the
/// material, which defines its color.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Diffuse<T: GenFloat> {
    /// The fraction of light that is absorbed for each color channel.
    pub albedo: Vector3<T>,
}

impl<T: GenFloat> BSDF<T> for Diffuse<T> {
    fn scatter(
        &self,
        s: &mut dyn Sampler<T>,
        _ray: &Ray<T>,
        hit_record: &HitRecord<T>,
    ) -> BSDFRecord<T> {
        let target = hit_record.p + hit_record.normal + sample_unit_sphere(s);
        let out = Ray {
            origin: hit_record.p,
            direction: (target - hit_record.p),
        };
        let attenuation = self.albedo;
        BSDFRecord { out, attenuation }
    }
}
