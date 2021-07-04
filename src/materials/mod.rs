pub use default::DefaultMaterial;
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::hittables::HitRecord;
use crate::linalg::{Color, Ray, Vec3};
use crate::pdfs::PDF;

pub mod default;
pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

use std::sync::Arc;

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub pdf: Option<Arc<dyn PDF>>,
    pub attenuation: Color,
}

impl ScatterRecord {
    pub fn new(specular_ray: Option<Ray>, pdf: Option<Arc<dyn PDF>>, attenuation: Color) -> Self {
        Self {
            specular_ray,
            pdf,
            attenuation,
        }
    }
}

pub trait Material: Send + Sync {
    fn scatter(&self, _ray_in: Ray, _hit_record: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emit(&self, _u: f64, _v: f64, _hit_record: &mut HitRecord) -> Color {
        Color::default()
    }

    fn scattering_pdf(&self, _ray_in: Ray, _ray_scattered: Ray, _hit_record: &HitRecord) -> f64 {
        0.0
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
