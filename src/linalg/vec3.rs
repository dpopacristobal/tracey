use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use rand::Rng;

use super::Color;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_scalar(v: f64) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let vec = Self::random_from_bounds(-1.0, 1.0);
            if vec.length_sq() <= 1.0 {
                return vec;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let vec = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if vec.length_sq() >= 1.0 {
                continue;
            }

            return vec;
        }
    }

    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0, std::f64::consts::PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z.powi(2)).sqrt();

        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn x_mut<'a>(&'a mut self) -> &'a mut f64 {
        &mut self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn y_mut<'a>(&'a mut self) -> &'a mut f64 {
        &mut self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn z_mut<'a>(&'a mut self) -> &'a mut f64 {
        &mut self.z
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_sq(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn into_unit_vec(self) -> Self {
        Self {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    pub fn add_scalar(self, rhs: f64) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }

    pub fn sub_scalar(self, rhs: f64) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }

    pub fn mul_scalar(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }

    pub fn div_scalar(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }

    pub fn random_from_bounds(low: f64, high: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(low, high),
            y: rng.gen_range(low, high),
            z: rng.gen_range(low, high),
        }
    }

    pub fn into_color(self) -> Color {
        Color::new(self.x, self.y, self.z)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        };
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(),
        }
    }
}

// [TODO] Make testing more exhaustive by not having 0.0 in the tests,
#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn vec3_constructor() {
        let lhs = Vec3::from_scalar(0.0);
        let rhs = Vec3::new(0.0, 0.0, 0.0);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn vec3_value_access() {
        let lhs = Vec3::new(0.0, 1.0, 2.0);

        assert_eq!(lhs.x(), 0.0);
        assert_eq!(lhs.y(), 1.0);
        assert_eq!(lhs.z(), 2.0);

        assert_eq!(lhs[0], 0.0);
        assert_eq!(lhs[1], 1.0);
        assert_eq!(lhs[2], 2.0);

        let lhs = Vec3::new(0.0, 1.0, 2.0);

        assert_eq!(lhs.x(), 0.0);
        assert_eq!(lhs.y(), 1.0);
        assert_eq!(lhs.z(), 2.0);

        let mut lhs = Vec3::new(0.0, 0.0, 0.0);
        lhs[1] = 1.0;
        lhs[2] = 2.0;

        assert_eq!(lhs.x(), 0.0);
        assert_eq!(lhs.y(), 1.0);
        assert_eq!(lhs.z(), 2.0);
    }

    #[test]
    fn vec3_add() {
        let lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs + rhs,
            Vec3 {
                x: 0.0,
                y: 2.0,
                z: 4.0
            }
        );

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(0.0, 1.0, 2.0);
        lhs += rhs;
        assert_eq!(
            lhs,
            Vec3 {
                x: 0.0,
                y: 2.0,
                z: 4.0
            }
        );

        let lhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.add_scalar(1.0),
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn vec3_sub() {
        let lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs - rhs,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(0.0, 1.0, 2.0);
        lhs -= rhs;
        assert_eq!(
            lhs,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );

        let lhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.sub_scalar(-1.0),
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn vec3_mul() {
        let lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(3.0, 3.0, 3.0);
        assert_eq!(
            lhs * rhs,
            Vec3 {
                x: 0.0,
                y: 3.0,
                z: 6.0
            }
        );

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(3.0, 3.0, 3.0);
        lhs *= rhs;
        assert_eq!(
            lhs,
            Vec3 {
                x: 0.0,
                y: 3.0,
                z: 6.0
            }
        );

        let lhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.mul_scalar(2.0),
            Vec3 {
                x: 0.0,
                y: 2.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn vec3_div() {
        let lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(1.0, 2.0, 4.0);
        assert_eq!(
            lhs / rhs,
            Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.5
            }
        );

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(1.0, 2.0, 4.0);
        lhs /= rhs;
        assert_eq!(
            lhs,
            Vec3 {
                x: 0.0,
                y: 0.5,
                z: 0.5
            }
        );

        let lhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.div_scalar(2.0),
            Vec3 {
                x: 0.0,
                y: 0.5,
                z: 1.0
            }
        );
    }

    #[test]
    fn vec3_length() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let length_squared = 14.0 as f64;
        assert_eq!(lhs.length(), length_squared.sqrt());
    }

    #[test]
    fn vec3_length_sq() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(lhs.length_sq(), 14.0);
    }

    #[test]
    fn vec3_dot() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let rhs = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(lhs.dot(rhs), 20.0);
    }

    #[test]
    fn vec3_cross() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let rhs = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(
            lhs.cross(rhs),
            Vec3 {
                x: -1.0,
                y: 2.0,
                z: -1.0
            }
        );
    }

    #[test]
    fn vec3_into_unit_vec() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let len = 14.0 as f64;
        assert_eq!(
            lhs.into_unit_vec(),
            Vec3 {
                x: 1.0 / len.sqrt(),
                y: 2.0 / len.sqrt(),
                z: 3.0 / len.sqrt()
            }
        );
    }
}
