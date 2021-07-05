extern crate rey_skytracer;

use rey_skytracer::render::{gen_random_scene, render};

use clap::Clap;

/// Simple CLI to showcase Tracey's ray tracing
#[derive(Clap)]
#[clap(name = "Tracey")]
struct TraceyArgs {
    /// Width of the output image in pixels
    #[clap(short, long)]
    width: u32,

    /// Number of rays used per pixel
    #[clap(short, long)]
    rays: u32,
}

fn main() {
    let args = TraceyArgs::parse();

    let (world, light) = gen_random_scene();
    render(&world, light, args.width, args.rays);
}
