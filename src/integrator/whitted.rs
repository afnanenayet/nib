//! An implementation of the Whitted/direct lighting rendering technique. This is the "classic"
//! ray-tracing technique. It was developed by Turner Whitted in 1980, in a paper titled "An
//! Improved Illumination Model for Shaded Display."

use crate::{
    integrator::{Integrator, RenderParams},
    types::{Float, PixelValue},
};
use cgmath::{ElementWise, InnerSpace, Vector3};
use serde::{Deserialize, Serialize};

/// The parameters for the Whitted integrator
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Whitted {
    /// The recursion limit for rays
    ///
    /// This settings sets an upper bound on the depth of the rays in the scene (this is necessary
    /// in case there is infinite recursion in the scene).
    pub max_depth: u32,
}

impl Default for Whitted {
    fn default() -> Self {
        Self { max_depth: 5 }
    }
}

impl Integrator for Whitted {
    fn render(&self, params: RenderParams) -> PixelValue<Float> {
        self.render_helper(params, 0)
    }
}

impl Whitted {
    /// The recursive helper method for the Whitted integrator
    ///
    /// This exists because we need to keep track of the stack depth as we cast new rays and the
    /// `Integrator` trait doesn't have a parameter for depth.
    fn render_helper(&self, params: RenderParams, depth: u32) -> PixelValue<Float> {
        // First, we check to see if the ray hit anything, if not, we return a black background.
        // TODO(afnan) change this to be more extensible, such as allowing for a gradient or
        // an environment map
        if let Some(collision) = params.context.accel.collision(&params.origin) {
            if depth >= self.max_depth {
                return params.context.background;
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
        let t = 0.5 * (unit_dir.y + 1.0);

        // linearly interpolate a color based on the angle of the ray
        return (Vector3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vector3::new(0.7, 0.7, 0.7) * t);
    }
}
