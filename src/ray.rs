use crate::Vec3;

/// Rays are just finite lines, i.e. lines with a start and a direction.
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Get a point that lies along the ray, `t` units away from the ray's origin.
    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction.scale(t))
    }
}
