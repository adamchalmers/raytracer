use crate::camera::Camera;
use crate::color::Color;
use crate::grid::{Grid, Point};
use crate::hittable::Hittable;
use crate::metrics::Metrics;
use crate::ray::Ray;
use crate::vector::Vec3;
use image;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::convert::TryInto;
use std::path::Path;
use std::time;

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub filename: &'static str,
    pub output_dir: &'static str,
    pub camera: Camera,
    /// Number of samples to use for antialiasing.
    pub samples: usize,
}

impl Renderer {
    /// `color_hit_by` computes the color of the object the ray hits.
    pub fn render_img<F, const NUM_PIXELS: usize>(
        &self,
        scene: &Hittable,
        color_hit_by: F,
        mut pixels: [[u8; 3]; NUM_PIXELS],
    ) -> Metrics
    where
        F: Fn(&Ray, &Hittable, u8) -> Color,
        F: std::marker::Sync,
    {
        let metrics = self.render(scene, color_hit_by, &mut pixels);
        self.output_img(&pixels);
        metrics
    }

    /// `color_hit_by` computes the color of the object the ray hits.
    pub fn render<F, const NUM_PIXELS: usize>(
        &self,
        scene: &Hittable,
        color_hit_by: F,
        pixels: &mut [[u8; 3]; NUM_PIXELS],
    ) -> Metrics
    where
        F: Fn(&Ray, &Hittable, u8) -> Color,
        F: std::marker::Sync,
    {
        // I'm storing the 2d image in a 1d vector because it's easier to iterate over.
        let grid = Grid {
            width: self.width,
            height: self.height,
        };

        let mut metrics = Metrics::new();
        metrics.rays_traced_total = (self.width * self.height * self.samples)
            .try_into()
            .unwrap();

        // Iterate over every pixel in the final image, in parallel
        let start = time::Instant::now();
        pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            let point = grid.to_point(i);
            // Sample a number of points inside the pixel, get each of their colors, and average them
            // all together. This is called "antialiasing" and helps the image look smoother.
            let avg_color: Vec3 = (0..self.samples)
                .into_iter()
                .map(|_| {
                    // Choose a random point inside this pixel
                    let mut rng = thread_rng();
                    let u = (point.x as f64 + rng.gen::<f64>()) / (self.width as f64);
                    let v =
                        ((self.height - point.y) as f64 + rng.gen::<f64>()) / (self.height as f64);

                    // Then get the ray from the camera to that point,
                    // check what color it hits.
                    let ray = self.camera.ray_to_point(u, v);
                    let color_at_this_point = color_hit_by(&ray, &scene, 0);

                    color_at_this_point.vec()
                })
                // Average the colour of all the points sampled from inside the pixel
                .sum::<Vec3>()
                .scale(1.0 / self.samples as f64);
            *pixel = Color::from(avg_color).to_rgb_gamma_corrected();
        });
        metrics.time_spent = start.elapsed();

        metrics
    }

    fn output_img<const NUM_PIXELS: usize>(&self, pixels: &[[u8; 3]; NUM_PIXELS]) {
        let mut img_buf = image::ImageBuffer::new(self.width as u32, self.height as u32);
        let grid = Grid {
            width: self.width,
            height: self.height,
        };
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let point = Point {
                x: x as usize,
                y: y as usize,
            };
            let bytes = pixels[grid.to_index(point)];
            *pixel = image::Rgb(bytes);
        }
        // Write the image to disk
        let path = Path::new(self.output_dir).join(self.filename);
        img_buf.save(path).unwrap();
    }
}

pub fn color_hit_by(ray: &Ray, scene: &Hittable, depth: u8) -> Color {
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

/// Render the nice blue/white background
fn background(r: &Ray) -> Color {
    let t = r.direction.unit().y * 0.5 + 1.0;
    let white = Color::new_uniform(1.0);
    let blue = Color::new(0.8, 1.0, 1.0);
    white.vec().interpolate(&blue.vec(), t).into()
}
