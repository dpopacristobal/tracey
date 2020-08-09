use std::rc::Rc;

use rand::Rng;

use crate::camera::Camera;
use crate::hittables::{Hit, Sphere, World};
use crate::linalg::Color;
use crate::linalg::Ray;
use crate::linalg::{Point3, Vec3};
use crate::materials::{Dielectric, Lambertian, Metal};

fn ray_color(ray: Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    let hit_result = world.hit(ray, 0.001, f64::INFINITY);
    if let Some(hit_record) = hit_result {
        let (scatter_result, attenuation) = hit_record.material.scatter(ray, &hit_record);
        if let Some(scatter_ray) = scatter_result {
            return attenuation * ray_color(scatter_ray, world, depth - 1);
        }

        return Color::default();
    }

    let unit_direction = ray.direction().into_unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::from_scalar(1.0).mul_scalar(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul_scalar(t)
}

fn gen_random_scene() -> World {
    let mut world = World::default();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));
    world.add(ground_sphere);

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let material_choice_prob = rng.gen_range(0.0, 1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0, 1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let lambertian_mat = Rc::new(Lambertian::new(Color::random_from_bounds(0.0, 1.0)));
                let metal_mat = Rc::new(Metal::new(
                    Color::random_from_bounds(0.5, 1.0),
                    rng.gen_range(0.0, 0.5),
                ));
                let dielectric_mat = Rc::new(Dielectric::new(1.5));

                // Lambertian sphere
                if material_choice_prob < 0.6 {
                    world.add(Rc::new(Sphere::new(center, 0.2, lambertian_mat)));
                }
                // Metal sphere
                else if material_choice_prob < 0.9 {
                    world.add(Rc::new(Sphere::new(center, 0.2, metal_mat)));
                }
                // Dielectric sphere
                else {
                    world.add(Rc::new(Sphere::new(center, 0.2, dielectric_mat)));
                };
            }
        }
    }

    let lambertian_mat = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        lambertian_mat,
    )));

    let metal_mat = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        metal_mat,
    )));

    let dielectric_mat = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        dielectric_mat,
    )));

    world
}

pub fn render() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 1;
    let max_depth = 50;

    let mut image_buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    // World
    let world = gen_random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let up_direction = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up_direction,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    let mut rng = rand::thread_rng();
    for (i, j, pixel) in image_buffer.enumerate_pixels_mut() {
        let mut pixel_color_accumulator = Color::default();
        for _ in 0..samples_per_pixel {
            let u = (i as f64 + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f64;
            let v =
                ((image_height - j) as f64 + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f64;
            let ray = camera.get_ray(u, v);
            pixel_color_accumulator.accumulate_sample(ray_color(ray, &world, max_depth));
        }

        let pixel_color: Color = pixel_color_accumulator.average_samples(samples_per_pixel);
        *pixel = image::Rgb(pixel_color.gamma_2_correct().into_rgb8());
    }

    // Output Image
    image_buffer.save("rendered_image.png").unwrap();
}
