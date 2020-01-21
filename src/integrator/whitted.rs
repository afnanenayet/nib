//! An implementation of the Whitted/direct lighting rendering technique. This is the "classic"
//! ray-tracing technique. It was developed by Turner Whitted in 1980, in a paper titled "An
//! Improved Illumination Model for Shaded Display."

use crate::{
    integrator::{Integrator, RenderParams},
    types::{GenFloat, PixelValue, BLACK},
};
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
    fn render(&self, params: RenderParams<T>) -> PixelValue {
        self.render_helper(params, 0)
    }
}

impl<T: GenFloat> Whitted<T> {
    /// The recursive helper method for the Whitted integrator
    ///
    /// This exists because we need to keep track of the stack depth as we cast new rays and the
    /// `Integrator` trait doesn't have a parameter for depth.
    fn render_helper(&self, params: RenderParams<T>, depth: u32) -> PixelValue {
        if depth > self.max_depth {
            return params.scene.background;
        }

        // First, we check to see if the ray hit anything
        if let Some(hit_record) = params.scene.accel.collision(&params.origin) {
            todo!();
        } else {
            return BLACK;
        }
    }
}
