//! The BSDF for dielectrics/glass-like materials

use crate::{
    hittable::HitRecord,
    material::{BSDFRecord, BSDF},
    math::{mirror, schlick},
    ray::Ray,
    sampler::Sampler,
    types::GenFloat,
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
pub struct Dielectric<T: GenFloat> {
    /// The refraction index of the material
    pub refraction_index: T,
}

/// Determine whether a ray will refract given the parameters
///
/// This method will return a refraction vector if the method refracts. If the material doesn't
/// refract (meaning that there will be total internal reflection), then it will return the `None`
/// variant.
fn refract<T: GenFloat>(v: Vector3<T>, n: Vector3<T>, ni_over_nt: T) -> Option<Vector3<T>> {
    // unit vector
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant =
        T::from(1).unwrap() - ni_over_nt * ni_over_nt * (T::from(1).unwrap() - dt * dt);
    if discriminant > T::from(0).unwrap() {
        Some(
            ((uv - n.map(|x| x * dt)) - n.map(|x| x * discriminant.sqrt())).map(|x| x * ni_over_nt),
        )
    } else {
        None
    }
}

impl<T: GenFloat> BSDF<T> for Dielectric<T> {
    fn scatter(
        &self,
        s: &mut dyn Sampler<T>,
        ray: &Ray<T>,
        hit_record: &HitRecord<T>,
    ) -> BSDFRecord<T> {
        let reflection_vector = mirror(&ray.direction, &hit_record.normal);
        let (outward_normal, ni_over_nt, cosine) =
            if ray.direction.dot(hit_record.normal) > T::from(0).unwrap() {
                (
                    -hit_record.normal,
                    self.refraction_index,
                    self.refraction_index * ray.direction.dot(hit_record.normal)
                        / ray.direction.magnitude(),
                )
            } else {
                (
                    hit_record.normal,
                    T::from(1).unwrap() / self.refraction_index,
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
            attenuation: Vector3::new(
                T::from(1).unwrap(),
                T::from(1).unwrap(),
                T::from(1).unwrap(),
            ),
            out: Ray {
                origin: hit_record.p,
                direction: outgoing_direction,
            },
        }
    }
}
