use crate::vector::Vec3;

const RGB_CORRECTION: f64 = 255.9999;

#[derive(Clone, Copy)]
pub struct Color(pub Vec3);

fn assert_is_probability(f: f64) {
    assert!(f >= 0.0);
    assert!(f <= 1.0);
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color::new(v.x, v.y, v.z)
    }
}

/// Color is a simple newtype around Vec3 to store RGB colors.
#[allow(dead_code)]
impl Color {
    /// r, g and b must be between 0 and 1 inclusive. Otherwise this function will panic.
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        assert_is_probability(r);
        assert_is_probability(g);
        assert_is_probability(b);
        Color(Vec3 { x: r, y: g, z: b })
    }

    pub fn new_uniform(f: f64) -> Self {
        Color::new(f, f, f)
    }

    /// Red
    pub const fn r(&self) -> f64 {
        self.0.x
    }

    /// Green
    pub const fn g(&self) -> f64 {
        self.0.y
    }

    /// Blue
    pub const fn b(&self) -> f64 {
        self.0.z
    }

    pub const fn vec(&self) -> Vec3 {
        self.0
    }

    /// Converts to the standard RGB format of "three uints between 0 and 255"
    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (self.r() * RGB_CORRECTION) as u8,
            (self.g() * RGB_CORRECTION) as u8,
            (self.b() * RGB_CORRECTION) as u8,
        ]
    }

    pub fn to_rgb_gamma_corrected(&self) -> [u8; 3] {
        [
            (self.r().sqrt() * RGB_CORRECTION) as u8,
            (self.g().sqrt() * RGB_CORRECTION) as u8,
            (self.b().sqrt() * RGB_CORRECTION) as u8,
        ]
    }

    pub fn scale(&self, f: f64) -> Self {
        self.vec().scale(f).into()
    }
}
