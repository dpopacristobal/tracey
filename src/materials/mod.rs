pub use default::DefaultMaterial;
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::hittables::HitRecord;
use crate::linalg::{Color, Point3, Ray, Vec3};

pub mod default;
pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color);

    fn emit(&self, _u: f64, _v: f64, _hit_point: Point3) -> Color {
        Color::default()
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n.mul_scalar(2.0 * v.dot(n))
}

pub fn refract(ray_in: Vec3, normal: Vec3, refractive_index_ratio: f64) -> Vec3 {
    let cos_theta = -ray_in.dot(normal);
    let ray_out_perp = (ray_in + normal.mul_scalar(cos_theta)).mul_scalar(refractive_index_ratio);
    let ray_out_parallel = normal.mul_scalar(-((1.0 - ray_out_perp.length_sq()).abs()).sqrt());

    ray_out_perp + ray_out_parallel
}

pub fn schlick(cos_theta: f64, refractive_index_ratio: f64) -> f64 {
    let r0 = ((1.0 - refractive_index_ratio) / (1.0 + refractive_index_ratio)).powi(2);
    r0 + (1.0 - r0) * ((1.0 - cos_theta).powi(5))
}
