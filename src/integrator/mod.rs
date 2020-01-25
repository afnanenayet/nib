//! The integrator module defines several integrators, and also provides an interface for generic
//! integrators that can be extended so that other integrators can be easily added.

use crate::types::PixelValue;
use crate::{
    sampler::Sampler,
    scene::ProcessedScene,
    types::{GenFloat, Ray},
};
use enum_dispatch::enum_dispatch;
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
pub struct RenderParams<'a, 'b, 'c, T: GenFloat> {
    /// The outgoing ray
    ///
    /// In rendering, we trace rays or paths, and they originate at a certain point and extend
    /// until the next collision, or go on forever.
    pub origin: &'a Ray<T>,

    /// A reference to the scene
    pub scene: &'b ProcessedScene<'b, T>,

    /// A reference to the sampler to use with the integrator
    pub sampler: &'c mut dyn Sampler<T>,
}

/// A trait that defines an integrator. An integrator defines the operations that are responsible
/// for taking input data for a given pixel, and calculating the output colors at each pixel.
#[enum_dispatch(SerializedIntegrator)]
pub trait Integrator<T: GenFloat>: Debug + Send + Sync {
    /// Calculate the color value for a particular pixel, given a reference to the scene.
    ///
    /// Given certain input parameters this method calculates the color values at a particular
    /// point.
    fn render(&self, params: RenderParams<T>) -> PixelValue<T>;
}

#[enum_dispatch]
#[derive(Debug)]
pub enum SerializedIntegrator<T: GenFloat> {
    Normal(Normal<T>),
    Whitted(Whitted<T>),
}
