extern crate image;

extern crate rey_skytracer;

use std::rc::Rc;

use rey_skytracer::linalg::color::Color;
use rey_skytracer::linalg::ray::Ray;
use rey_skytracer::linalg::vec3::{Point3, Vec3};

fn degrees_to_radians(degrees: f64) -> f64
{
    (degrees * std::f64::consts::PI) / 180.0
}

#[derive(Clone)]
struct HitRecord
{
    hit_point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord
{
    fn new(hit_point: Point3, normal: Vec3, t: f64, front_face: bool) -> Self
    {
        Self {
            hit_point,
            normal,
            t,
            front_face,
        }
    }

    fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3)
    {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face
        {
            self.normal = outward_normal;
        }
        else
        {
            self.normal = outward_normal.mul_scalar(-1.0);
        }
    }
}

trait Hit
{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

struct HittableList
{
    objects: Vec<Rc<dyn Hit>>,
}

impl HittableList
{
    fn new() -> Self
    {
        Self {
            objects: Vec::new(),
        }
    }

    // Do we take the object by reference or do we take ownership of it?
    // So Rc doesn't have the copy trait, which suggests that if we take ownership of it we would be destroying the original version? Is that what we want?
    fn from_hittable(object: Rc<dyn Hit>) -> Self
    {
        // Do not clone it here if we take it
        Self {
            objects: vec![object.clone()],
        }
    }

    fn clear(&mut self)
    {
        self.objects.clear();
    }

    fn add(&mut self, object: Rc<dyn Hit>)
    {
        self.objects.push(object);
    }
}

impl Hit for HittableList
{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool
    {
        let mut temp_hit_record =
            HitRecord::new(Point3::from_scalar(0.0), Vec3::from_scalar(0.0), 0.0, false);
        let mut is_hit = false;
        let mut closest_so_far = t_max;
        for object in &self.objects
        {
            if object.hit(ray, t_min, closest_so_far, &mut temp_hit_record)
            {
                is_hit = true;
                closest_so_far = temp_hit_record.t;
                *hit_record = temp_hit_record.clone();
            }
        }

        is_hit
    }
}

struct Sphere
{
    center: Point3,
    radius: f64,
}

impl Sphere
{
    fn new(center: Point3, radius: f64) -> Self
    {
        Self { center, radius }
    }
}

impl Hit for Sphere
{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool
    {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_sq();
        let half_b = oc.dot(*ray.direction());
        let c = oc.length_sq() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant > 0.0
        {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if (temp < t_max) && (temp > t_min)
            {
                hit_record.t = temp;
                hit_record.hit_point = ray.at(hit_record.t);
                let outward_normal = (hit_record.hit_point - self.center).div_scalar(self.radius);
                hit_record.set_face_normal(ray, outward_normal);
                return true;
            }
            temp = (-half_b + root) / a;
            if (temp < t_max) && (temp > t_min)
            {
                hit_record.t = temp;
                hit_record.hit_point = ray.at(hit_record.t);
                let outward_normal = (hit_record.hit_point - self.center).div_scalar(self.radius);
                hit_record.set_face_normal(ray, outward_normal);
                return true;
            }
        }

        false
    }
}

fn ray_color(ray: Ray, world: &HittableList) -> Color
{
    let mut hit_record =
        HitRecord::new(Point3::from_scalar(0.0), Vec3::from_scalar(0.0), 0.0, false);
    if world.hit(ray, 0.0, f64::INFINITY, &mut hit_record)
    {
        return (hit_record.normal.into_color() + Color::from_scalar(1.0)).mul_scalar(0.5);
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

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut()
    {
        let u = x as f64 / (image_width - 1) as f64;
        let v = (image_height - y) as f64 / (image_height - 1) as f64;

        let ray = Ray::new(
            origin,
            lower_left_corner + horizontal.mul_scalar(u) + vertical.mul_scalar(v) - origin,
        );
        let color = ray_color(ray, &world);

        *pixel = image::Rgb(color.into_rgb8());
    }

    image_buffer.save("rendered_image.png").unwrap();
}
