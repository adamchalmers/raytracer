use crate::vector::Vec3;

pub type Color = Vec3;

/// Color is a simple newtype around Vec3 to store RGB colors.
#[allow(dead_code)]
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { x: r, y: g, z: b }
    }

    /// Red
    pub fn r(&self) -> f64 {
        return self.x;
    }

    /// Green
    pub fn g(&self) -> f64 {
        return self.y;
    }

    /// Blue
    pub fn b(&self) -> f64 {
        return self.z;
    }

    /// Converts to the standard RGB format of "three uints between 0 and 255"
    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (self.x * 255.9) as u8,
            (self.y * 255.9) as u8,
            (self.z * 255.9) as u8,
        ]
    }
}
