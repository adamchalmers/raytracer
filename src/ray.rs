use crate::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.a
    }
    pub fn direction(&self) -> Vec3 {
        self.b
    }
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.a + (self.b.scale_f(t))
    }
}
