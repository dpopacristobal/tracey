use std::ops;

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

impl ops::Index<usize> for Vec3 {
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

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        match index {
            0 => &mut self.v0,
            1 => &mut self.v1,
            2 => &mut self.v2,
            _ => panic!(),
        }
    }
}
