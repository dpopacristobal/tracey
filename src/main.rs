extern crate rey_skytracer;

use rey_skytracer::render::render;
use rey_skytracer::scene::{get_empty_cornell_box_scene, Scene};

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

    let scene = get_empty_cornell_box_scene();
    render(args.width, args.rays, scene);
}
