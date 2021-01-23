use std::path::Path;
use std::sync::Arc;

use rand::Rng;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hittables::{BvhNode, Hit, Sphere, Triangle, World, XYRect, XZRect, YZRect};
use crate::linalg::{Color, Point3, Ray, Vec3};
use crate::load_mesh::load_mesh;
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
    let mut hittable_list = World::default();

    let red_mat = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white_mat = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green_mat = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let blue_mat = Arc::new(Lambertian::new(Color::new(0.45, 0.71, 0.95)));
    let light_mat = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));
    let metal_mat = Arc::new(Metal::new(Color::new(0.45, 0.71, 0.95), 0.2));

    hittable_list.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        red_mat.clone(),
    )));
    hittable_list.add(Arc::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, green_mat,
    )));
    hittable_list.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light_mat,
    )));

    hittable_list.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white_mat.clone(),
    )));

    // Top
    hittable_list.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white_mat.clone(),
    )));

    // Back
    hittable_list.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white_mat.clone(),
    )));

    let triangle_mesh_opt = load_mesh(Path::new("./sample_meshes/tachikoma_3.obj"), metal_mat);
    if let Some(triangle_mesh) = triangle_mesh_opt {
        hittable_list.add(Arc::new(triangle_mesh));
    }

    let bvh_node = BvhNode::from_world(&mut hittable_list, 0.0, 1.0);
    let mut world = World::default();
    world.add(Arc::new(bvh_node));

    world
}

pub fn render(world: &World, image_width: u32, samples_per_pixel: i32) {
    // let aspect_ratio = 16.0 / 9.0;
    let aspect_ratio = 1.0;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let max_depth = 50;

    let mut image_buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    // Camera
    let look_from = Point3::new(277.5, 277.5, -800.0);
    let look_at = Point3::new(277.5, 277.5, 0.0);
    let up_direction = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up_direction,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let background = Color::new(0.0, 0.0, 0.0);

    // Render
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
    image_buffer.save("out/rendered_image.png").unwrap();
}
