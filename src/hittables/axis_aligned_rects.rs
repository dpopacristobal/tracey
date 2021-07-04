use std::sync::Arc;

use rand::Rng;

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

pub struct XZRect {
    x_min: f64,
    x_max: f64,
    z_min: f64,
    z_max: f64,
    y: f64,
    material: Arc<dyn Material>,
}

impl XZRect {
    pub fn new(
        x_min: f64,
        x_max: f64,
        z_min: f64,
        z_max: f64,
        y: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            x_min,
            x_max,
            z_min,
            z_max,
            y,
            material,
        }
    }
}

impl Hit for XZRect {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.y - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x() + t * ray.direction().x();
        let z = ray.origin().z() + t * ray.direction().z();
        if x < self.x_min || x > self.x_max || z < self.z_min || z > self.z_max {
            return None;
        }

        let mut hit_record = HitRecord::new(
            Point3::new(x, self.y, z),
            Vec3::default(),
            self.material.clone(),
            t,
            false,
        );
        hit_record.set_face_normal(ray, Vec3::new(0.0, 1.0, 0.0));
        Some(hit_record)
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            Point3::new(self.x_min, self.y - 0.0001, self.z_min),
            Point3::new(self.x_max, self.y + 0.0001, self.z_max),
        ))
    }

    fn pdf_value(&self, hit_point: Point3, direction: Vec3) -> f64 {
        if let Some(hit_record) =
            self.hit(Ray::new(hit_point, direction), 0.001, std::f64::INFINITY)
        {
            let area = (self.x_max - self.x_min) * (self.z_max - self.z_min);
            let distance_sq = hit_record.t * hit_record.t * direction.length_sq();
            let cos_theta = direction.dot(hit_record.normal).abs() / direction.length();

            distance_sq / (cos_theta * area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let rand_point = Point3::new(
            rng.gen_range(self.x_min, self.x_max),
            self.y,
            rng.gen_range(self.z_min, self.z_max),
        );

        rand_point - origin
    }
}

pub struct YZRect {
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
    x: f64,
    material: Arc<dyn Material>,
}

impl YZRect {
    pub fn new(
        y_min: f64,
        y_max: f64,
        z_min: f64,
        z_max: f64,
        x: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            y_min,
            y_max,
            z_min,
            z_max,
            x,
            material,
        }
    }
}

impl Hit for YZRect {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.x - ray.origin().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin().y() + t * ray.direction().y();
        let z = ray.origin().z() + t * ray.direction().z();
        if y < self.y_min || y > self.y_max || z < self.z_min || z > self.z_max {
            return None;
        }

        let mut hit_record = HitRecord::new(
            Point3::new(self.x, y, z),
            Vec3::default(),
            self.material.clone(),
            t,
            false,
        );
        hit_record.set_face_normal(ray, Vec3::new(1.0, 0.0, 0.0));
        Some(hit_record)
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            Point3::new(self.x - 0.0001, self.y_min, self.z_min),
            Point3::new(self.x + 0.0001, self.y_max, self.z_max),
        ))
    }
}
