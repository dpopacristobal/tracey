pub use bounding_box::BoundingBox;
pub use bvh_node::BvhNode;
pub use sphere::Sphere;
pub use world::World;

mod bounding_box;
pub mod bvh_node;
pub mod sphere;
pub mod world;

use std::sync::Arc;

use crate::linalg::Ray;
use crate::linalg::{Point3, Vec3};
use crate::materials::{DefaultMaterial, Material};

#[derive(Clone)]
pub struct HitRecord {
    pub hit_point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hit: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t_min: f64, t_max: f64) -> Option<BoundingBox>;
}

impl HitRecord {
    pub fn new(
        hit_point: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Self {
        Self {
            hit_point,
            normal,
            material,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = outward_normal.mul_scalar(-1.0);
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            hit_point: Point3::default(),
            normal: Vec3::default(),
            material: Arc::new(DefaultMaterial::default()),
            t: 0.0,
            front_face: false,
        }
    }
}
