use std::sync::Arc;

use rand::Rng;

use super::PDF;

use crate::linalg::Vec3;

pub struct MixturePDF {
    pdfs: [Arc<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(pdfs: [Arc<dyn PDF>; 2]) -> Self {
        Self { pdfs }
    }
}

// TODO(dpopacristobal): Should the user be allowed to specify the weight assigned to each PDF?
impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.pdfs[0].value(direction) + 0.5 * self.pdfs[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let rand: f64 = rng.gen_range(0.0, 1.0);
        if rand < 0.5 {
            self.pdfs[0].generate()
        } else {
            self.pdfs[1].generate()
        }
    }
}
