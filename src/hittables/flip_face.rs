use std::sync::Arc;

use crate::linalg::Ray;

use super::{BoundingBox, Hit, HitRecord};

pub struct FlipFace {
    hittable: Arc<dyn Hit>,
}

impl FlipFace {
    pub fn new(hittable: Arc<dyn Hit>) -> Self {
        Self { hittable }
    }
}

impl Hit for FlipFace {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut hit_record) = self.hittable.hit(ray, t_min, t_max) {
            hit_record.front_face = !hit_record.front_face;
            Some(hit_record)
        } else {
            None
        }
    }
    fn bounding_box(&self, t_min: f64, t_max: f64) -> Option<BoundingBox> {
        self.hittable.bounding_box(t_min, t_max)
    }
}
