mod average;
mod camera;
mod color;
mod hittable;
mod metrics;
mod ray;
mod render;
mod texture;
mod vector;

use crate::average::Scalable;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, Sphere};
use crate::metrics::Metrics;
use crate::ray::Ray;
use crate::render::Renderer;
use crate::vector::Vec3;
use rand::rngs::ThreadRng;

const NUM_ANTIALIAS_SAMPLES: u32 = 100;
const FILENAME: &str = "fractal10.png";
const OUTPUT_DIR: &str = "output";

fn main() {
    let camera = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::zero(),
    };

    let r = Renderer {
        width: 500,
        height: 250,
        output_dir: OUTPUT_DIR,
        filename: FILENAME,
        camera,
        samples: NUM_ANTIALIAS_SAMPLES,
    };
    let metrics = r.write(&scene(), color_hit_by);
    eprintln!("{}", metrics.describe());
    eprintln!("{:?}", metrics);
}

fn scene() -> Hittable {
    // Let's put a sphere in the middle of the scene.
    let little_sphere = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });
    // And a big grassy plain
    let big_sphere = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    });
    Hittable::Many(vec![big_sphere, little_sphere])
}

/// Render the nice blue/white background
fn background(r: &Ray) -> Color {
    let t = r.direction.unit().y * 0.5 + 1.0;
    let white = Color::new_uniform(1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    white.vec().interpolate(&blue.vec(), t).into()
}

fn color_hit_by(ray: &Ray, scene: &Hittable, rng: &mut ThreadRng, metrics: &mut Metrics) -> Color {
    // What color should this pixel be?
    // If the ray hits an object:
    if let Some(hit) = scene.hit(&ray, 0.001, std::f64::MAX) {
        // It should reflect off that object, and we can calculate that reflection's colour recursively.
        let target = hit.normal + texture::random_in_unit_sphere(rng);
        let reflected_ray = Ray {
            origin: hit.p,
            direction: target,
        };
        color_hit_by(&reflected_ray, &scene, rng, metrics).scale(0.5)

    // Otherwise, it'll be the color of the background.
    } else {
        background(ray)
    }
}
