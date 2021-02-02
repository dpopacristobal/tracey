use crate::linalg::{Vec3, ONB};

use super::PDF;

pub struct CosinePDF {
    onb: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        let onb = ONB::new(w);
        Self { onb }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cos_theta = direction.into_unit_vec().dot(self.onb.w());

        if cos_theta <= 0.0 {
            0.0
        } else {
            cos_theta / std::f64::consts::PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.onb.local(Vec3::random_cosine_dir())
    }
}
