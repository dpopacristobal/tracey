use crate::hittables::common::HitRecord;
use crate::linalg::color::Color;
use crate::linalg::ray::Ray;

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color);
}
