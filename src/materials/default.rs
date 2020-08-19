use crate::hittables::HitRecord;
use crate::linalg::color::Color;
use crate::linalg::ray::Ray;
use crate::materials::Material;

pub struct DefaultMaterial {}

impl Material for DefaultMaterial {
    fn scatter(&self, _ray_in: Ray, _hit_record: &HitRecord) -> (Option<Ray>, Color) {
        (None, Color::default())
    }
}

impl Default for DefaultMaterial {
    fn default() -> Self {
        Self {}
    }
}
