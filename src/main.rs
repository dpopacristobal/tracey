extern crate rey_skytracer;

use rey_skytracer::render::{gen_random_scene, render};

fn main() {
    render(&gen_random_scene(), 720, 500);
}
