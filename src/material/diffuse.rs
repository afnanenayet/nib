use crate::{
    hittable::HitRecord,
    material::{BSDFRecord, BSDF},
    math::sample_unit_sphere,
    types::{GenFloat, Ray},
};
use cgmath::Vector3;
use rand::prelude::*;

/// A diffuse BSDF function
///
/// This BSDF models a typical matte, or non-glossy surface. The user can specify the albedo of the
/// material, which defines its color.
#[derive(Debug, Clone, Copy)]
pub struct Diffuse<T: GenFloat> {
    /// The fraction of light that is absorbed for each color channel.
    pub albedo: Vector3<T>,
}

impl<T, R> BSDF<T, R> for Diffuse<T>
where
    T: GenFloat,
    R: Rng + ?Sized,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn scatter(&self, r: &mut R, ray: &Ray<T>, hit_record: &HitRecord<T>) -> BSDFRecord<T> {
        let target = hit_record.p + hit_record.normal + sample_unit_sphere(&mut r);
        let out = Ray {
            origin: hit_record.p,
            direction: target - hit_record.p,
        };
        let attenuation = self.albedo;
        BSDFRecord { out, attenuation }
    }
}
