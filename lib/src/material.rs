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

fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        // Generate a vector where all components are between -1 and 1.
        let p = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3::new_uniform(1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_point_in_unit_sphere() {
        for _ in 0..1000000 {
            random_point_in_unit_sphere();
        }
    }
}
