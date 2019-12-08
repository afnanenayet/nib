use crate::{
    hittable::HitRecord,
    material::{BSDFRecord, BSDF},
    math::mirror,
    types::{GenFloat, Ray},
};
use cgmath::Vector3;
use rand::prelude::*;

/// A perfect mirror surface
///
/// This BSDF simply reflects incoming light rays around the normal vector of the surface (the
/// normal is computed from the ray intersection on the geometric object). The mirror BSDF has no
/// parameters, which is why it is an empty struct.
#[derive(Debug, Copy, Clone, Default)]
pub struct Mirror {}

impl<T, R> BSDF<T, R> for Mirror
where
    T: GenFloat,
    R: Rng + ?Sized,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    fn scatter(&self, _r: &mut R, ray: &Ray<T>, hit_record: &HitRecord<T>) -> BSDFRecord<T> {
        let mirror_direction = mirror(&ray.direction, &hit_record.normal);
        BSDFRecord {
            out: Ray {
                origin: ray.origin,
                direction: mirror_direction,
            },
            attenuation: Vector3::new(
                T::from(1).unwrap(),
                T::from(1).unwrap(),
                T::from(1).unwrap(),
            ),
        }
    }
}
