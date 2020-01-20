mod accel;
mod camera;
mod cli;
mod hittable;
mod image_exporter;
mod integrator;
mod material;
mod math;
mod renderer;
mod sampler;
mod scene;
mod types;

use crate::scene::Scene;
use std::error::Error;
use cli::{dispatch_scene_parse, Args};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();
    let scene: Scene<f32> = dispatch_scene_parse(&args.scene)?;
    Ok(())
}
