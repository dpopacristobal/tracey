use super::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ONB {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl ONB {
    pub fn new(n: Vec3) -> Self {
        let unit_n = n.into_unit_vec();
        let w = unit_n;

        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let v = w.cross(a).into_unit_vec();
        let u = w.cross(v);

        Self { u, v, w }
    }

    pub fn u(self) -> Vec3 {
        self.u
    }

    pub fn v(self) -> Vec3 {
        self.v
    }

    pub fn w(self) -> Vec3 {
        self.w
    }

    pub fn local(self, a: Vec3) -> Vec3 {
        self.u.mul_scalar(a.x()) + self.v.mul_scalar(a.y()) + self.w.mul_scalar(a.z())
    }
}
