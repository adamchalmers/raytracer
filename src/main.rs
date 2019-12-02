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

fn background(r: Ray) -> Color {
    let t = r.direction.unit().y * 0.5 + 1.0;
    let white = Color::new_uniform(1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    white.interpolate(&blue, t)
}

fn render(p: Pixel) -> [u8; 3] {
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

    let direction = lower_left_corner + horizontal.scale(u) + vertical.scale(v);
    let ray = Ray { origin, direction };
    let sphere = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.9,
    };
    let color = if sphere.hit(&ray) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        background(ray)
    };
    color.to_rgb()
}
