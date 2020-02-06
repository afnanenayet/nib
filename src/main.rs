mod accel;
mod camera;
mod cli;
mod hittable;
mod image_exporter;
mod integrator;
mod material;
mod math;
mod ray;
mod renderer;
mod sampler;
mod scene;
mod types;

use crate::{
    image_exporter::{FramebufferExporter, PPMExporter},
    renderer::Renderer,
    scene::Scene,
};
use anyhow;
use cli::{dispatch_scene_parse, Args};
use std::{
    error::Error,
    path::Path,
    sync::{Arc, Mutex},
};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    let scene: Scene<f32> = dispatch_scene_parse(&args.scene, args.filetype.as_deref())?;
    let processed_scene = scene.into();
    let mut renderer = Renderer {
        integrator: Box::new(integrator::Whitted::new(5)),
        //integrator: Box::new(integrator::Normal::default()),
        // TODO(afnan) throw this back in when we figure out how to pass our boy between threads
        // safely or generate one per thread
        //sampler: Box::new(sampler::Random::default()),
        scene: processed_scene,
        width: args.width,
        height: args.height,
    };
    let buffer = renderer.render(args.threads)?;
    let exporter = PPMExporter {
        width: args.width,
        height: args.height,
    };
    let output_str = &args.output.unwrap_or("out.ppm".to_string());
    let output_path = Path::new(output_str);
    exporter.export(&buffer[..], output_path)?;
    Ok(())
}
