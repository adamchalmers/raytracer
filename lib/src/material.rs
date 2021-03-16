use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vector::Vec3;
use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub enum Material {
    Diffuse { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
}

pub struct Scatter {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        match self {
            Self::Diffuse { albedo } => {
                let target = hit.p + hit.normal + random_point_in_unit_sphere();
                Some(Scatter {
                    attenuation: *albedo,
                    scattered: Ray {
                        origin: hit.p,
                        direction: target - hit.p,
                    },
                })
            }

            Self::Metal { albedo, fuzz } => {
                let reflected = ray_in.direction.unit().reflect(&hit.normal);
                if reflected.dot(&hit.normal) > 0.0 {
                    Some(Scatter {
                        attenuation: *albedo,
                        scattered: Ray {
                            origin: hit.p,
                            direction: reflected + random_point_in_unit_sphere() * *fuzz,
                        },
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        // Pick a random line which might intersect the unit sphere
        let x = rng.gen::<f64>() * 2.0 - 1.0;
        let y = rng.gen::<f64>() * 2.0 - 1.0;
        // Pick a random point along that line, such that if the line intersects,
        // the point will be inside the sphere.
        let z = rng.gen::<f64>() * (1.0 - x.powf(2.0) - y.powf(2.0)).sqrt();
        // Check the assumption
        let p = Vec3::new(x, y, z);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
