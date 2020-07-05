extern crate rey_skytracer;

use rey_skytracer::linalg::color::Color;
use rey_skytracer::linalg::ray::Ray;
use rey_skytracer::linalg::vec3::{Point3, Vec3};

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().into_unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::from_scalar(1.0).mul_scalar(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul_scalar(t)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Point3::from_scalar(0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - horizontal.div_scalar(2.0)
        - vertical.div_scalar(2.0)
        - Vec3::new(0.0, 0.0, focal_length);

    for i in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", i);
        for j in 0..image_width {
            let u: f64 = j as f64 / (image_width - 1) as f64;
            let v: f64 = i as f64 / (image_height - 1) as f64;

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal.mul_scalar(u) + vertical.mul_scalar(v) - origin,
            );
            let color = ray_color(&ray);
            println!("{}", color);
        }
    }
    eprintln!("Done")
}
