//! The BSDF for dielectrics/glass-like materials

use crate::{
    hittable::HitRecord,
    material::{BSDFRecord, BSDF},
    math::{mirror, schlick},
    ray::Ray,
    sampler::Sampler,
    types::Float,
};
use cgmath::{InnerSpace, Vector3};
use serde::{Deserialize, Serialize};

/// The BSDF model for a perfectly smooth dielectric material
///
/// You need to specify the internal and external indices of refraction for this BSDF model. It's
/// important to make sure that you specify meaningful IOR values so that the material looks
/// realistic and accurate. There are many references for IOR values that you can find on the
/// internet.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Dielectric {
    /// The refraction index of the material
    pub refraction_index: Float,

    /// The fraction of light absorbed in each color channel
    ///
    /// This is just the color/tint of the material. The default value for this is `[1.0, 1.0,
    /// 1.0]`
    #[serde(default = "default_albedo")]
    pub albedo: Vector3<Float>,
}

/// The default provider for `albedo` in `Dielectric`
///
/// This is the default value provider for serde so elements can deserialize a struct without
/// support for this
fn default_albedo() -> Vector3<Float> {
    Vector3::from([1.0; 3])
}

impl Default for Dielectric {
    fn default() -> Self {
        Self {
            refraction_index: 1.0,
            albedo: default_albedo(),
        }
    }
}

/// Determine whether a ray will refract given the parameters
///
/// This method will return a refraction vector if the method refracts. If the material doesn't
/// refract (meaning that there will be total internal reflection), then it will return the `None`
/// variant.
fn refract(v: Vector3<Float>, n: Vector3<Float>, ni_over_nt: Float) -> Option<Vector3<Float>> {
    // unit vector
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 1.0 {
        Some(
            ((uv - n.map(|x| x * dt)) - n.map(|x| x * discriminant.sqrt())).map(|x| x * ni_over_nt),
        )
    } else {
        None
    }
}

impl BSDF for Dielectric {
    fn scatter(&self, s: &mut dyn Sampler<Float>, ray: &Ray, hit_record: &HitRecord) -> BSDFRecord {
        let reflection_vector = mirror(&ray.direction, &hit_record.normal);
        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(hit_record.normal) > 0.0 {
            (
                -hit_record.normal,
                self.refraction_index,
                self.refraction_index * ray.direction.dot(hit_record.normal)
                    / ray.direction.magnitude(),
            )
        } else {
            (
                hit_record.normal,
                1.0 / self.refraction_index,
                -self.refraction_index * ray.direction.dot(hit_record.normal)
                    / ray.direction.magnitude(),
            )
        };
        let outgoing_direction = match refract(ray.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflection_prob = schlick(cosine, self.refraction_index);
                let r = s.next(1).unwrap()[0];

                if r < reflection_prob {
                    reflection_vector
                } else {
                    refracted
                }
            }
            None => reflection_vector,
        };
        BSDFRecord {
            attenuation: Vector3::new(1.0, 1.0, 1.0),
            out: Ray {
                origin: hit_record.p,
                direction: outgoing_direction,
            },
        }
    }
}
