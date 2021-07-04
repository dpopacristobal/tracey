extern crate rey_skytracer;

use rey_skytracer::render::{gen_random_scene, render};

fn main() {
    let (world, light) = gen_random_scene();
    render(&world, light, 720, 100);
}
