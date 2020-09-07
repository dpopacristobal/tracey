use std::sync::Arc;

use crate::linalg::{Point3, Ray, Vec3};
use crate::materials::Material;

use super::{BoundingBox, Hit, HitRecord};

pub struct XYRect {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z: f64,
    material: Arc<dyn Material>,
}

impl XYRect {
    pub fn new(
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        z: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
            z,
            material,
        }
    }
}

impl Hit for XYRect {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.z - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let y = ray.origin().y() + t * ray.direction().y();
        if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
            return None;
        }

        let mut hit_record = HitRecord::new(
            Point3::new(x, y, self.z),
            Vec3::default(),
            self.material.clone(),
            t,
            false,
        );
        hit_record.set_face_normal(ray, Vec3::new(0.0, 0.0, 1.0));
        Some(hit_record)
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            Point3::new(self.x_min, self.y_min, self.z - 0.0001),
            Point3::new(self.x_max, self.y_max, self.z + 0.0001),
        ))
    }
}
