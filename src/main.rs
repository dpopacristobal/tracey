extern crate rey_skytracer;

use rey_skytracer::linalg::color::Color;
use rey_skytracer::linalg::vec3::{Point3, Vec3};

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let color = Color::new(
                j as f64 / (image_width - 1) as f64,
                i as f64 / (image_height - 1) as f64,
                0.25,
            );

            println!("{}", color);
        }
    }
}
