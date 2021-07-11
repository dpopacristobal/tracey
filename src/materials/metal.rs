use crate::hittables::HitRecord;
use crate::linalg::{Color, Ray, Vec3};
use crate::materials::Material;

use super::{reflect, ScatterRecord};

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
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected_direction = reflect(ray_in.direction().into_unit_vec(), hit_record.normal);
        let reflected_ray = Ray::new(
            hit_record.hit_point,
            reflected_direction + Vec3::random_in_unit_sphere().mul_scalar(self.fuzz_factor),
        );

        // TODO(dpopacristobal): There is probably a much neater way of doing this...
        let mut ret: Option<ScatterRecord> = None;
        if reflected_ray.direction().dot(hit_record.normal) > 0.0 {
            ret = Some(ScatterRecord::new(Some(reflected_ray), None, self.albedo));
        }

        ret
    }
}
