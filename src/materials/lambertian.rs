use crate::hittables::HitRecord;
use crate::linalg::{Color, Ray, Vec3, ONB};
use crate::materials::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color, f64) {
        let onb = ONB::new(hit_record.normal);
        // let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        // let scatter_direction = Vec3::random_in_hemisphere(hit_record.normal);
        let scatter_direction = onb.local(Vec3::random_cosine_dir());
        let scattered_ray = Ray::new(hit_record.hit_point, scatter_direction);
        // let pdf = hit_record.normal.dot(*scattered_ray.direction()) / std::f64::consts::PI;
        let pdf = onb.w().dot(*scattered_ray.direction()) / std::f64::consts::PI;

        (Some(scattered_ray), self.albedo, pdf)
    }

    fn scattering_pdf(&self, ray_in: Ray, ray_scattered: Ray, hit_record: &HitRecord) -> f64 {
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
