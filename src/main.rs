extern crate tracey;

use std::path::Path;
use std::sync::Arc;

use tracey::hittables::World;
use tracey::linalg::Color;
use tracey::load_mesh::load_mesh;
use tracey::materials::Lambertian;
use tracey::render::render;
use tracey::scene::{get_cornell_box_scene, get_random_spheres_scene, Scene};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "tracey", about = "A simple CLI to render scenes using tracey.")]
struct TraceyArgs {
    /// Width of the output image in pixels
    #[structopt(long)]
    width: u32,

    /// Number of rays used per pixel
    #[structopt(long)]
    rays: u32,

    /// Path to .obj mesh that will be put into a Cornell Box scene
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
    if let Some(mesh_path) = mesh_path {
        let grey_mat = Arc::new(Lambertian::new(Color::new(0.25, 0.25, 0.25)));
        let mut objects = World::default();
        let triangle_mesh_opt = load_mesh(Path::new(&mesh_path), grey_mat);
        objects.add(Arc::new(triangle_mesh_opt));
        get_cornell_box_scene(objects)
    } else {
        match sample_scene.unwrap() {
            0 => {
                let white_mat = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
                let mut objects = World::default();
                let box_objects_mesh = load_mesh(
                    Path::new("./sample_meshes/cornell_box_objects.obj"),
                    white_mat,
                );
                objects.add(Arc::new(box_objects_mesh));
                get_cornell_box_scene(objects)
            }
            1 => {
                let white_mat = Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0)));
                let tachikoma_mat = Arc::new(Lambertian::new(Color::new(0.45, 0.71, 0.95)));
                let tie_fighter_mat = Arc::new(Lambertian::new(Color::new(0.196, 0.18, 0.176)));
                let monkey_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.255, 0.0)));

                let mut objects = World::default();

                let box_objects_mesh = load_mesh(
                    Path::new("./sample_meshes/cornell_box_objects.obj"),
                    white_mat,
                );
                objects.add(Arc::new(box_objects_mesh));

                let tachikoma_mesh = load_mesh(
                    Path::new("./sample_meshes/tachikoma_small.obj"),
                    tachikoma_mat,
                );
                objects.add(Arc::new(tachikoma_mesh));

                let tie_fighter_mesh = load_mesh(
                    Path::new("./sample_meshes/tie_fighter.obj"),
                    tie_fighter_mat,
                );
                objects.add(Arc::new(tie_fighter_mesh));

                let monkey_mesh =
                    load_mesh(Path::new("./sample_meshes/blender_monkey.obj"), monkey_mat);
                objects.add(Arc::new(monkey_mesh));

                get_cornell_box_scene(objects)
            }
            2 => get_random_spheres_scene(),
            _ => panic!("Invalid sample-scene value used. Use one of [0, 1, 2]"),
        }
    }
}
