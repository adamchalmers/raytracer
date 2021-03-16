use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

#[allow(dead_code)]
impl Vec3 {
    /// Create a vector where all components have value f
    pub const fn new_uniform(f: f64) -> Self {
        Vec3 { x: f, y: f, z: f }
    }

    /// Create the zero vector <0, 0, 0>
    pub const fn zero() -> Self {
        Vec3::new_uniform(0.0)
    }

    /// Gets a unit vector in the same direction as self.
    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    /// Compute the dot product
    pub fn dot(&self, other: &Self) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - normal.scale(self.dot(normal) * 2.0)
    }

    pub fn refract(&self, normal: &Self, ni_over_nt: f64) -> Option<Self> {
        let dt = self.unit().dot(&normal);
        let discriminant = 1.0 - ni_over_nt.powf(2.0) * (1.0 - dt.powf(2.0));
        if discriminant > 0.0 {
            let refracted =
                (self.unit() - *normal * dt) * ni_over_nt - *normal * discriminant.sqrt();
            Some(refracted)
        } else {
            None
        }
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// Compute the cross product
    pub fn cross(self, other: Self) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    // Create a weighted average of the two vectors, i.e.
    // t*other + (1-t)self
    pub fn interpolate(&self, other: &Vec3, t: f64) -> Self {
        *self * (1.0 - t) + *other * t
    }

    /// Scale the vector by a scalar
    pub fn scale(&self, f: f64) -> Self {
        Vec3 {
            x: f * self.x,
            y: f * self.y,
            z: f * self.z,
        }
    }
}

impl<'a> Sum<&'a Vec3> for Vec3 {
    fn sum<I: Iterator<Item = &'a Vec3>>(iter: I) -> Vec3 {
        let mut output = Vec3::zero();
        for v in iter {
            output += *v;
        }
        output
    }
}

impl<'a> Sum<Vec3> for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Vec3 {
        let mut output = Vec3::zero();
        for v in iter {
            output += v;
        }
        output
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        };
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}
