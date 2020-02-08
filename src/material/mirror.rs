use crate::{
    hittable::HitRecord,
    material::{BSDFRecord, BSDF},
    math::{mirror, sample_unit_sphere},
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
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Mirror<T: GenFloat> {
    /// The perturbation factor for the surface, which dictates how "fuzzy" reflections will be
    ///
    /// A perturbation of zero corresponds to a perfectly mirror surface. You can increase this
    /// factor to 1.0, which will yield very fuzzy reflections.
    pub perturbation: T,

    /// The color of the mirror surface
    ///
    /// We allow reflective materials to have color. A regular mirror would have an albedo of
    /// `[1.0, 1.0, 1.0]`.
    pub albedo: Vector3<T>,
}

impl<T: GenFloat> Default for Mirror<T> {
    fn default() -> Self {
        Self {
            perturbation: T::from(0).unwrap(),
            albedo: Vector3::new(
                T::from(1).unwrap(),
                T::from(1).unwrap(),
                T::from(1).unwrap(),
            ),
        }
    }
}

impl<T: GenFloat> BSDF<T> for Mirror<T> {
    fn scatter(
        &self,
        s: &mut dyn Sampler<T>,
        ray: &Ray<T>,
        hit_record: &HitRecord<T>,
    ) -> BSDFRecord<T> {
        let mirror_direction = mirror(&ray.direction, &hit_record.normal);
        let direction =
            (mirror_direction + sample_unit_sphere(s).map(|x| x * self.perturbation)).normalize();
        let attenuation = if hit_record.normal.dot(direction) > T::from(0).unwrap() {
            self.albedo
        } else {
            Vector3::new(
                T::from(0).unwrap(),
                T::from(0).unwrap(),
                T::from(0).unwrap(),
            )
        };
        BSDFRecord {
            out: Ray {
                origin: hit_record.p,
                direction,
            },
            attenuation,
        }
    }
}
