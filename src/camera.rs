use crate::ray::Ray;
use crate::vector::Vec3;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    /// Get the ray which starts at the camera, crosses through the image, and hits the given point.
    pub fn ray_to_point(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + self.horizontal.scale(u) + self.vertical.scale(v)
            - self.origin;
        Ray {
            origin: self.origin,
            direction,
        }
    }
}
