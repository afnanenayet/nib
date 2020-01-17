mod accel;
mod camera;
mod hittable;
mod image_exporter;
mod integrator;
mod material;
mod math;
mod renderer;
mod sampler;
mod scene;
mod types;

use std::path::PathBuf;
use structopt::StructOpt;

/// An oxidized renderer
#[derive(StructOpt, Debug)]
#[structopt(author = "Afnan Enayet")]
struct Args {
    /// The path to the file describing the scene
    pub scene: PathBuf,

    /// If enabled, this flag will hide the progress bar. The progress bar is ordinarily displayed
    /// to STDERR.
    #[structopt(short = "p", long = "hide-progress")]
    pub hide_progress: bool,

    /// The number of threads to use in the renderer
    #[structopt(short, long)]
    pub threads: Option<u32>,
}

fn main() {
    let args = Args::from_args();
    let sampler = sampler::Random::default();
    let camera: camera::Pinhole<f32> = camera::Pinhole::default();
}
