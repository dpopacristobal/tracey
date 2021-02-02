use crate::linalg::Vec3;

pub use cosine::CosinePDF;

pub mod cosine;

pub trait PDF: Send + Sync {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
