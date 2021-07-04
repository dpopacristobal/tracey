use crate::linalg::Vec3;

pub use cosine::CosinePDF;
pub use hittable::HittablePDF;
pub use mixture::MixturePDF;

pub mod cosine;
pub mod hittable;
pub mod mixture;

pub trait PDF: Send + Sync {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
