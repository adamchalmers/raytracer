use crate::vector::Vec3;

pub type Color = Vec3;

#[allow(dead_code)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { x: r, y: g, z: b }
    }

    pub fn r(&self) -> f64 {
        return self.x;
    }

    pub fn g(&self) -> f64 {
        return self.y;
    }

    pub fn b(&self) -> f64 {
        return self.z;
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (self.x * 255.9) as u8,
            (self.y * 255.9) as u8,
            (self.z * 255.9) as u8,
        ]
    }
}
