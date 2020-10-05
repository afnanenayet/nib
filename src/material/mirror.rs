use crate::{
    hittable::HitRecord,
    material::{BSDFRecord, BSDF},
    math::mirror,
    ray::Ray,
    sampler::{primitives::sample_unit_sphere, Sampler},
    types::Float,
};
use cgmath::{InnerSpace, Vector3};
use serde::{Deserialize, Serialize};

/// A perfect mirror surface
///
/// This BSDF simply reflects incoming light rays around the normal vector of the surface (the
/// normal is computed from the ray intersection on the geometric object). The mirror BSDF has no
/// parameters, which is why it is an empty struct.
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Mirror {
    /// The perturbation factor for the surface, which dictates how "fuzzy" reflections will be
    ///
    /// A perturbation of zero corresponds to a perfectly mirror surface. You can increase this
    /// factor to 1.0, which will yield very fuzzy reflections.
    pub perturbation: Float,

    /// The color of the mirror surface
    ///
    /// We allow reflective materials to have color. A regular mirror would have an albedo of
    /// `[1.0, 1.0, 1.0]`.
    pub albedo: Vector3<Float>,
}

impl Default for Mirror {
    fn default() -> Self {
        Self {
            perturbation: 0.0,
            albedo: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl BSDF for Mirror {
    fn scatter(&self, s: &mut dyn Sampler<Float>, ray: &Ray, hit_record: &HitRecord) -> BSDFRecord {
        let mirror_direction = mirror(&ray.direction, &hit_record.normal);
        let direction =
            (mirror_direction + sample_unit_sphere(s).map(|x| x * self.perturbation)).normalize();
        let attenuation = if hit_record.normal.dot(direction) > 0.0 {
            self.albedo
        } else {
            Vector3::new(0.0, 0.0, 0.0)
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
