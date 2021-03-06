//! The integrator module defines several integrators, and also provides an interface for generic
//! integrators that can be extended so that other integrators can be easily added.

use crate::{
    ray::Ray,
    renderer::Renderer,
    sampler::Sampler,
    types::{Float, PixelValue},
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub mod normal;
pub mod whitted;

pub use normal::Normal;
pub use whitted::Whitted;

/// A struct representing the various options and parameters that can be configured for the
/// `render` method in a particular integrator
///
/// We use a struct rather than simply passing in parameters to make it easier to extend the
/// parameters that are used in the render method, as you add new parameters without breaking
/// existing integrators.
///
/// You should pass this struct by value since it's simply a struct of references to objects, which
/// is pretty cheap.
#[derive(Debug)]
pub struct RenderParams<'a, 'b, 'c> {
    /// The outgoing ray
    ///
    /// In rendering, we trace rays or paths, and they originate at a certain point and extend
    /// until the next collision, or go on forever.
    pub origin: &'a Ray,

    /// A reference to the renderer itself
    pub context: &'b Renderer,

    /// A reference to the sampler to use with the integrator
    pub sampler: &'c mut dyn Sampler<Float>,
}

/// A trait that defines an integrator. An integrator defines the operations that are responsible
/// for taking input data for a given pixel, and calculating the output colors at each pixel.
#[enum_dispatch(SerializedIntegrator)]
pub trait Integrator: Debug + Send + Sync {
    /// Calculate the color value for a particular pixel, given a reference to the scene.
    ///
    /// Given certain input parameters this method calculates the color values at a particular
    /// point.
    fn render(&self, params: RenderParams) -> PixelValue<Float>;
}

#[enum_dispatch]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SerializedIntegrator {
    Normal(Normal),
    Whitted(Whitted),
}
