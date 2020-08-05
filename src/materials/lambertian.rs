use crate::hittables::common::HitRecord;
use crate::linalg::color::Color;
use crate::linalg::ray::Ray;
use crate::linalg::vec3::Vec3;
use crate::materials::common::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color) {
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        let scattered_ray = Ray::new(hit_record.hit_point, scatter_direction);

        (Some(scattered_ray), self.albedo)
    }
}
