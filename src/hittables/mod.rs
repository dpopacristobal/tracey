pub use sphere::Sphere;
pub use world::World;

pub mod sphere;
pub mod world;

use std::rc::Rc;

use crate::linalg::Ray;
use crate::linalg::{Point3, Vec3};
use crate::materials::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub hit_point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hit {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(
        hit_point: Point3,
        normal: Vec3,
        material: Rc<dyn Material>,
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
