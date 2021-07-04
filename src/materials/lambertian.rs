use crate::hittables::HitRecord;
use crate::linalg::{Color, Ray};
use crate::materials::Material;
use crate::pdfs::CosinePDF;

use super::ScatterRecord;

use std::sync::Arc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new(
            None,
            Some(Arc::new(CosinePDF::new(hit_record.normal))),
            self.albedo,
        ))
    }

    fn scattering_pdf(&self, _ray_in: Ray, ray_scattered: Ray, hit_record: &HitRecord) -> f64 {
        let cos_theta = hit_record
            .normal
            .dot(ray_scattered.direction().into_unit_vec());
        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / std::f64::consts::PI
        }
    }
}
