use crate::hittables::HitRecord;
use crate::linalg::{Color, Ray};
use crate::materials::Material;

pub struct DefaultMaterial {}

impl Material for DefaultMaterial {}

impl Default for DefaultMaterial {
    fn default() -> Self {
        Self {}
    }
}
