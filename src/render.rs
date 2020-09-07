use std::sync::Arc;

use rand::Rng;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hittables::{BvhNode, Hit, Sphere, World, XYRect};
use crate::linalg::{Color, Point3, Ray, Vec3};
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Metal};

fn ray_color(ray: Ray, background: Color, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    let hit_result = world.hit(ray, 0.001, f64::INFINITY);
    if let Some(hit_record) = hit_result {
        let emitted_color = hit_record.material.emit(0.0, 0.0, hit_record.hit_point);
        let (scatter_result, attenuation) = hit_record.material.scatter(ray, &hit_record);
        if let Some(scatter_ray) = scatter_result {
            emitted_color + attenuation * ray_color(scatter_ray, background, world, depth - 1)
        } else {
            emitted_color
        }
    } else {
        background
    }
}

pub fn gen_random_scene() -> World {
    let mut world = World::default();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));
    world.add(ground_sphere);

    let mut rng = rand::thread_rng();
    for a in -5..5 {
        for b in -5..5 {
            let material_choice_prob = rng.gen_range(0.0, 1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0, 1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let lambertian_mat = Arc::new(Lambertian::new(Color::random_from_bounds(0.0, 1.0)));
                let metal_mat = Arc::new(Metal::new(
                    Color::random_from_bounds(0.5, 1.0),
                    rng.gen_range(0.0, 0.5),
                ));
                let dielectric_mat = Arc::new(Dielectric::new(1.5));

                // Lambertian sphere
                if material_choice_prob < 0.6 {
                    world.add(Arc::new(Sphere::new(center, 0.2, lambertian_mat)));
                }
                // Metal sphere
                else if material_choice_prob < 0.9 {
                    world.add(Arc::new(Sphere::new(center, 0.2, metal_mat)));
                }
                // Dielectric sphere
                else {
                    world.add(Arc::new(Sphere::new(center, 0.2, dielectric_mat)));
                };
            }
        }
    }

    let lambertian_mat = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        lambertian_mat,
    )));

    let metal_mat = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        metal_mat,
    )));

    let dielectric_mat = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        dielectric_mat,
    )));

    let diffuse_light_mat = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(XYRect::new(
        0.0,
        2.0,
        0.0,
        2.0,
        0.0,
        diffuse_light_mat,
    )));

    let bvh_node = BvhNode::from_world(&mut world, 0.0, 1.0);
    let mut world = World::default();
    world.add(Arc::new(bvh_node));

    world
}

pub fn render(world: &World, image_width: u32, samples_per_pixel: i32) {
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let max_depth = 50;

    let mut image_buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

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

    let background = Color::new(0.0, 0.0, 0.0);

    // Render
    // TODO(dnlpc): Explain better what this is actually doing.
    let mut pixels: Vec<(u32, u32, &mut image::Rgb<u8>)> = Vec::new();
    pixels.reserve((image_width * image_height) as usize);
    for (i, j, pixel) in image_buffer.enumerate_pixels_mut() {
        pixels.push((i, j, pixel));
    }

    pixels.par_iter_mut().for_each(|(i, j, pixel)| {
        let mut pixel_color_accumulator = Color::default();
        for _ in 0..samples_per_pixel {
            let mut rng = rand::thread_rng();
            let u = (*i as f64 + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f64;
            let v =
                ((image_height - *j) as f64 + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f64;
            let ray = camera.get_ray(u, v);
            pixel_color_accumulator
                .accumulate_sample(ray_color(ray, background, &world, max_depth));
        }

        let pixel_color: Color = pixel_color_accumulator.average_samples(samples_per_pixel);
        **pixel = image::Rgb(pixel_color.gamma_2_correct().into_rgb8());
    });

    // Output Image
    image_buffer.save("rendered_image.png").unwrap();
}
