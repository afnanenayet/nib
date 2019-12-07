mod accel;
mod camera;
mod hittable;
mod image_exporter;
mod integrator;
mod math;
mod renderer;
mod sampler;
mod scene;
mod types;

fn main() {
    let sampler = sampler::Random::default();
    let camera: camera::Pinhole<f32> = camera::Pinhole::default();
}
