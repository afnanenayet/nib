//! An implementation of the sphere primitive

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    types::GenFloat,
};
use cgmath::{prelude::*, Vector3};
use serde::{Deserialize, Serialize};

/// A sphere primitive
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sphere<T: GenFloat> {
    /// The center of the sphere in spatial coordinates
    pub center: Vector3<T>,

    /// The radius of the sphere
    pub radius: T,
}

impl<T: GenFloat> Hittable<T> for Sphere<T> {
    fn hit(&self, ray: &Ray<T>) -> Option<HitRecord<T>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let b = T::from(2.0).unwrap() * oc.dot(ray.direction);
        let c = oc.magnitude2() - (self.radius * self.radius);
        let discriminant = (b * b) - (T::from(4).unwrap() * a * c);

        // Otherwise we'll get a NaN
        if discriminant < T::from(0).unwrap() {
            return None;
        }
        let t = ((T::from(-1).unwrap() * b) - discriminant.sqrt()) / (T::from(2).unwrap() * a);

        // A collision can't have a negative distance
        if t < T::from(0).unwrap() {
            return None;
        }
        let p = ray.origin + (ray.direction * t);
        let normal = (p - self.center).normalize();
        Some(HitRecord {
            distance: t,
            p,
            normal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Represents a test case for the sphere collision, with the input value (the ray), and the
    /// expected result, an optional `HitRecord`.
    struct TestCase {
        /// The outgoing ray
        pub ray: Ray<f32>,

        /// The sphere configuration being used in this case
        pub sphere: Sphere<f32>,

        /// The expected result
        pub expected: Option<HitRecord<f32>>,
    }

    /// Unit tests for when the outgoing ray should completely miss the sphere
    #[test]
    fn zero_intersections() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                ray: Ray {
                    origin: Vector3::new(2.0, 0.0, 0.0),
                    direction: Vector3::new(1.0, 0.0, 0.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                },
                expected: None,
            },
            TestCase {
                ray: Ray {
                    origin: Vector3::new(0.0, 2.0, 0.0),
                    direction: Vector3::new(0.0, 1.0, 0.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 0.2,
                },
                expected: None,
            },
            TestCase {
                ray: Ray {
                    origin: Vector3::new(0.0, 0.0, 2.0),
                    direction: Vector3::new(0.0, 0.0, 1.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 0.7,
                },
                expected: None,
            },
        ];

        for test_case in test_cases {
            let sphere = test_case.sphere;
            let result = sphere.hit(&test_case.ray);
            assert_eq!(test_case.expected, result);
        }
    }

    /// We are testing for instances where the ray hits the sphere at exactly one point
    #[test]
    fn one_intersection() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                ray: Ray {
                    origin: Vector3::new(-1.0, -1.0, 0.0),
                    direction: Vector3::new(0.0, 1.0, 0.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                },
                expected: Some(HitRecord {
                    p: Vector3::new(-1.0, 0.0, 0.0),
                    normal: Vector3::new(-1.0, 0.0, 0.0),
                    distance: 1.0,
                }),
            },
            TestCase {
                ray: Ray {
                    origin: Vector3::new(-1.0, -1.0, 0.0),
                    direction: Vector3::new(1.0, 0.0, 0.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                },
                expected: Some(HitRecord {
                    p: Vector3::new(0.0, -1.0, 0.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                    distance: 1.0,
                }),
            },
            TestCase {
                ray: Ray {
                    origin: Vector3::new(0.0, 1.0, -1.0),
                    direction: Vector3::new(0.0, 0.0, 1.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                },
                expected: Some(HitRecord {
                    p: Vector3::new(0.0, 1.0, 0.0),
                    normal: Vector3::new(0.0, 1.0, 0.0),
                    distance: 1.0,
                }),
            },
        ];

        for test_case in test_cases {
            let sphere = test_case.sphere;
            let result = sphere.hit(&test_case.ray);
            assert_eq!(test_case.expected, result);
        }
    }

    // Testing cases where the ray intersects the sphere at two points, which should yield the
    // point of intersection that's closest to the ray's origin
    #[test]
    fn two_intersections() {
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                ray: Ray {
                    origin: Vector3::new(0.0, -2.0, 0.0),
                    direction: Vector3::new(0.0, 1.0, 0.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                },
                expected: Some(HitRecord {
                    p: Vector3::new(0.0, -1.0, 0.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                    distance: 1.0,
                }),
            },
            TestCase {
                ray: Ray {
                    origin: Vector3::new(0.0, 2.0, 0.0),
                    direction: Vector3::new(0.0, -1.0, 0.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                },
                expected: Some(HitRecord {
                    p: Vector3::new(0.0, 1.0, 0.0),
                    normal: Vector3::new(0.0, 1.0, 0.0),
                    distance: 1.0,
                }),
            },
            TestCase {
                ray: Ray {
                    origin: Vector3::new(0.0, 0.0, 2.0),
                    direction: Vector3::new(0.0, 0.0, -1.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                },
                expected: Some(HitRecord {
                    p: Vector3::new(0.0, 0.0, 1.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                    distance: 1.0,
                }),
            },
            TestCase {
                ray: Ray {
                    origin: Vector3::new(-2.0, 0.0, 0.0),
                    direction: Vector3::new(1.0, 0.0, 0.0),
                },
                sphere: Sphere {
                    center: Vector3::new(0.0, 0.0, 0.0),
                    radius: 1.0,
                },
                expected: Some(HitRecord {
                    p: Vector3::new(-1.0, 0.0, 0.0),
                    normal: Vector3::new(-1.0, 0.0, 0.0),
                    distance: 1.0,
                }),
            },
        ];

        for test_case in test_cases {
            let sphere = test_case.sphere;
            let result = sphere.hit(&test_case.ray);
            assert_eq!(test_case.expected, result);
        }
    }
}
