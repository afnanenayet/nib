use crate::{
    hittable::HitRecord,
    material::{BSDFRecord, BSDF},
    math::mirror,
    ray::Ray,
    sampler::Sampler,
    types::GenFloat,
};
use cgmath::{InnerSpace, Vector3};
use serde::{Deserialize, Serialize};

/// A perfect mirror surface
///
/// This BSDF simply reflects incoming light rays around the normal vector of the surface (the
/// normal is computed from the ray intersection on the geometric object). The mirror BSDF has no
/// parameters, which is why it is an empty struct.
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct Mirror {}

impl<T> BSDF<T> for Mirror
where
    T: GenFloat,
{
    fn scatter(
        &self,
        _s: &mut dyn Sampler<T>,
        ray: &Ray<T>,
        hit_record: &HitRecord<T>,
    ) -> BSDFRecord<T> {
        let mirror_direction = mirror(&ray.direction, &hit_record.normal);
        BSDFRecord {
            out: Ray {
                origin: ray.origin,
                direction: mirror_direction.normalize(),
            },
            attenuation: Vector3::new(
                T::from(1).unwrap(),
                T::from(1).unwrap(),
                T::from(1).unwrap(),
            ),
        }
    }
}
