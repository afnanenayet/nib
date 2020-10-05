//! Implementations of pinhole cameras

use crate::{camera::Camera, ray::Ray, types::Float};
use cgmath::{InnerSpace, Vector3};
use serde::{Deserialize, Serialize};

/// The classic pinhole camera
///
/// No bells, no whistles, just projected rays.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct BasicPinhole {
    /// The origin point of the camera's field of view
    pub origin: Vector3<Float>,
    /// The horizontal span of the camera's field of view
    pub horizontal: Vector3<Float>,
    /// The vertical span of the camera's field of view
    pub vertical: Vector3<Float>,
    /// The lower left corner of the camera's field of view
    pub lower_left: Vector3<Float>,
}

impl Camera for BasicPinhole {
    fn to_ray(&self, u: Float, v: Float) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left + (self.horizontal * u) + (self.vertical * v)
                - self.origin)
                .normalize(),
        }
    }
}

impl Default for BasicPinhole {
    /// Return the standard camera parameters as defined in page 20 of "Ray Tracing in One Weekend"
    fn default() -> Self {
        Self {
            origin: Vector3::new(0.0, 0.0, 0.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0),
            lower_left: Vector3::new(-2.0, -1.0, -1.0),
        }
    }
}

impl Pinhole {
    /// Initialize the Pinhole camera with computed parameters
    ///
    /// This implementation Pinhole camera provides convenient parameters for users that convert
    /// back into a basic pinhole camera. The underlying method for calculating rays based off of a
    /// camera plane is still the same. Rather than repeating the calculations for every traced
    /// ray, we can calculate the results once and cache them for every use. The init method does
    /// exactly that, actually creating an underlying `BasicPinhole` struct for use at runtime.  
    pub fn init(self, aspect_ratio: Float) -> BasicPinhole {
        let theta = self.vfov * std::f32::consts::PI / 180.0;
        let half_height = Float::tan(theta / 2.0);
        let half_width = aspect_ratio * half_height;
        let w = (self.origin - self.target).normalize();
        let u_prime = (self.up.cross(w)).normalize();
        let v_prime = w.cross(u_prime);
        let lower_left =
            self.origin - u_prime.map(|x| x * half_width) - v_prime.map(|x| x * half_height) - w;
        let horizontal = u_prime * 2.0 * half_width;
        let vertical = v_prime * 2.0 * half_height;

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
pub struct Pinhole {
    /// The target that the camera is pointing towards from the origin
    pub target: Vector3<Float>,
    /// The origin point of the camera
    pub origin: Vector3<Float>,
    /// The vertical field of view of the camera
    pub vfov: Float,
    /// Which direction you consider up for the camera
    pub up: Vector3<Float>,
    /// The aspect ratio of
    pub aspect_ratio: Float,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_ray_f32() {
        let camera: BasicPinhole = Default::default();

        // this is equivalent to the lower left corner of the frame
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 0.0), ray);

        // middle
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.5, 0.5), ray);

        // upper left corner
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 1.0), ray);

        // upper right corner
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 1.0), ray);

        // lower right corner
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 0.0), ray);
    }

    #[test]
    fn test_get_ray_f64() {
        type FType = f64;
        let camera: BasicPinhole = Default::default();

        // this is equivalent to the lower left corner of the frame
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 0.0), ray);

        // middle
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.5, 0.5), ray);

        // upper left corner
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(0.0, 1.0), ray);

        // upper right corner
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, 1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 1.0), ray);

        // lower right corner
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(2.0, -1.0, -1.0).normalize(),
        };
        assert_eq!(camera.to_ray(1.0, 0.0), ray);
    }
}
