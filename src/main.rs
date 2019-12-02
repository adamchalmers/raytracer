mod color;
mod hittable;
mod ray;
mod render;
mod vector;

use crate::color::Color;
use crate::hittable::{Hittable, Sphere};
use crate::ray::Ray;
use crate::render::{Pixel, Renderer};
use crate::vector::Vec3;

fn main() {
    let r = Renderer {
        width: 200,
        height: 100,
        output_dir: "output",
        filename: "fractal6.png",
    };
    r.write(render)
}

/// Render the nice blue/white background
fn background(r: Ray) -> Color {
    let t = r.direction.unit().y * 0.5 + 1.0;
    let white = Color::new_uniform(1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    white.interpolate(&blue, t)
}

fn render(p: Pixel) -> Color {
    let origin = Vec3::new_uniform(0.0);
    let lower_left_corner = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };
    let horizontal = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };

    let u = p.x as f64 / p.width as f64;
    let v = p.y as f64 / p.height as f64;

    // The direction of a ray starting at the camera and ending at the pixel
    let direction = lower_left_corner + horizontal.scale(u) + vertical.scale(v);
    let ray = Ray { origin, direction };

    // Let's put a sphere in the middle of the scene.
    let little_sphere = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });
    let big_sphere = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    });
    let scene = Hittable::Many(vec![little_sphere, big_sphere]);

    // What color should this pixel be? Depends on if the ray hits an object in the scene.
    if let Some(hit) = scene.hit(&ray, 0.0, std::f64::MAX) {
        (hit.normal + Vec3::new_uniform(1.0)).scale(0.5)
    } else {
        background(ray)
    }
}
