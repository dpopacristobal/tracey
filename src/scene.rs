use std::path::Path;
use std::sync::Arc;

use crate::camera::Camera;
use crate::hittables::{BvhNode, FlipFace, Hit, World, XYRect, XZRect, YZRect};
use crate::linalg::{Color, Point3, Ray, Vec3};
use crate::load_mesh::load_mesh;
use crate::materials::{DiffuseLight, Lambertian, Metal};

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

    let red_mat = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white_mat = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green_mat = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let blue_mat = Arc::new(Lambertian::new(Color::new(0.45, 0.71, 0.95)));
    let light_mat = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));
    let metal_mat = Arc::new(Metal::new(Color::new(0.45, 0.71, 0.95), 0.2));
    let mut hittable_list = World::default();
    for object in objects.objects() {
        hittable_list.add(object.clone());
    }

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

    hittable_list.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light_mat.clone(),
    )))));

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
