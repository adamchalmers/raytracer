use crate::Ray;
use crate::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    /// Does the ray hit this sphere?
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let origin_to_center = ray.origin - self.center;

        // Use the quadratic equation's discriminant to check how many places the ray intersects
        // this sphere.
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * origin_to_center.dot(ray.direction);
        let c = origin_to_center.dot(origin_to_center) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - (4.0 * a * c);
        if discriminant < 0.0 {
            None
        } else {
            let hit_point = (-b - discriminant.sqrt()) / (2.0 * a);
            Some(hit_point)
        }
    }
}
