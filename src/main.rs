extern crate image;

extern crate rey_skytracer;

use rey_skytracer::linalg::color::Color;
use rey_skytracer::linalg::ray::Ray;
use rey_skytracer::linalg::vec3::{Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64
{
    let oc = *ray.origin() - center;

    let a = (*ray.direction()).dot(*ray.direction());
    let b = 2.0 * oc.dot(*ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0
    {
        return -1.0;
    }
    (-b - discriminant.sqrt()) / (2.0 * a)
}

fn ray_color(ray: Ray) -> Color
{
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0
    {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).into_unit_vec();
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0).mul_scalar(0.5);
    }

    let unit_direction = ray.direction().into_unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::from_scalar(1.0).mul_scalar(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul_scalar(t)
}

fn main()
{
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let mut image_buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from_scalar(0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - horizontal.div_scalar(2.0)
        - vertical.div_scalar(2.0)
        - Vec3::new(0.0, 0.0, focal_length);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut()
    {
        let u = x as f64 / (image_width - 1) as f64;
        let v = (image_height - y) as f64 / (image_height - 1) as f64;

        let ray = Ray::new(
            origin,
            lower_left_corner + horizontal.mul_scalar(u) + vertical.mul_scalar(v) - origin,
        );
        let color = ray_color(ray);

        *pixel = image::Rgb(color.into_rgb8());
    }

    image_buffer.save("rendered_image.png").unwrap();
}
