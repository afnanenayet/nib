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

use crate::scene::*;
use cgmath::Vector3;
use hittable::Sphere;
use serde_json;
use std::{error::Error, fs::File, io::Read, path::PathBuf};
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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();
    let mut scene_file = File::open(args.scene)?;
    let mut buffer = String::new();
    scene_file.read_to_string(&mut buffer)?;
    let scene: Scene<f32> = serde_json::from_str(&buffer)?;
    Ok(())
}
