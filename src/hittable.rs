use crate::material::Material;
use crate::Ray;
use crate::Vec3;

pub enum Hittable {
    Sphere(Sphere),
    Many(Box<[Hittable; crate::NUM_OBJECTS]>),
}

pub struct Hit {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Hittable {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Self::Sphere(s) => s.hit(ray, t_min, t_max),
            Self::Many(hittables) => {
                let mut closest_so_far = t_max;
                let mut best_hit: Option<Hit> = None;
                let hittables = &**hittables;
                for hittable in hittables {
                    if let Some(hit) = hittable.hit(ray, t_min, t_max) {
                        if hit.t < closest_so_far {
                            closest_so_far = hit.t;
                            best_hit = Some(hit);
                        }
                    }
                }
                best_hit
            }
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    /// Does the ray hit this sphere?
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let origin_to_center = ray.origin - self.center;

        // Use the quadratic equation's discriminant to check how many places the ray intersects
        // this sphere.
        let a = ray.direction.dot(&ray.direction);
        let b = origin_to_center.dot(&ray.direction);
        let c = origin_to_center.dot(&origin_to_center) - self.radius.powf(2.0);
        let discriminant = b.powf(2.0) - (a * c);

        let hit_at = |t| {
            if t < t_max && t > t_min {
                let p = ray.point_at(t);
                Some(Hit {
                    t,
                    p,
                    normal: (p - self.center).scale(1.0 / self.radius),
                    material: self.material,
                })
            } else {
                None
            }
        };

        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;
            hit_at(t1).or_else(|| hit_at(t2))
        } else {
            None
        }
    }
}
