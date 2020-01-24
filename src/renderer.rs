//! The renderer implementation, which is pretty much the entire app
//!
//! The renderer takes a scene, and calculates the value for each pixel using a given integrator.
//! This acts as the main executor module to coordinate computation in the renderer.

use crate::{
    camera::Camera,
    integrator::{Integrator, RenderParams},
    sampler::{self, Sampler},
    scene::ProcessedScene,
    types::{GenFloat, PixelValue},
};
use anyhow;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// The data a renderer requires to produce an image
///
/// The renderer struct stores data that the renderer requires to process the image. This struct
/// also implements the methods necessary to produce an image.
#[derive(Debug)]
pub struct Renderer<'a, T: GenFloat> {
    /// The integrator to use with the scene
    pub integrator: Box<dyn Integrator<T>>,

    ///// The sampler/sampling str>ategy to use with the scene
    //pub sampler: Box<dyn Sampler<T>>,
    /// A representation of the scene and lighting information
    pub scene: ProcessedScene<'a, T>,

    /// The camera to use to render this instance
    pub camera: Box<dyn Camera<T>>,

    /// The width of the output image
    pub width: u32,

    /// The height of the output image
    pub height: u32,
}

impl<'a, T> Renderer<'a, T>
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
    T: GenFloat,
{
    /// A small convenience method to generate the progress bar for the CLI
    fn create_progress_bar(&self) -> ProgressBar {
        let n = (self.width * self.height).into();
        let pb = ProgressBar::new(n);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{elapsed_precise}/{eta_precise} [{wide_bar}] {percent}%")
                .progress_chars("=> "),
        );
        pb.set_draw_delta(n / 100);
        pb
    }

    pub fn render(&mut self) -> anyhow::Result<Vec<PixelValue<T>>> {
        let pb = self.create_progress_bar();
        let sampler = sampler::Random::default();

        // This is the naive single threaded implementation. TODO(afnan) make this multithreaded
        // once the results are confirmed to be correct.
        // We use a sampler per thread rather than locking a sampler over all threads because the
        // lock contention would be too high. Having a sampler per thread seems to tbe the most
        // performant choice.
        let buffer = (0..(self.width * self.height))
            .into_par_iter()
            .map_with(
                || sampler.clone(),
                |sampler_generator, i| {
                    let mut sampler = sampler_generator();
                    let x = i % self.width;
                    let y = self.height - (i / self.width);
                    let acc: PixelValue<T> = (0..self.scene.samples_per_pixel)
                        .map(|_| {
                            let camera_samples = sampler.next(2).unwrap();

                            let u = (T::from(x).unwrap() + camera_samples[0])
                                / T::from(self.width).unwrap();
                            let v = (T::from(y).unwrap() + camera_samples[1])
                                / T::from(self.height).unwrap();
                            let ray = self.camera.to_ray(u, v);
                            let params = RenderParams {
                                origin: &ray,
                                scene: &self.scene,
                                sampler: &mut sampler,
                            };
                            let color = self.integrator.render(params);
                            color
                        })
                        .fold(
                            PixelValue::new(
                                T::from(0).unwrap(),
                                T::from(0).unwrap(),
                                T::from(0).unwrap(),
                            ),
                            |acc, x| acc + x,
                        );
                    pb.inc(1);
                    let spp = T::from(self.scene.samples_per_pixel).unwrap();
                    PixelValue::new(acc.x / spp, acc.y / spp, acc.z / spp)
                },
            )
            .collect();
        pb.finish_and_clear();
        Ok(buffer)
    }
}
