use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

extern crate rey_skytracer;

use rey_skytracer::hittables::{Sphere, World};
use rey_skytracer::linalg::Color;
use rey_skytracer::linalg::Point3;
use rey_skytracer::materials::{Dielectric, Lambertian, Metal};
use rey_skytracer::render::render;

pub fn gen_benchmark_sphere_scene() -> World {
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

    world
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render sphere scene", |b| {
        b.iter(|| render(&gen_benchmark_sphere_scene(), 1280, 20))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
