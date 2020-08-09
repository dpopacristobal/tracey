use crate::linalg::Ray;
use crate::linalg::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * std::f64::consts::PI) / 180.0
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        up_direction: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).into_unit_vec();
        let u = up_direction.cross(w).into_unit_vec();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u.mul_scalar(focus_dist * viewport_width);
        let vertical = v.mul_scalar(focus_dist * viewport_height);
        let lower_left_corner = origin
            - horizontal.mul_scalar(0.5)
            - vertical.mul_scalar(0.5)
            - w.mul_scalar(focus_dist);
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let ray = Vec3::random_in_unit_disk().mul_scalar(self.lens_radius);
        let offset = self.u.mul_scalar(ray.x()) + self.v.mul_scalar(ray.y());

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal.mul_scalar(s) + self.vertical.mul_scalar(t)
                - self.origin
                - offset,
        )
    }
}
