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
    fn scatter(&self, _ray_in: Ray, _hit_record: &HitRecord) -> (Option<Ray>, Color) {
        (None, Color::default())
    }

    fn emit(&self, _u: f64, _v: f64, _hit_point: Point3) -> Color {
        self.color
    }
}
