use std::sync::Arc;

use super::PDF;

use crate::hittables::Hit;
use crate::linalg::{Point3, Vec3};

pub struct HittablePDF {
    hittable: Arc<dyn Hit>,
    origin: Point3,
}

impl HittablePDF {
    pub fn new(hittable: Arc<dyn Hit>, origin: Point3) -> Self {
        Self { hittable, origin }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: Vec3) -> f64 {
        self.hittable.pdf_value(self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.hittable.random(self.origin)
    }
}
