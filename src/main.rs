mod camera;
mod color;
mod grid;
mod hittable;
mod material;
mod metrics;
mod ray;
mod render;
mod vector;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, Sphere};
use crate::material::Material;
use crate::ray::Ray;
use crate::render::Renderer;
use crate::vector::Vec3;

const NUM_ANTIALIAS_SAMPLES: usize = 100;
const FILENAME: &str = "fractal11.png";
const OUTPUT_DIR: &str = "output";
const NUM_OBJECTS: usize = 2;
const IMG_SCALE: usize = 400;

fn main() {
    let camera = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::zero(),
    };

    let r = Renderer {
        width: 2 * IMG_SCALE,
        height: 1 * IMG_SCALE,
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
        material: Material::Diffuse {
            albedo: Vec3::new_uniform(0.9),
        },
    });
    // And a big grassy plain
    let big_sphere = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: Material::Diffuse {
            albedo: Vec3::new(0.3, 0.5, 0.3),
        },
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

fn color_hit_by(ray: &Ray, scene: &Hittable, depth: u8) -> Color {
    // What color should this pixel be?
    // If the ray hits an object:
    if let Some(hit) = scene.hit(&ray, 0.001, std::f64::MAX) {
        // It should reflect off that object, and we can calculate that reflection's colour recursively.
        // I tried converting this to an iteration or a tail-recursion; neither affected performance,
        // so I stuck with the plain old recursion, because I thought it was more readable.

        if depth < 50 {
            if let Some(scatter) = hit.material.scatter(&ray, &hit) {
                Color::from(
                    color_hit_by(&scatter.scattered, &scene, depth + 1).vec() * scatter.attenuation,
                )
            } else {
                Color::new_uniform(0.0)
            }
        } else {
            Color::new_uniform(0.0)
        }

    // Otherwise, it'll be the color of the background.
    } else {
        background(ray)
    }
}
