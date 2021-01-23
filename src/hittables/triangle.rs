use std::sync::Arc;

use crate::linalg::{Point3, Ray, Vec3};
use crate::materials::Material;

use super::{BoundingBox, Hit, HitRecord};

pub struct Triangle {
    vertices: [Vec3; 3],
    normal: Vec3,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3], material: Arc<dyn Material>) -> Self {
        let normal = (vertices[1] - vertices[0])
            .cross(vertices[2] - vertices[0])
            .into_unit_vec();

        Self {
            vertices,
            normal,
            material,
        }
    }
}

impl Hit for Triangle {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = ray.direction().cross(edge2);
        let a = edge1.dot(h);
        if a > -0.0001 && a < 0.0001 {
            return None;
        }
        let f = 1.0 / a;
        let s = *ray.origin() - self.vertices[0];
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(edge1);
        let v = f * ray.direction().dot(q);
        if v < 0.01 || u + v > 1.0 {
            return None;
        }
        let t = f * edge2.dot(q);
        if t < 0.0001 {
            return None;
        }

        let hit_point = ray.at(t);
        let mut hit_record =
            HitRecord::new(hit_point, Vec3::default(), self.material.clone(), t, false);
        hit_record.set_face_normal(ray, self.normal.into_unit_vec());

        Some(hit_record)
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64) -> Option<BoundingBox> {
        let mut max = self.vertices[0];
        let mut min = self.vertices[0];
        for vertex in &self.vertices {
            *max.x_mut() = vertex.x().max(max.x());
            *max.y_mut() = vertex.y().max(max.y());
            *max.z_mut() = vertex.z().max(max.z());

            *min.x_mut() = vertex.x().min(min.x());
            *min.y_mut() = vertex.y().min(min.y());
            *min.z_mut() = vertex.z().min(min.z());
        }

        *max.x_mut() = max.x() + 0.0001;
        *max.y_mut() = max.y() + 0.0001;
        *max.z_mut() = max.z() + 0.0001;

        *min.x_mut() = min.x() - 0.0001;
        *min.y_mut() = min.y() - 0.0001;
        *min.z_mut() = min.z() - 0.0001;

        Some(BoundingBox::new(min, max))
    }
}
