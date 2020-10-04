//! The renderer implementation, which is pretty much the entire app
//!
//! The renderer takes a scene, and calculates the value for each pixel using a given integrator.
//! This acts as the main executor module to coordinate computation in the renderer.

use crate::{
    accel::Accel,
    camera,
    hittable::Textured,
    integrator::{Integrator, RenderParams},
    sampler::{self, Sampler},
    types::{Float, PixelValue},
};
use anyhow;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Arc;

pub type Arena = Arc<Vec<Textured>>;

/// All of the information associated with the renderer required for generating an image from the
/// scene
///
/// The `RendererContext` struct contains all of the information that an integrator needs to
/// generate an image. This is generated from the input `Scene` struct that is primarily used for
/// serializing and deserializing scene information from user input.
#[derive(Debug)]
pub struct Renderer {
    pub arena: Arena,
    pub accel: Box<dyn Accel>,
    pub camera: Box<dyn camera::Camera>,
    pub background: PixelValue<Float>,
    pub samples_per_pixel: u32,
    pub integrator: Box<dyn Integrator>,
    pub height: u32,
    pub width: u32,
}

impl Renderer {
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

    /// Render the image, returning a buffer of pixels
    ///
    /// You can optionally specify the number of threads you'd like to use. If this is unset or set
    /// to 0, Rayon will automatically infer the number of threads to use based on the number of
    /// logical CPUs detected on the system.
    pub fn render(&mut self, num_threads: Option<usize>) -> anyhow::Result<Vec<PixelValue<Float>>> {
        let pb = self.create_progress_bar();
        let sampler = sampler::Random::default();

        if let Some(n) = num_threads {
            set_threads(n)?;
        }

        // So we can avoid recomputing these with every pixel
        let width_float = self.width as Float;
        let height_float = self.height as Float;
        let spp_float = self.samples_per_pixel as Float;

        // We use a sampler per thread rather than sharing a sampler over all threads because the
        // lock contention causes a large performance hit.
        let mut buffer = Vec::with_capacity((self.width * self.height) as usize);
        (0..(self.width * self.height))
            .into_par_iter()
            .map_with(
                || sampler.clone(),
                |sampler_generator, i| {
                    let mut sampler = sampler_generator();
                    let x = (i % self.width) as Float;
                    let y = (self.height - (i / self.width)) as Float;
                    let acc: PixelValue<Float> = (0..self.samples_per_pixel)
                        .map(|_| {
                            let camera_samples = sampler.next(2).unwrap();

                            let u = (x + camera_samples[0]) / width_float;
                            let v = (y + camera_samples[1]) / height_float;
                            let ray = self.camera.to_ray(u, v);
                            let params = RenderParams {
                                origin: &ray,
                                context: &self,
                                sampler: &mut sampler,
                            };
                            let color = self.integrator.render(params);
                            color
                        })
                        .fold(PixelValue::new(0.0, 0.0, 0.0), |acc, x| acc + x);
                    pb.inc(1);
                    PixelValue::new(acc.x / spp_float, acc.y / spp_float, acc.z / spp_float)
                },
            )
            .collect_into_vec(&mut buffer);
        pb.finish_and_clear();
        Ok(buffer)
    }
}

/// Set the number of threads in the global threadpool
fn set_threads(num_threads: usize) -> Result<(), rayon::ThreadPoolBuildError> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
}
