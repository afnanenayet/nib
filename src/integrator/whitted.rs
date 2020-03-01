//! An implementation of the Whitted/direct lighting rendering technique. This is the "classic"
//! ray-tracing technique. It was developed by Turner Whitted in 1980, in a paper titled "An
//! Improved Illumination Model for Shaded Display."

use crate::{
    integrator::{Integrator, RenderParams},
    types::{GenFloat, PixelValue},
};
use cgmath::{ElementWise, InnerSpace, Vector3};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// The parameters for the Whitted integrator
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Whitted<T: GenFloat> {
    /// The recursion limit for rays
    ///
    /// This settings sets an upper bound on the depth of the rays in the scene (this is necessary
    /// in case there is infinite recursion in the scene).
    pub max_depth: u32,

    /// This exists so we can have the type parameter `T`, which is used with the `RenderParams<T>`
    /// struct in the `Integrator` implementation.
    ///
    /// Users do not need to supply this field if deserializing a scene file, serde will be able to
    /// produce a valid default value.
    #[serde(default = "default_phantom")]
    phantom: PhantomData<T>,
}

/// A helper method to supply phantom data by default
///
/// Serde requires a method to supply a default value. This function is a shim so that serde can
/// generate a default value for phantom data.
fn default_phantom<T>() -> PhantomData<T> {
    PhantomData
}

impl<T: GenFloat> Default for Whitted<T> {
    fn default() -> Self {
        Self {
            max_depth: 5,
            phantom: PhantomData,
        }
    }
}

impl<T: GenFloat> Integrator<T> for Whitted<T> {
    fn render(&self, params: RenderParams<T>) -> PixelValue<T> {
        self.render_helper(params, 0)
    }
}

impl<T: GenFloat> Whitted<T> {
    pub fn new(max_depth: u32) -> Self {
        Self {
            max_depth,
            phantom: PhantomData,
        }
    }
}

impl<T: GenFloat> Whitted<T> {
    /// The recursive helper method for the Whitted integrator
    ///
    /// This exists because we need to keep track of the stack depth as we cast new rays and the
    /// `Integrator` trait doesn't have a parameter for depth.
    fn render_helper(&self, params: RenderParams<T>, depth: u32) -> PixelValue<T> {
        // First, we check to see if the ray hit anything, if not, we return a black background.
        // TODO(afnan) change this to be more extensible, such as allowing for a gradient or
        // an environment map
        if let Some(collision) = params.scene.accel.collision(&params.origin) {
            if depth >= self.max_depth {
                return params.scene.background;
            }
            let bsdf_record =
                collision
                    .object
                    .mat
                    .scatter(params.sampler, params.origin, &collision.hit_record);
            // Calculate values of the rays recursively, accumulating as we go
            let new_params = RenderParams {
                origin: &bsdf_record.out,
                ..params
            };
            let recursive_color = self.render_helper(new_params, depth + 1);
            let color = bsdf_record.attenuation.mul_element_wise(recursive_color);
            return color;
        }

        // Background is a gradient (temporary measure)
        let unit_dir = params.origin.direction.normalize();
        let t = T::from(0.5).unwrap() * (unit_dir.y + T::from(1.0).unwrap());

        // linearly interpolate a color based on the angle of the ray
        return (Vector3::new(
            T::from(1.0).unwrap(),
            T::from(1.0).unwrap(),
            T::from(1.0).unwrap(),
        ) * (T::from(1.0).unwrap() - t))
            + (Vector3::new(
                T::from(0.7).unwrap(),
                T::from(0.7).unwrap(),
                T::from(0.7).unwrap(),
            ) * t);
    }
}
