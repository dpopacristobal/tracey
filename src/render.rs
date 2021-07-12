use std::fs;
use std::sync::Arc;

use rand::Rng;
use rayon::prelude::*;

use crate::hittables::{Hit, World};
use crate::linalg::{Color, Ray};
use crate::pdfs::{HittablePDF, MixturePDF, PDF};
use crate::scene::Scene;

fn ray_color(
    ray: Ray,
    background: Color,
    world: &World,
    light: Option<Arc<dyn Hit>>,
    depth: i32,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    let hit_result = world.hit(ray, 0.001, f64::INFINITY);
    if let Some(mut hit_record) = hit_result {
        let material = hit_record.material.clone();
        let emitted_color = material.emit(0.0, 0.0, &mut hit_record);
        let scatter_record_opt = hit_record.material.scatter(ray, &hit_record);
        if let Some(scatter_record) = scatter_record_opt {
            if let Some(specular_ray) = scatter_record.specular_ray {
                scatter_record.attenuation
                    * ray_color(specular_ray, background, world, light, depth - 1)
            } else {
                let (scatter_ray, pdf_val) = if let Some(light) = light.as_ref() {
                    let light_pdf: Arc<dyn PDF> =
                        Arc::new(HittablePDF::new(light.clone(), hit_record.hit_point));
                    let mixture_pdf = MixturePDF::new([light_pdf, scatter_record.pdf.unwrap()]);

                    let scatter_ray = Ray::new(hit_record.hit_point, mixture_pdf.generate());
                    let pdf_val = mixture_pdf.value(*scatter_ray.direction());

                    (scatter_ray, pdf_val)
                } else {
                    let pdf = scatter_record.pdf.unwrap();
                    let scatter_ray = Ray::new(hit_record.hit_point, pdf.generate());
                    let pdf_val = pdf.value(*scatter_ray.direction());

                    (scatter_ray, pdf_val)
                };
                emitted_color
                    + scatter_record.attenuation.mul_scalar(
                        hit_record
                            .material
                            .scattering_pdf(ray, scatter_ray, &hit_record)
                            / pdf_val,
                    ) * ray_color(scatter_ray, background, world, light, depth - 1)
            }
        } else {
            emitted_color
        }
    } else {
        background
    }
}

pub fn render(image_width: u32, samples_per_pixel: u32, scene: Scene) {
    // TODO(dpopacristobal): Would it be worth exposing this in the CLI?
    let max_depth = 20;
    let image_height = (image_width as f64 / scene.aspect_ratio) as u32;

    let mut image_buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    // Render the scene.
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
            let ray = scene.camera.get_ray(u, v);
            pixel_color_accumulator.accumulate_sample(ray_color(
                ray,
                scene.background,
                &scene.world,
                scene.light.clone(),
                max_depth,
            ));
        }

        let pixel_color: Color = pixel_color_accumulator.average_samples(samples_per_pixel);
        **pixel = image::Rgb(pixel_color.gamma_2_correct().into_rgb8());
    });

    // Output the rendered image to .png.
    fs::create_dir_all("out")
        .expect("Output directory does not exist and failed trying to create it");
    image_buffer.save("out/rendered_image.png").unwrap();
}
