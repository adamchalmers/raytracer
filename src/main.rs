mod camera;
mod color;
mod grid;
mod hittable;
mod metrics;
mod ray;
mod render;
mod texture;
mod vector;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, Sphere};
use crate::ray::Ray;
use crate::render::Renderer;
use crate::vector::Vec3;

const NUM_ANTIALIAS_SAMPLES: usize = 100;
const FILENAME: &str = "fractal10.png";
const OUTPUT_DIR: &str = "output";
const NUM_OBJECTS: usize = 2;

fn main() {
    let camera = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::zero(),
    };

    let r = Renderer {
        width: 1000,
        height: 500,
        output_dir: OUTPUT_DIR,
        filename: FILENAME,
        camera,
        samples: NUM_ANTIALIAS_SAMPLES,
    };
    let metrics = r.render(&scene(), color_hit_by);
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
    Hittable::Many(Box::new([big_sphere, little_sphere]))
}

/// Render the nice blue/white background
fn background(r: &Ray) -> Color {
    let t = r.direction.unit().y * 0.5 + 1.0;
    let white = Color::new_uniform(1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    white.vec().interpolate(&blue.vec(), t).into()
}

fn color_hit_by(ray: &Ray, scene: &Hittable) -> Color {
    // What color should this pixel be?
    // If the ray hits an object:
    if let Some(hit) = scene.hit(&ray, 0.001, std::f64::MAX) {
        // It should reflect off that object, and we can calculate that reflection's colour recursively.
        let target = hit.normal + texture::random_in_unit_sphere();
        let reflected_ray = Ray {
            origin: hit.p,
            direction: target,
        };
        color_hit_by(&reflected_ray, &scene).scale(0.5)

    // Otherwise, it'll be the color of the background.
    } else {
        background(ray)
    }
}
