//! The integrator module defines several integrators, and also provides an interface for generic
//! integrators that can be extended so that other integrators can be easily added.

use crate::scene::Scene;
use crate::types::{GenFloat, GenInteger, Ray};
use cgmath::Vector3;

/// A struct representing the various options and parameters that can be configured for the
/// `render` method in a particular integrator
///
/// We use a struct rather than simply passing in parameters to make it easier to extend the
/// parameters that are used in the render method, as you add new parameters without breaking
/// existing integrators.
///
/// You should pass this struct by value since it's simply a struct of references to objects, which
/// is pretty cheap.
pub struct RenderParams<'a, T> {
    /// The outgoing ray
    ///
    /// In rendering, we trace rays or paths, and they originate at a certain point and extend
    /// until the next collision, or go on forever.
    origin: &'a Ray<T>,

    /// A reference to the scene
    ///
    /// This is necessary so the render
    scene: &'a Scene,
}

/// A trait that defines an integrator. An integrator defines the operations that are responsible
/// for taking input data for a given pixel, and calculating the output colors at each pixel.
pub trait Integrator<I: GenInteger, F: GenFloat> {
    /// Given certain input parameters, output the color values at a particular point. This method
    /// takes a sampler to generate random numbers and a pixel location.
    fn render(params: RenderParams<I>) -> Vector3<F>;
}
