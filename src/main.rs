use std::rc::Rc;

extern crate image;
extern crate rand;

use rand::Rng;

extern crate rey_skytracer;

use rey_skytracer::linalg::color::Color;
use rey_skytracer::linalg::ray::Ray;
use rey_skytracer::linalg::vec3::{Point3, Vec3};

fn degrees_to_radians(degrees: f64) -> f64
{
    (degrees * std::f64::consts::PI) / 180.0
}

fn clamp(val: f64, min: f64, max: f64) -> f64
{
    if val < min
    {
        return min;
    }

    if val > max
    {
        return max;
    }

    val
}

#[derive(Clone)]
struct HitRecord
{
    hit_point: Point3,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord
{
    fn new(
        hit_point: Point3,
        normal: Vec3,
        material: Rc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Self
    {
        Self {
            hit_point,
            normal,
            material,
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

trait Material
{
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color);
}

struct Lambertian
{
    albedo: Color,
}

impl Lambertian
{
    fn new(albedo: Color) -> Self
    {
        Self { albedo }
    }
}

impl Material for Lambertian
{
    fn scatter(&self, _ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color)
    {
        let scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        let scattered_ray = Ray::new(hit_record.hit_point, scatter_direction);

        (Some(scattered_ray), self.albedo)
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3
{
    v - n.mul_scalar(2.0 * v.dot(n))
}

struct Metal
{
    albedo: Color,
    fuzz_factor: f64,
}

impl Metal
{
    fn new(albedo: Color, fuzz_factor: f64) -> Self
    {
        Self {
            albedo,
            fuzz_factor,
        }
    }
}

impl Material for Metal
{
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color)
    {
        let reflected_direction = reflect(ray_in.direction().into_unit_vec(), hit_record.normal);
        let reflected_ray = Ray::new(
            hit_record.hit_point,
            reflected_direction + Vec3::random_in_unit_sphere().mul_scalar(self.fuzz_factor),
        );

        // This is probably not how you do this and there is a much neater way
        let mut ret: Option<Ray> = None;
        if reflected_ray.direction().dot(hit_record.normal) > 0.0
        {
            ret = Some(reflected_ray);
        }

        (ret, self.albedo)
    }
}

// Should add a description here about what the refractive index ratio actually means
fn refract(ray_in: Vec3, normal: Vec3, refractive_index_ratio: f64) -> Vec3
{
    let cos_theta = -ray_in.dot(normal);
    let ray_out_perp = (ray_in + normal.mul_scalar(cos_theta)).mul_scalar(refractive_index_ratio);
    let ray_out_parallel = normal.mul_scalar(-((1.0 - ray_out_perp.length_sq()).abs()).sqrt());

    ray_out_perp + ray_out_parallel
}

fn schlick(cos_theta: f64, refractive_index_ratio: f64) -> f64
{
    let r0 = ((1.0 - refractive_index_ratio) / (1.0 + refractive_index_ratio)).powi(2);
    r0 + (1.0 - r0) * ((1.0 - cos_theta).powi(5))
}

struct Dielectric
{
    refractive_index: f64,
}

impl Dielectric
{
    fn new(refractive_index: f64) -> Self
    {
        Self { refractive_index }
    }
}

impl Material for Dielectric
{
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> (Option<Ray>, Color)
    {
        let attenuation = Color::from_scalar(1.0);
        let refractive_index_ratio = if hit_record.front_face
        {
            1.0 / self.refractive_index
        }
        else
        {
            self.refractive_index
        };

        let unit_direction = ray_in.direction().into_unit_vec();

        let cos_theta = (-unit_direction.dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let reflect_prob = schlick(cos_theta, refractive_index_ratio);
        let mut rng = rand::thread_rng();
        let direction = if refractive_index_ratio * sin_theta > 1.0
            || rng.gen_range(0.0, 1.0) < reflect_prob
        {
            reflect(unit_direction, hit_record.normal)
        }
        else
        {
            refract(unit_direction, hit_record.normal, refractive_index_ratio)
        };

        let refracted_ray = Ray::new(hit_record.hit_point, direction);

        (Some(refracted_ray), attenuation)
    }
}

trait Hit
{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        let mut ret: Option<HitRecord> = None;

        let mut closest_so_far = t_max;
        for object in &self.objects
        {
            let hit_result = object.hit(ray, t_min, closest_so_far);
            if let Some(hit_record) = hit_result
            {
                closest_so_far = hit_record.t;
                ret = Some(hit_record);
            }
        }

        ret
    }
}

struct Sphere
{
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere
{
    fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self
    {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere
{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
    {
        let mut hit_record = HitRecord::new(
            Point3::from_scalar(0.0),
            Vec3::from_scalar(0.0),
            self.material.clone(),
            0.0,
            false,
        );

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
                return Some(hit_record);
            }
            temp = (-half_b + root) / a;
            if (temp < t_max) && (temp > t_min)
            {
                hit_record.t = temp;
                hit_record.hit_point = ray.at(hit_record.t);
                let outward_normal = (hit_record.hit_point - self.center).div_scalar(self.radius);
                hit_record.set_face_normal(ray, outward_normal);
                return Some(hit_record);
            }
        }

        None
    }
}

struct Camera
{
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera
{
    fn new(
        look_from: Point3,
        look_at: Point3,
        up_direction: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self
    {
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

    fn get_ray(&self, s: f64, t: f64) -> Ray
    {
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

fn ray_color(ray: Ray, world: &HittableList, depth: i32) -> Color
{
    if depth <= 0
    {
        return Color::from_scalar(0.0);
    }

    let hit_result = world.hit(ray, 0.001, f64::INFINITY);
    if let Some(hit_record) = hit_result
    {
        let (scatter_result, attenuation) = hit_record.material.scatter(ray, &hit_record);
        if let Some(scatter_ray) = scatter_result
        {
            return attenuation * ray_color(scatter_ray, world, depth - 1);
        }

        return Color::from_scalar(0.0);
    }

    let unit_direction = ray.direction().into_unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::from_scalar(1.0).mul_scalar(1.0 - t) + Color::new(0.5, 0.7, 1.0).mul_scalar(t)
}

fn main()
{
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 10;
    let max_depth = 50;

    let mut image_buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);

    // World
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_sphere_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_sphere_left = Rc::new(Dielectric::new(1.5));
    let material_sphere_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_sphere_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_sphere_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_sphere_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_sphere_right,
    )));

    // Camera
    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let up_direction = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;

    let camera = Camera::new(
        look_from,
        look_at,
        up_direction,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    let mut rng = rand::thread_rng();
    for (i, j, pixel) in image_buffer.enumerate_pixels_mut()
    {
        let mut pixel_color_accumulator = Color::from_scalar(0.0);
        for _ in 0..samples_per_pixel
        {
            let u = (i as f64 + rng.gen_range(0.0, 1.0)) / (image_width - 1) as f64;
            let v =
                ((image_height - j) as f64 + rng.gen_range(0.0, 1.0)) / (image_height - 1) as f64;
            let ray = camera.get_ray(u, v);
            pixel_color_accumulator.accumulate_sample(ray_color(ray, &world, max_depth));
        }

        let pixel_color: Color = pixel_color_accumulator.average_samples(samples_per_pixel);
        *pixel = image::Rgb(pixel_color.gamma_2_correct().into_rgb8());
    }

    // Output Image
    image_buffer.save("rendered_image.png").unwrap();
}
