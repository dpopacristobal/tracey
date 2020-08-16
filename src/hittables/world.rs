use std::sync::Arc;

use crate::linalg::ray::Ray;

use super::{Hit, HitRecord};

#[derive(Default)]
pub struct World {
    objects: Vec<Arc<dyn Hit>>,
}

impl World {
    pub fn from_hittable(object: Arc<dyn Hit>) -> Self {
        Self {
            objects: vec![object.clone()],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hit>) {
        self.objects.push(object);
    }
}

impl Hit for World {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ret: Option<HitRecord> = None;

        let mut closest_so_far = t_max;
        for object in &self.objects {
            let hit_result = object.hit(ray, t_min, closest_so_far);
            if let Some(hit_record) = hit_result {
                closest_so_far = hit_record.t;
                ret = Some(hit_record);
            }
        }

        ret
    }
}
