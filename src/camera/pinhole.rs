//! Implementations of pinhole cameras

use crate::{camera::Camera, ray::Ray, sampler::Sampler, types::GenFloat};
use cgmath::{InnerSpace, Vector3};
use num::Float;
use serde::{Deserialize, Serialize};

/// The classic pinhole camera
///
/// No bells, no whistles, just projected rays.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct BasicPinhole<T: GenFloat> {
    /// The origin point of the camera's field of view
    pub origin: Vector3<T>,
    /// The horizontal span of the camera's field of view
    pub horizontal: Vector3<T>,
    /// The vertical span of the camera's field of view
    pub vertical: Vector3<T>,
    /// The lower left corner of the camera's field of view
    pub lower_left: Vector3<T>,
}

impl<T: GenFloat> Camera<T> for BasicPinhole<T> {
    fn to_ray(&self, u: T, v: T, _: &mut dyn Sampler<T>) -> Ray<T> {
        Ray {
            origin: self.origin,
            direction: (self.lower_left + (self.horizontal * u) + (self.vertical * v)
                - self.origin)
                .normalize(),
        }
    }
}

impl<T: GenFloat> Default for BasicPinhole<T> {
    /// Return the standard camera parameters as defined in page 20 of "Ray Tracing in One Weekend"
    fn default() -> Self {
        // So we don't have to type this repeatedly
        let zero = T::from(0).unwrap();
        Self {
            origin: Vector3::new(zero, zero, zero),
            horizontal: Vector3::new(T::from(4).unwrap(), zero, zero),
            vertical: Vector3::new(zero, T::from(2).unwrap(), zero),
            lower_left: Vector3::new(
                T::from(-2.0).unwrap(),
                T::from(-1.0).unwrap(),
                T::from(-1.0).unwrap(),
            ),
        }
    }
}

impl<T: GenFloat> Pinhole<T> {
    /// Initialize the Pinhole camera with computed parameters
    ///
    /// This implementation Pinhole camera provides convenient parameters for users that convert
    /// back into a basic pinhole camera. The underlying method for calculating rays based off of a
    /// camera plane is still the same. Rather than repeating the calculations for every traced
    /// ray, we can calculate the results once and cache them for every use. The init method does
    /// exactly that, actually creating an underlying `BasicPinhole` struct for use at runtime.  
    pub fn init(self, aspect_ratio: T) -> BasicPinhole<T> {
        let theta = self.vfov * T::from(std::f32::consts::PI).unwrap() / T::from(180).unwrap();
        let half_height = Float::tan(theta / T::from(2).unwrap());
        let half_width = aspect_ratio * half_height;
        let w = (self.origin - self.target).normalize();
        let u_prime = (self.up.cross(w)).normalize();
        let v_prime = w.cross(u_prime);
        let lower_left =
            self.origin - u_prime.map(|x| x * half_width) - v_prime.map(|x| x * half_height) - w;
        let horizontal: Vector3<T> = u_prime.map(|x| x * T::from(2).unwrap() * half_width);
        let vertical: Vector3<T> = v_prime.map(|x| x * T::from(2).unwrap() * half_height);

        BasicPinhole {
            origin: self.origin,
            horizontal,
            vertical,
            lower_left,
        }
    }
}

/// A pinhole camera, much like `BasicPinhole`, that allows you to specify the aspect ratio and the
/// field of view.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Pinhole<T: GenFloat> {
    /// The target that the camera is pointing towards from the origin
    pub target: Vector3<T>,
    /// The origin point of the camera
    pub origin: Vector3<T>,
    /// The vertical field of view of the camera
    pub vfov: T,
    /// Which direction you consider up for the camera
    pub up: Vector3<T>,
    /// The aspect ratio of
    pub aspect_ratio: T,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sampler::Random;

    fn test_sampler<T: GenFloat>() -> Random<T> {
        Random::default()
    }

    #[test]
    fn test_get_ray_f32() {
        type FType = f32;
        let camera: BasicPinhole<FType> = Default::default();
        let mut s = test_sampler();

        // this is equivalent to the lower left corner of the frame
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 0.0, &mut s), ray);

        // middle
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.5, 0.5, &mut s), ray);

        // upper left corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 1.0, &mut s), ray);

        // upper right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 1.0, &mut s), ray);

        // lower right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 0.0, &mut s), ray);
    }

    #[test]
    fn test_get_ray_f64() {
        type FType = f64;
        let camera: BasicPinhole<FType> = Default::default();
        let mut s = test_sampler();

        // this is equivalent to the lower left corner of the frame
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 0.0, &mut s), ray);

        // middle
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.5, 0.5, &mut s), ray);

        // upper left corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 1.0, &mut s), ray);

        // upper right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 1.0, &mut s), ray);

        // lower right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 0.0, &mut s), ray);
    }
}
