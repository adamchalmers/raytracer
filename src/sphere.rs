use crate::Ray;
use crate::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray) -> bool {
        let origin_to_center = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * origin_to_center.dot(ray.direction);
        let c = origin_to_center.dot(origin_to_center) - self.radius.powf(2.0);
        let d = b.powf(2.0) - (4.0 * a * c);
        d > 0.0
    }
}
