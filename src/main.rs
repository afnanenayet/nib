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

use crate::{
    image_exporter::{FramebufferExporter, PPMExporter},
    renderer::Renderer,
    scene::Scene,
};
use cli::{dispatch_scene_parse, Args};
use std::{
    error::Error,
    path::Path,
    sync::{Arc, Mutex},
};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();
    let scene: Scene<f32> = dispatch_scene_parse(&args.scene, args.filetype.as_deref())?;
    let processed_scene = scene.into();
    let mut renderer = Renderer {
        integrator: Box::new(integrator::Whitted::default()),
        //sampler: Box::new(sampler::Random::default()),
        scene: processed_scene,
        camera: Box::new(camera::Pinhole::default()),
        width: 800,
        height: 400,
    };
    let buffer = renderer.render()?;
    let exporter = PPMExporter {
        width: 800,
        height: 400,
    };
    let output_path = Path::new("out.ppm");
    exporter.export(&buffer, output_path)?;
    Ok(())
}
