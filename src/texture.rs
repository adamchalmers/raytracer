use crate::average::Scalable;
use crate::vector::Vec3;
use rand::rngs::ThreadRng;
use rand::Rng;

/// Generate a vector where all components are between -1 and 1.
pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen(), rng.gen(), rng.gen()).scale(2.0) - Vec3::new_uniform(1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_in_unit_sphere() {
        let mut rng = rand::thread_rng();
        let p = random_in_unit_sphere(&mut rng);
        assert!(p.squared_length() < 1.0);
    }
}