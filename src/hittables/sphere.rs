use std::sync::Arc;

use crate::linalg::ray::Ray;
use crate::linalg::vec3::{Point3, Vec3};
use crate::materials::Material;

use super::{Hit, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = HitRecord::new(
            Point3::default(),
            Vec3::default(),
            self.material.clone(),
            0.0,
            false,
        );

        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_sq();
        let half_b = oc.dot(*ray.direction());
        let c = oc.length_sq() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if (temp < t_max) && (temp > t_min) {
                hit_record.t = temp;
                hit_record.hit_point = ray.at(hit_record.t);
                let outward_normal = (hit_record.hit_point - self.center).div_scalar(self.radius);
                hit_record.set_face_normal(ray, outward_normal);
                return Some(hit_record);
            }
            temp = (-half_b + root) / a;
            if (temp < t_max) && (temp > t_min) {
                hit_record.t = temp;
                hit_record.hit_point = ray.at(hit_record.t);
                let outward_normal = (hit_record.hit_point - self.center).div_scalar(self.radius);
                hit_record.set_face_normal(ray, outward_normal);
                return Some(hit_record);
            }
        }

        None
    }
}
