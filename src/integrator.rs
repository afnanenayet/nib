//! THe integrator module defines several integrators, and also provides an interface for generic
//! integrators that can be extended so that other integrators can be easily added.

use crate::scene::Scene;
use crate::types::{GenFloat, GenInteger};
use cgmath::{Point3, Vector3};

/// A trait that defines an integrator. An integrator defines the operations that are responsible
/// for taking input data for a given pixel, and calculating the output colors at each pixel.
pub trait Integrator<I: GenInteger, F: GenFloat> {
    /// Given certain input parameters, output the color values at a particular point. This method
    /// takes a sampler to generate random numbers and a pixel location.
    // TODO(afnan) add sampler parameter as an argument
    fn render(p: &Point3<I>, scene: &Scene) -> Vector3<F>;
}
