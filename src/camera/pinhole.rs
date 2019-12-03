//! An implementation of a basic pinhole camera

use crate::{
    camera::Camera,
    types::{GenFloat, Ray},
};
use cgmath::Vector3;
use serde::{Deserialize, Serialize};

/// The classic pinhole camera
///
/// No bells, no whistles, just projected rays.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pinhole<T: GenFloat> {
    /// The origin point of the camera's field of view
    pub origin: Vector3<T>,
    /// The horizontal span of the camera's field of view
    pub horizontal: Vector3<T>,
    /// The vertical span of the camera's field of view
    pub vertical: Vector3<T>,
    /// The lower left corner of the camera's field of view
    pub lower_left: Vector3<T>,
}

impl<T: GenFloat> Camera<T> for Pinhole<T> {
    fn to_ray(&self, u: T, v: T) -> Ray<T> {
        Ray {
            origin: self.origin,
            direction: self.lower_left + (self.horizontal * u) + (self.vertical * v) - self.origin,
        }
    }
}

impl<T: GenFloat> Default for Pinhole<T> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_ray_f32() {
        type FType = f32;
        let camera: Pinhole<FType> = Default::default();

        // this is equivalent to the lower left corner of the frame
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, -1.0, -1.0),
        };
        assert_eq!(camera.to_ray(0.0, 0.0), ray);

        // middle
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
        };
        assert_eq!(camera.to_ray(0.5, 0.5), ray);

        // upper left corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, 1.0, -1.0),
        };
        assert_eq!(camera.to_ray(0.0, 1.0), ray);

        // upper right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, 1.0, -1.0),
        };
        assert_eq!(camera.to_ray(1.0, 1.0), ray);

        // lower right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, -1.0, -1.0),
        };
        assert_eq!(camera.to_ray(1.0, 0.0), ray);
    }

    #[test]
    fn test_get_ray_f64() {
        type FType = f64;
        let camera: Pinhole<FType> = Default::default();

        // this is equivalent to the lower left corner of the frame
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, -1.0, -1.0),
        };
        assert_eq!(camera.to_ray(0.0, 0.0), ray);

        // middle
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
        };
        assert_eq!(camera.to_ray(0.5, 0.5), ray);

        // upper left corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, 1.0, -1.0),
        };
        assert_eq!(camera.to_ray(0.0, 1.0), ray);

        // upper right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, 1.0, -1.0),
        };
        assert_eq!(camera.to_ray(1.0, 1.0), ray);

        // lower right corner
        let ray: Ray<FType> = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, -1.0, -1.0),
        };
        assert_eq!(camera.to_ray(1.0, 0.0), ray);
    }
}
