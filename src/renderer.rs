//! The renderer implementation, which is pretty much the entire app
//!
//! The renderer takes a scene, and calculates the value for each pixel using a given integrator.

use crate::{integrator::Integrator, sampler::Sampler, scene::ProcessedScene, types::GenFloat};

/// The data a renderer requires to produce an image
///
/// The renderer struct stores data that the renderer requires to process the image. This struct
/// also implements the methods necessary to produce an image.
#[derive(Debug)]
pub struct Renderer<'a, T: GenFloat> {
    /// The integrator to use with the scene
    pub integrator: Box<dyn Integrator<T>>,

    /// The sampler/sampling strategy to use with the scene
    pub sampler: Box<dyn Sampler<T>>,

    /// A representation of the scene and lighting information
    pub scene: Box<ProcessedScene<'a, T>>,
}
