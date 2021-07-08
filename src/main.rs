extern crate rey_skytracer;

use std::path::Path;
use std::sync::Arc;

use rey_skytracer::hittables::World;
use rey_skytracer::linalg::Color;
use rey_skytracer::load_mesh::load_mesh;
use rey_skytracer::materials::Lambertian;
use rey_skytracer::render::render;
use rey_skytracer::scene::{get_cornell_box_scene, Scene};

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

    let blue_mat = Arc::new(Lambertian::new(Color::new(0.45, 0.71, 0.95)));
    // let metal_mat = Arc::new(Metal::new(Color::new(0.45, 0.71, 0.95), 0.2));

    let mut objects = World::default();
    let triangle_mesh_opt = load_mesh(Path::new("./sample_meshes/tachikoma_3.obj"), blue_mat);
    objects.add(Arc::new(triangle_mesh_opt.unwrap()));

    let scene = get_cornell_box_scene(objects);
    render(args.width, args.rays, scene);
}
