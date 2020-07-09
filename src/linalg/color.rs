use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color
{
    r: f64,
    g: f64,
    b: f64,
}

impl Color
{
    pub fn new(r: f64, g: f64, b: f64) -> Self
    {
        Color { r, g, b }
    }

    pub fn from_scalar(v: f64) -> Self
    {
        Color { r: v, g: v, b: v }
    }

    pub fn r(self) -> f64
    {
        self.r
    }

    pub fn g(self) -> f64
    {
        self.g
    }

    pub fn b(self) -> f64
    {
        self.b
    }

    pub fn add_scalar(self, rhs: f64) -> Self
    {
        Self {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
        }
    }

    pub fn sub_scalar(self, rhs: f64) -> Self
    {
        Self {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
        }
    }

    pub fn mul_scalar(self, rhs: f64) -> Self
    {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }

    pub fn div_scalar(self, rhs: f64) -> Self
    {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }

    pub fn into_rgb8(self) -> [u8; 3]
    {
        let ir = (255.99 * self.r) as u8;
        let ig = (255.99 * self.g) as u8;
        let ib = (255.99 * self.b) as u8;

        [ir, ig, ib]
    }
}

impl Add for Color
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self
    {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color
{
    fn add_assign(&mut self, rhs: Self)
    {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl Sub for Color
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self
    {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl SubAssign for Color
{
    fn sub_assign(&mut self, rhs: Self)
    {
        *self = Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        };
    }
}

impl Mul for Color
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self
    {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl MulAssign for Color
{
    fn mul_assign(&mut self, rhs: Self)
    {
        *self = Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        };
    }
}

impl Div for Color
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self
    {
        Self {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl DivAssign for Color
{
    fn div_assign(&mut self, rhs: Self)
    {
        *self = Self {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        };
    }
}

impl Index<usize> for Color
{
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output
    {
        match index
        {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!(),
        }
    }
}

impl IndexMut<usize> for Color
{
    fn index_mut(&mut self, index: usize) -> &mut f64
    {
        match index
        {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Color
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let ir = (255.99 * self.r) as u8;
        let ig = (255.99 * self.g) as u8;
        let ib = (255.99 * self.b) as u8;

        write!(f, "{} {} {}", ir, ig, ib)
    }
}

// [TODO] Make testing more exhaustive by not having 0.0 in the tests,
#[cfg(test)]
mod test
{
    use super::Color;

    #[test]
    fn color_constructor()
    {
        let lhs = Color::from_scalar(0.0);
        let rhs = Color::new(0.0, 0.0, 0.0);

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn color_value_access()
    {
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

        let mut lhs = Color::new(0.0, 0.0, 0.0);
        lhs[1] = 1.0;
        lhs[2] = 2.0;

        assert_eq!(lhs.r(), 0.0);
        assert_eq!(lhs.g(), 1.0);
        assert_eq!(lhs.b(), 2.0);
    }

    #[test]
    fn color_add()
    {
        let lhs = Color::new(0.0, 1.0, 2.0);
        let rhs = Color::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs + rhs,
            Color {
                r: 0.0,
                g: 2.0,
                b: 4.0
            }
        );

        let mut lhs = Color::new(0.0, 1.0, 2.0);
        let rhs = Color::new(0.0, 1.0, 2.0);
        lhs += rhs;
        assert_eq!(
            lhs,
            Color {
                r: 0.0,
                g: 2.0,
                b: 4.0
            }
        );

        let lhs = Color::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.add_scalar(1.0),
            Color {
                r: 1.0,
                g: 2.0,
                b: 3.0
            }
        );
    }

    #[test]
    fn color_sub()
    {
        let lhs = Color::new(0.0, 1.0, 2.0);
        let rhs = Color::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs - rhs,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0
            }
        );

        let mut lhs = Color::new(0.0, 1.0, 2.0);
        let rhs = Color::new(0.0, 1.0, 2.0);
        lhs -= rhs;
        assert_eq!(
            lhs,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0
            }
        );

        let lhs = Color::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.sub_scalar(-1.0),
            Color {
                r: 1.0,
                g: 2.0,
                b: 3.0
            }
        );
    }

    #[test]
    fn color_mul()
    {
        let lhs = Color::new(0.0, 1.0, 2.0);
        let rhs = Color::new(3.0, 3.0, 3.0);
        assert_eq!(
            lhs * rhs,
            Color {
                r: 0.0,
                g: 3.0,
                b: 6.0
            }
        );

        let mut lhs = Color::new(0.0, 1.0, 2.0);
        let rhs = Color::new(3.0, 3.0, 3.0);
        lhs *= rhs;
        assert_eq!(
            lhs,
            Color {
                r: 0.0,
                g: 3.0,
                b: 6.0
            }
        );

        let lhs = Color::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.mul_scalar(2.0),
            Color {
                r: 0.0,
                g: 2.0,
                b: 4.0
            }
        );
    }

    #[test]
    fn color_div()
    {
        let lhs = Color::new(0.0, 1.0, 2.0);
        let rhs = Color::new(1.0, 2.0, 4.0);
        assert_eq!(
            lhs / rhs,
            Color {
                r: 0.0,
                g: 0.5,
                b: 0.5
            }
        );

        let mut lhs = Color::new(0.0, 1.0, 2.0);
        let rhs = Color::new(1.0, 2.0, 4.0);
        lhs /= rhs;
        assert_eq!(
            lhs,
            Color {
                r: 0.0,
                g: 0.5,
                b: 0.5
            }
        );

        let lhs = Color::new(0.0, 1.0, 2.0);
        assert_eq!(
            lhs.div_scalar(2.0),
            Color {
                r: 0.0,
                g: 0.5,
                b: 1.0
            }
        );
    }
}
