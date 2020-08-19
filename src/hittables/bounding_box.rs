use crate::linalg::ray::Ray;
use crate::linalg::vec3::Point3;

use super::{Hit, HitRecord};

pub struct BoundingBox {
    min_corner: Point3,
    max_corner: Point3,
}

impl BoundingBox {
    fn new(min_corner: Point3, max_corner: Point3) -> Self {
        Self {
            min_corner,
            max_corner,
        }
    }
}

impl Hit for BoundingBox {
    fn hit(&self, ray: Ray, mut t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        for idx in 0..3 {
            let first_solution = (self.min_corner[idx] - ray.origin()[idx]) / ray.direction()[idx];
            let second_solution = (self.min_corner[idx] - ray.origin()[idx]) / ray.direction()[idx];
            let t_min_local = first_solution.min(second_solution);
            let t_max_local = first_solution.max(second_solution);

            t_min = t_min.max(t_min_local);
            t_max = t_max.min(t_max_local);
            if t_max <= t_min {
                return None;
            }
        }

        Some(HitRecord::default())
    }
}
