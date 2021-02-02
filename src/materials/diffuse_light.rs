use crate::hittables::HitRecord;
use crate::linalg::{Color, Point3, Ray};
use crate::materials::Material;

pub struct DiffuseLight {
    color: Color,
}

impl DiffuseLight {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for DiffuseLight {
    fn emit(&self, _u: f64, _v: f64, hit_record: &mut HitRecord) -> Color {
        if hit_record.front_face {
            self.color
        } else {
            Color::default()
        }
    }
}
