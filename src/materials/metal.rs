use crate::hittables::common::HitRecord;
use crate::linalg::color::Color;
use crate::linalg::ray::Ray;
use crate::linalg::vec3::Vec3;
use crate::materials::common::Material;

use super::utils::reflect;

pub struct Metal {
    albedo: Color,
    fuzz_factor: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz_factor: f64) -> Self {
        Self {
            albedo,
            fuzz_factor,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color) {
        let reflected_direction = reflect(ray_in.direction().into_unit_vec(), hit_record.normal);
        let reflected_ray = Ray::new(
            hit_record.hit_point,
            reflected_direction + Vec3::random_in_unit_sphere().mul_scalar(self.fuzz_factor),
        );

        // This is probably not how you do this and there is a much neater way
        let mut ret: Option<Ray> = None;
        if reflected_ray.direction().dot(hit_record.normal) > 0.0 {
            ret = Some(reflected_ray);
        }

        (ret, self.albedo)
    }
}
