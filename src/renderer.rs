//! The renderer implementation, which is pretty much the entire app
//!
//! The renderer takes a scene, and calculates the value for each pixel using a given integrator.
//! This acts as the main executor module to coordinate computation in the renderer.

use crate::{
    camera::Camera,
    integrator::{Integrator, RenderParams},
    sampler::Sampler,
    scene::ProcessedScene,
    types::{GenFloat, PixelValue},
};
use indicatif::ProgressIterator;

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

    /// The camera to use to render this instance
    pub camera: Box<dyn Camera<T>>,

    /// The width of the output image
    pub width: u32,

    /// The height of the output image
    pub height: u32,
}

impl<'a, T: GenFloat> Renderer<'a, T> {
    pub fn render(&mut self) -> Vec<PixelValue> {
        // This is the naive single threaded implementation. TODO(afnan) make this multithreaded
        // once the results are confirmed to be correct.
        (0..self.width)
            .progress()
            .zip((0..self.height).progress())
            .map(|(x, y)| {
                let u = T::from(x).unwrap() / T::from(self.width).unwrap();
                let v = T::from(y).unwrap() / T::from(self.height).unwrap();
                let ray = self.camera.to_ray(u, v);

                let params = RenderParams {
                    origin: &ray,
                    scene: &self.scene,
                    sampler: &mut *self.sampler,
                };
                self.integrator.render(params)
            })
            .collect()
    }
}
