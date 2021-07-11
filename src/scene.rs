use std::sync::Arc;

use rand::Rng;

use crate::camera::Camera;
use crate::hittables::{BvhNode, FlipFace, Hit, Sphere, World, XYRect, XZRect, YZRect};
use crate::linalg::{Color, Point3, Vec3};
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Material, Metal};

pub struct Scene {
    pub world: World,
    pub light: Option<Arc<dyn Hit>>,
    pub camera: Camera,
    pub aspect_ratio: f64,
    pub background: Color,
}

impl Scene {
    pub fn new(
        world: World,
        light: Option<Arc<dyn Hit>>,
        camera: Camera,
        aspect_ratio: f64,
        background: Color,
    ) -> Self {
        Self {
            world,
            light,
            camera,
            aspect_ratio,
            background,
        }
    }
}

pub fn get_cornell_box_scene(objects: World) -> Scene {
    let aspect_ratio = 1.0;

    // Set up the camera.
    let look_from = Point3::new(277.5, 277.5, -800.0);
    let look_at = Point3::new(277.5, 277.5, 0.0);
    let up_direction = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        look_from,
        look_at,
        up_direction,
        40.0,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    let background = Color::new(0.0, 0.0, 0.0);

    let mut hittable_list = World::default();
    for object in objects.objects() {
        hittable_list.add(object.clone());
    }

    let red_mat = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white_mat = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green_mat = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light_mat = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    // Light that illuminates the Cornell box.
    hittable_list.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light_mat.clone(),
    )))));

    // Left wall of the Cornell box.
    hittable_list.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        red_mat.clone(),
    )));

    // Right wall of the Cornell box.
    hittable_list.add(Arc::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, green_mat,
    )));

    // Floor of the Cornell box.
    hittable_list.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white_mat.clone(),
    )));

    // Ceiling of the Cornell box.
    hittable_list.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white_mat.clone(),
    )));

    // Back wall of the Cornell box.
    hittable_list.add(Arc::new(XYRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, white_mat,
    )));

    let bvh_node = BvhNode::from_world(&mut hittable_list, 0.0, 1.0);
    let mut world = World::default();
    world.add(Arc::new(bvh_node));

    let light: Option<Arc<dyn Hit>> = Some(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light_mat,
    )));

    Scene::new(world, light, camera, aspect_ratio, background)
}

pub fn get_random_spheres_scene() -> Scene {
    let aspect_ratio = 1.5;

    // Set up the camera.
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let up_direction = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        up_direction,
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    let background = Color::new(0.7, 0.8, 1.0);

    let mut hittable_list = World::default();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    )));

    let mut rng = rand::thread_rng();

    // Create a large number of small random spheres.
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0, 1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_mat: Arc<dyn Material> = if choose_mat < 0.8 {
                    // Create a random lambertian sphere.
                    let color =
                        Color::random_from_bounds(0.0, 1.0) * Color::random_from_bounds(0.0, 1.0);
                    Arc::new(Lambertian::new(color))
                } else if choose_mat < 0.95 {
                    // Create a random metal sphere.
                    let color = Color::random_from_bounds(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    Arc::new(Metal::new(color, fuzz))
                } else {
                    // Create a random dielectric sphere.
                    Arc::new(Dielectric::new(1.5))
                };

                hittable_list.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
            }
        }
    }

    // Create a large lambertian sphere in a fixed position.
    let lambertian_mat = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        lambertian_mat,
    )));

    // Create a large dielectric sphere in a fixed position.
    let dielectric_mat = Arc::new(Dielectric::new(1.5));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        dielectric_mat,
    )));

    // Create a large metal sphere in a fixed position.
    let metal_mat = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    hittable_list.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        metal_mat,
    )));

    let bvh_node = BvhNode::from_world(&mut hittable_list, 0.0, 1.0);
    let mut world = World::default();
    world.add(Arc::new(bvh_node));

    Scene::new(world, None, camera, aspect_ratio, background)
}
