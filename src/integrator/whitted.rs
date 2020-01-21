//! An implementation of the Whitted/direct lighting rendering technique. This is the "classic"
//! ray-tracing technique. It was developed by Turner Whitted in 1980, in a paper titled "An
//! Improved Illumination Model for Shaded Display."

use crate::{
    integrator::{Integrator, RenderParams},
    math::component_mul,
    types::{GenFloat, PixelValue},
};
use cgmath::Vector3;
use num::Zero;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// The parameters for the Whitted integrator
#[derive(Debug, Serialize, Deserialize)]
pub struct Whitted<T: GenFloat> {
    /// The recursion limit for rays
    ///
    /// This settings sets an upper bound on the depth of the rays in the scene (this is necessary
    /// in case there is infinite recursion in the scene).
    pub max_depth: u32,

    /// This exists so we can have the type parameter `T`, which is used with the `RenderParams<T>`
    /// struct in the `Integrator` implementation.
    phantom: PhantomData<T>,
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
    /// The recursive helper method for the Whitted integrator
    ///
    /// This exists because we need to keep track of the stack depth as we cast new rays and the
    /// `Integrator` trait doesn't have a parameter for depth.
    fn render_helper(&self, params: RenderParams<T>, depth: u32) -> PixelValue<T> {
        if depth > self.max_depth {
            return params.scene.background;
        }

        // First, we check to see if the ray hit anything, if not, we return a black background.
        // TODO(afnan) change this to be more extensible, such as allowing for a gradient or
        // an environment map
        if let Some(collision) = params.scene.accel.collision(&params.origin) {
            let bsdf_record =
                collision
                    .object
                    .mat
                    .scatter(params.sampler, params.origin, &collision.hit_record);
            // Calculate values of the rays recursively, accumulating as we go
            let color = component_mul(
                bsdf_record.attenuation,
                self.render_helper(params, depth + 1),
            );
            return color;
        }
        return Vector3::zero();
    }
}
