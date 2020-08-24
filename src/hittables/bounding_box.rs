use crate::linalg::ray::Ray;
use crate::linalg::vec3::Point3;

use super::{Hit, HitRecord};

#[derive(Clone)]
pub struct BoundingBox {
    min_corner: Point3,
    max_corner: Point3,
}

impl BoundingBox {
    pub fn new(min_corner: Point3, max_corner: Point3) -> Self {
        Self {
            min_corner,
            max_corner,
        }
    }

    pub fn combine(&self, other: &BoundingBox) -> Self {
        let min_corner = Point3::new(
            self.min_corner.x().min(other.min_corner.x()),
            self.min_corner.y().min(other.min_corner.y()),
            self.min_corner.z().min(other.min_corner.z()),
        );
        let max_corner = Point3::new(
            self.max_corner.x().max(other.max_corner.x()),
            self.max_corner.y().max(other.max_corner.y()),
            self.max_corner.z().max(other.max_corner.z()),
        );

        Self {
            min_corner,
            max_corner,
        }
    }

    pub fn min_corner(&self) -> Point3 {
        self.min_corner
    }

    pub fn max_corner(&self) -> Point3 {
        self.max_corner
    }
}

impl Hit for BoundingBox {
    fn hit(&self, ray: Ray, mut t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        for idx in 0..3 {
            let inv_direction = 1.0 / ray.direction()[idx];
            let mut t_min_local = (self.min_corner[idx] - ray.origin()[idx]) * inv_direction;
            let mut t_max_local = (self.max_corner[idx] - ray.origin()[idx]) * inv_direction;

            if inv_direction < 0.0 {
                std::mem::swap(&mut t_min_local, &mut t_max_local);
            }

            t_min = t_min.max(t_min_local);
            t_max = t_max.min(t_max_local);
            if t_max <= t_min {
                return None;
            }
        }

        Some(HitRecord::default())
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64) -> Option<Self> {
        Some(self.clone())
    }
}
