use rand::Rng;

use crate::hittables::HitRecord;
use crate::linalg::{Color, Ray};
use crate::materials::Material;

use super::{reflect, refract, schlick};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color) {
        let attenuation = Color::from_scalar(1.0);
        let refractive_index_ratio = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray_in.direction().into_unit_vec();

        let cos_theta = (-unit_direction.dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let reflect_prob = schlick(cos_theta, refractive_index_ratio);
        let mut rng = rand::thread_rng();
        let direction =
            if refractive_index_ratio * sin_theta > 1.0 || rng.gen_range(0.0, 1.0) < reflect_prob {
                reflect(unit_direction, hit_record.normal)
            } else {
                refract(unit_direction, hit_record.normal, refractive_index_ratio)
            };

        let refracted_ray = Ray::new(hit_record.hit_point, direction);

        (Some(refracted_ray), attenuation)
    }
}
