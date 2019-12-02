mod average;
mod camera;
mod color;
mod hittable;
mod ray;
mod render;
mod vector;

use crate::average::Scalable;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, Sphere};
use crate::ray::Ray;
use crate::render::Renderer;
use crate::vector::Vec3;

const NUM_ANTIALIAS_SAMPLES: u32 = 10;

fn main() {
    let camera = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::zero(),
    };

    let r = Renderer {
        width: 200,
        height: 100,
        output_dir: "output",
        filename: "fractal7.png",
        camera,
        samples: NUM_ANTIALIAS_SAMPLES,
    };
    r.write(color_hit_by)
}

/// Render the nice blue/white background
fn background(r: &Ray) -> Color {
    let t = r.direction.unit().y * 0.5 + 1.0;
    let white = Color::new_uniform(1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    white.vec().interpolate(&blue.vec(), t).into()
}

fn color_hit_by(ray: &Ray) -> Color {
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
    let scene = Hittable::Many(vec![big_sphere, little_sphere]);

    // What color should this pixel be?
    // If the ray hits an object, it'll be that object's colour.
    if let Some(hit) = scene.hit(&ray, 0.0, std::f64::MAX) {
        (hit.normal + Vec3::new_uniform(1.0)).scale(0.5).into()
    // Otherwise, it'll be the color of the background.
    } else {
        background(ray)
    }
}
