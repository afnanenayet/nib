#[cfg(not(target_os = "windows"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

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
    types::Float,
};
use anyhow;
use cli::{dispatch_scene_parse, Args};
use std::path::Path;
use structopt::StructOpt;

#[cfg(not(target_os = "windows"))]
use jemallocator;

fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    let scene: Scene<Float> = dispatch_scene_parse(&args.scene, args.filetype.as_deref())?;
    let (height, width) = (scene.height, scene.width);
    let mut renderer: Renderer<Float> = scene.into();
    let buffer = renderer.render(args.threads)?;
    let exporter = PPMExporter { width, height };
    let output_str = &args.output.unwrap_or("out.ppm".to_string());
    let output_path = Path::new(output_str);
    exporter.export(&buffer[..], output_path)?;
    Ok(())
}
