mod color;
mod ray;
mod render;
mod sphere;
mod vector;

use crate::color::Color;
use crate::ray::Ray;
use crate::render::{Pixel, Renderer};
use crate::sphere::Sphere;
use crate::vector::Vec3;

fn main() {
    let r = Renderer {
        width: 1680,
        height: 1050,
        output_dir: "output",
        filename: "fractal4.png",
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
        x: -8.0,
        y: -5.0,
        z: -1.0,
    };
    let horizontal = Vec3 {
        x: 16.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 10.0,
        z: 0.0,
    };

    let u = p.x as f64 / p.width as f64;
    let v = p.y as f64 / p.height as f64;

    // The direction of a ray starting at the camera and ending at the pixel
    let direction = lower_left_corner + horizontal.scale(u) + vertical.scale(v);
    let ray = Ray { origin, direction };

    // Let's put a sphere in the middle of the scene.
    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.9,
    };

    // What color should this pixel be? Depends on if the ray hits an object in the scene.
    // And there's only one object, so we can check pretty easily!
    if sphere.hit(&ray) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        background(ray)
    }
}
