use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

pub struct Vec3 {
    v0: f64,
    v1: f64,
    v2: f64,
}

pub type Point = Vec3;
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

    pub fn x(&self) -> &f64 {
        &self.v0
    }

    pub fn y(&self) -> &f64 {
        &self.v1
    }

    pub fn z(&self) -> &f64 {
        &self.v2
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.v0
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.v1
    }

    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.v2
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

    pub fn r(&self) -> &f64 {
        &self.v0
    }

    pub fn g(&self) -> &f64 {
        &self.v1
    }

    pub fn b(&self) -> &f64 {
        &self.v2
    }

    pub fn r_mut(&mut self) -> &mut f64 {
        &mut self.v0
    }

    pub fn g_mut(&mut self) -> &mut f64 {
        &mut self.v1
    }

    pub fn b_mut(&mut self) -> &mut f64 {
        &mut self.v2
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
