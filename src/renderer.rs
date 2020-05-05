//! The renderer implementation, which is pretty much the entire app
//!
//! The renderer takes a scene, and calculates the value for each pixel using a given integrator.
//! This acts as the main executor module to coordinate computation in the renderer.

use crate::{
    accel::Accel,
    camera,
    integrator::{Integrator, RenderParams},
    sampler::{self, Sampler},
    types::{GenFloat, PixelValue},
};
use anyhow;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

/// All of the information associated with the renderer required for generating an image from the
/// scene
///
/// The `RendererContext` struct contains all of the information that an integrator needs to
/// generate an image. This is generated from the input `Scene` struct that is primarily used for
/// serializing and deserializing scene information from user input.
#[derive(Debug)]
pub struct Renderer<'a, T: GenFloat> {
    pub accel: Box<dyn Accel<T> + 'a>,
    pub camera: Box<dyn camera::Camera<T> + 'a>,
    pub background: PixelValue<T>,
    pub samples_per_pixel: u32,
    pub integrator: Box<dyn Integrator<T> + 'a>,
    pub height: u32,
    pub width: u32,
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

    /// Render the image, returning a buffer of pixels
    ///
    /// You can optionally specify the number of threads you'd like to use. If this is unset or set
    /// to 0, Rayon will automatically infer the number of threads to use based on the number of
    /// logical CPUs detected on the system.
    pub fn render(&mut self, num_threads: Option<usize>) -> anyhow::Result<Vec<PixelValue<T>>> {
        let pb = self.create_progress_bar();
        let sampler = sampler::Random::default();

        if let Some(n) = num_threads {
            set_threads(n)?;
        }

        // We use a sampler per thread rather than sharing a sampler over all threads because the
        // lock contention causes a large performance hit.
        let mut buffer = Vec::with_capacity((self.width * self.height) as usize);
        (0..(self.width * self.height))
            .into_par_iter()
            .map_with(
                || sampler.clone(),
                |sampler_generator, i| {
                    let mut sampler = sampler_generator();
                    let x = i % self.width;
                    let y = self.height - (i / self.width);
                    let acc: PixelValue<T> = (0..self.samples_per_pixel)
                        .map(|_| {
                            let camera_samples = sampler.next(2).unwrap();

                            let u = (T::from(x).unwrap() + camera_samples[0])
                                / T::from(self.width).unwrap();
                            let v = (T::from(y).unwrap() + camera_samples[1])
                                / T::from(self.height).unwrap();
                            let ray = self.camera.to_ray(u, v);
                            let params = RenderParams {
                                origin: &ray,
                                context: &self,
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
                    let spp = T::from(self.samples_per_pixel).unwrap();
                    PixelValue::new(acc.x / spp, acc.y / spp, acc.z / spp)
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
