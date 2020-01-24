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
        // TODO(afnan) throw this back in when we figure out how to pass our boy between threads
        // safely or generate one per thread
        //sampler: Box::new(sampler::Random::default()),
        scene: processed_scene,
        camera: Box::new(camera::Pinhole::default()),
        width: args.width,
        height: args.height,
    };
    let buffer = renderer.render(args.threads)?;
    let exporter = PPMExporter {
        width: args.width,
        height: args.height,
    };
    let output_path = Path::new("out.ppm");
    exporter.export(&buffer, output_path)?;
    Ok(())
}
