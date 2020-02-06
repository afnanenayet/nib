//! An implementation of a basic pinhole camera

use crate::{camera::Camera, ray::Ray, types::GenFloat};
use cgmath::{InnerSpace, Vector3};
use num::Float;
use serde::{Deserialize, Serialize};

/// The classic pinhole camera
///
/// No bells, no whistles, just projected rays.
#[derive(Debug, Serialize, Deserialize)]
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
    fn to_ray(&self, u: T, v: T) -> Ray<T> {
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

impl<T: GenFloat> Camera<T> for Pinhole<T> {
    fn to_ray(&self, u: T, v: T) -> Ray<T> {
        let theta = self.vfov * T::from(std::f32::consts::PI).unwrap() / T::from(180).unwrap();
        let half_height = Float::tan(theta / T::from(2).unwrap());
        let half_width = self.aspect * half_height;
        let w = (self.origin - self.target).normalize();
        let u_prime = (self.up.cross(w)).normalize();
        let v_prime = w.cross(u_prime);
        let lower_left =
            self.origin - u_prime.map(|x| x * half_width) - v_prime.map(|x| x * half_height) - w;
        let horizontal: Vector3<T> = u_prime.map(|x| x * T::from(2).unwrap() * half_width);
        let vertical: Vector3<T> = v_prime.map(|x| x * T::from(2).unwrap() * half_height);
        Ray {
            origin: self.origin,
            direction: (lower_left + (horizontal * u) + (vertical * v) - self.origin).normalize(),
        }
    }
}

impl<T: GenFloat> Pinhole<T> {}

/// A pinhole camera, much like `BasicPinhole`, that allows you to specify the aspect ratio and the
/// field of view.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pinhole<T: GenFloat> {
    /// The target that the camera is pointing towards from the origin
    pub target: Vector3<T>,
    /// The origin point of the camera
    pub origin: Vector3<T>,
    /// The vertical field of view of the camera
    pub vfov: T,
    /// Which direction you consider up for the camera
    pub up: Vector3<T>,
    /// The aspect ratio of the camera
    pub aspect: T,
}

#[cfg(test)]
mod test {
    use super::*;

    type FType = f32;

    #[test]
    fn test_get_ray_f32() {
        let camera: BasicPinhole<FType> = Default::default();

        // this is equivalent to the lower left corner of the frame
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 0.0), ray);

        // middle
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.5, 0.5), ray);

        // upper left corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 1.0), ray);

        // upper right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 1.0), ray);

        // lower right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 0.0), ray);
    }

    #[test]
    fn test_get_ray_f64() {
        type FType = f64;
        let camera: BasicPinhole<FType> = Default::default();

        // this is equivalent to the lower left corner of the frame
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 0.0), ray);

        // middle
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.5, 0.5), ray);

        // upper left corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 1.0), ray);

        // upper right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 1.0), ray);

        // lower right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 0.0), ray);
    }
}
