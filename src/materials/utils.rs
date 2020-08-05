use crate::linalg::vec3::Vec3;

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
