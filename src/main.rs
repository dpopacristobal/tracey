extern crate rey_skytracer;

use std::path::Path;
use std::sync::Arc;

use rey_skytracer::hittables::World;
use rey_skytracer::linalg::Color;
use rey_skytracer::load_mesh::load_mesh;
use rey_skytracer::materials::{Lambertian, Metal};
use rey_skytracer::render::render;
use rey_skytracer::scene::{get_cornell_box_scene, get_random_spheres_scene, Scene};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Tracey",
    about = "A simple CLI to show the ray tracing capabilities in Tracey."
)]
struct TraceyArgs {
    /// Width of the output image in pixels
    #[structopt(long)]
    width: u32,

    /// Number of rays used per pixel
    #[structopt(long)]
    rays: u32,

    /// Path to .obj mesh that will be put into a Cornell box scene
    #[structopt(long, conflicts_with = "sample-scene")]
    mesh_path: Option<String>,

    /// Sample scene number
    #[structopt(long, conflicts_with = "mesh-path")]
    sample_scene: Option<u32>,
}

fn main() {
    let args = TraceyArgs::from_args();
    let scene = scene_from_args(args.mesh_path, args.sample_scene);
    render(args.width, args.rays, scene);
}

fn scene_from_args(mesh_path: Option<String>, sample_scene: Option<u32>) -> Scene {
    // let metal_mat = Arc::new(Metal::new(Color::new(0.45, 0.71, 0.95), 0.2));
    if let Some(mesh_path) = mesh_path {
        let grey_mat = Arc::new(Lambertian::new(Color::new(0.25, 0.25, 0.25)));
        let mut objects = World::default();
        let triangle_mesh_opt = load_mesh(Path::new(&mesh_path), grey_mat);
        objects.add(Arc::new(triangle_mesh_opt.unwrap()));
        get_cornell_box_scene(objects)
    } else {
        match sample_scene.unwrap() {
            0 => {
                let white_mat = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
                let mut objects = World::default();
                let triangle_mesh_opt = load_mesh(
                    Path::new("./sample_meshes/cornell_box_objects.obj"),
                    white_mat,
                );
                objects.add(Arc::new(triangle_mesh_opt.unwrap()));
                get_cornell_box_scene(objects)
            }
            1 => {
                let metal_mat = Arc::new(Metal::new(Color::new(0.45, 0.71, 0.95), 0.2));
                let blue_mat = Arc::new(Lambertian::new(Color::new(0.45, 0.71, 0.95)));
                let mut objects = World::default();
                let triangle_mesh_opt =
                    load_mesh(Path::new("./sample_meshes/tachikoma.obj"), blue_mat);
                objects.add(Arc::new(triangle_mesh_opt.unwrap()));
                get_cornell_box_scene(objects)
            }
            2 => get_random_spheres_scene(),
            _ => panic!("Invalid sample-scene value used. Use one of [0, 1, 2]"),
        }
    }
}
