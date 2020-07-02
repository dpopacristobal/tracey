use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    v0: f64,
    v1: f64,
    v2: f64,
}

// [TODO] Might be worth looking into wrapping these into tuples so that they can each have their own methods and do not share functionality.
pub type Color = Vec3;

impl Vec3 {
    pub fn new(v0: f64, v1: f64, v2: f64) -> Self {
        Vec3 { v0, v1, v2 }
    }

    pub fn from_scalar(v: f64) -> Self {
        Vec3 {
            v0: v,
            v1: v,
            v2: v,
        }
    }

    pub fn x(&self) -> f64 {
        self.v0
    }

    pub fn y(&self) -> f64 {
        self.v1
    }

    pub fn z(&self) -> f64 {
        self.v2
    }

    pub fn length(&self) -> f64 {
        (self.v0 * self.v0 + self.v1 * self.v1 + self.v2 * self.v2).sqrt()
    }

    pub fn length_sq(&self) -> f64 {
        self.v0 * self.v0 + self.v1 * self.v1 + self.v2 * self.v2
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.v0 * rhs.v0 + self.v1 * rhs.v1 + self.v2 * rhs.v2
    }

    pub fn cross(self, rhs: &Self) -> Self {
        Self {
            v0: self.v1 * rhs.v2 - self.v2 * rhs.v1,
            v1: self.v2 * rhs.v0 - self.v0 * rhs.v2,
            v2: self.v0 * rhs.v1 - self.v1 * rhs.v0,
        }
    }

    pub fn into_unit_vec(self) -> Self {
        Self {
            v0: self.v0 / self.length(),
            v1: self.v1 / self.length(),
            v2: self.v2 / self.length(),
        }
    }

    pub fn r(&self) -> f64 {
        self.v0
    }

    pub fn g(&self) -> f64 {
        self.v1
    }

    pub fn b(&self) -> f64 {
        self.v2
    }

    pub fn add_scalar(self, rhs: f64) -> Self {
        Self {
            v0: self.v0 + rhs,
            v1: self.v1 + rhs,
            v2: self.v2 + rhs,
        }
    }

    pub fn sub_scalar(self, rhs: f64) -> Self {
        Self {
            v0: self.v0 - rhs,
            v1: self.v1 - rhs,
            v2: self.v2 - rhs,
        }
    }

    pub fn mul_scalar(self, rhs: f64) -> Self {
        Self {
            v0: self.v0 * rhs,
            v1: self.v1 * rhs,
            v2: self.v2 * rhs,
        }
    }

    pub fn div_scalar(self, rhs: f64) -> Self {
        Self {
            v0: self.v0 / rhs,
            v1: self.v1 / rhs,
            v2: self.v2 / rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            v0: self.v0 + rhs.v0,
            v1: self.v1 + rhs.v1,
            v2: self.v2 + rhs.v2,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            v0: self.v0 + rhs.v0,
            v1: self.v1 + rhs.v1,
            v2: self.v2 + rhs.v2,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            v0: self.v0 - rhs.v0,
            v1: self.v1 - rhs.v1,
            v2: self.v2 - rhs.v2,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            v0: self.v0 - rhs.v0,
            v1: self.v1 - rhs.v1,
            v2: self.v2 - rhs.v2,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            v0: self.v0 * rhs.v0,
            v1: self.v1 * rhs.v1,
            v2: self.v2 * rhs.v2,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            v0: self.v0 * rhs.v0,
            v1: self.v1 * rhs.v1,
            v2: self.v2 * rhs.v2,
        };
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            v0: self.v0 / rhs.v0,
            v1: self.v1 / rhs.v1,
            v2: self.v2 / rhs.v2,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            v0: self.v0 / rhs.v0,
            v1: self.v1 / rhs.v1,
            v2: self.v2 / rhs.v2,
        };
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.v0,
            1 => &self.v1,
            2 => &self.v2,
            _ => panic!(),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.v0,
            1 => &mut self.v1,
            2 => &mut self.v2,
            _ => panic!(),
        }
    }
}

// [TODO] Make testing more exhaustive by not having 0.0 in the tests,
#[cfg(test)]
mod test {
    use super::{Color, Vec3};

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

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);

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
    fn color_value_access() {
        let lhs = Color::new(0.0, 1.0, 2.0);

        assert_eq!(lhs.r(), 0.0);
        assert_eq!(lhs.g(), 1.0);
        assert_eq!(lhs.b(), 2.0);

        assert_eq!(lhs[0], 0.0);
        assert_eq!(lhs[1], 1.0);
        assert_eq!(lhs[2], 2.0);

        let mut lhs = Color::new(0.0, 1.0, 2.0);

        assert_eq!(lhs.r(), 0.0);
        assert_eq!(lhs.g(), 1.0);
        assert_eq!(lhs.b(), 2.0);
    }

    #[test]
    fn vec3_add() {
        let lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs + rhs,
            Vec3 {
                v0: 0.0,
                v1: 2.0,
                v2: 4.0
            }
        );

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(0.0, 1.0, 2.0);
        lhs += rhs;
        assert_eq!(
            lhs,
            Vec3 {
                v0: 0.0,
                v1: 2.0,
                v2: 4.0
            }
        );

        let lhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.add_scalar(1.0),
            Vec3 {
                v0: 1.0,
                v1: 2.0,
                v2: 3.0
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
                v0: 0.0,
                v1: 0.0,
                v2: 0.0
            }
        );

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(0.0, 1.0, 2.0);
        lhs -= rhs;
        assert_eq!(
            lhs,
            Vec3 {
                v0: 0.0,
                v1: 0.0,
                v2: 0.0
            }
        );

        let lhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.sub_scalar(-1.0),
            Vec3 {
                v0: 1.0,
                v1: 2.0,
                v2: 3.0
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
                v0: 0.0,
                v1: 3.0,
                v2: 6.0
            }
        );

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(3.0, 3.0, 3.0);
        lhs *= rhs;
        assert_eq!(
            lhs,
            Vec3 {
                v0: 0.0,
                v1: 3.0,
                v2: 6.0
            }
        );

        let lhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.mul_scalar(2.0),
            Vec3 {
                v0: 0.0,
                v1: 2.0,
                v2: 4.0
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
                v0: 0.0,
                v1: 0.5,
                v2: 0.5
            }
        );

        let mut lhs = Vec3::new(0.0, 1.0, 2.0);
        let rhs = Vec3::new(1.0, 2.0, 4.0);
        lhs /= rhs;
        assert_eq!(
            lhs,
            Vec3 {
                v0: 0.0,
                v1: 0.5,
                v2: 0.5
            }
        );

        let lhs = Vec3::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.div_scalar(2.0),
            Vec3 {
                v0: 0.0,
                v1: 0.5,
                v2: 1.0
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
        assert_eq!(lhs.dot(&rhs), 20.0);
    }

    #[test]
    fn vec3_cross() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let rhs = Vec3::new(2.0, 3.0, 4.0);
        assert_eq!(
            lhs.cross(&rhs),
            Vec3 {
                v0: -1.0,
                v1: 2.0,
                v2: -1.0
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
                v0: 1.0 / len.sqrt(),
                v1: 2.0 / len.sqrt(),
                v2: 3.0 / len.sqrt()
            }
        );
    }
}
