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
    pub fn render<F>(&self, scene: &Hittable, color_hit_by: F) -> Metrics
    where
        F: Fn(&Ray, &Hittable) -> Color,
        F: std::marker::Sync,
    {
        // I'm storing the 2d image in a 1d vector because it's easier to iterate over.
        let grid = Grid::new(self.width, self.height);
        let mut metrics = Metrics::new();

        metrics.rays_traced_total = (self.width * self.height * self.samples)
            .try_into()
            .unwrap();

        // Iterate over every pixel in the final image, in parallel
        let start = time::Instant::now();
        let pixels: Vec<[u8; 3]> = (0..grid.len())
            .into_par_iter()
            .map(|i| {
                let p = grid.to_point(i);

                // Sample a number of points inside the pixel, get each of their colors, and average them
                // all together. This is called "antialiasing" and helps the image look smoother.
                let avg_color: Vec3 = (0..self.samples)
                    .into_par_iter()
                    .map(|_| {
                        // Choose a random point inside this pixel
                        let mut rng = thread_rng();
                        let u = (p.x as f64 + rng.gen::<f64>()) / (self.width as f64);
                        let v =
                            ((self.height - p.y) as f64 + rng.gen::<f64>()) / (self.height as f64);

                        // Then get the ray from the camera to that point,
                        // check what color it hits.
                        let ray = self.camera.ray_to_point(u, v);
                        let color_at_this_point = color_hit_by(&ray, &scene);

                        color_at_this_point.vec()
                    })
                    // Average the colour of all the points sampled from inside the pixel
                    .sum::<Vec3>()
                    .scale(1.0 / self.samples as f64);
                Color::from(avg_color).to_rgb_gamma_corrected()
            })
            .collect();
        metrics.time_spent = start.elapsed();

        self.output_img(pixels, grid);
        metrics
    }

    fn output_img(&self, pixels: Vec<[u8; 3]>, grid: Grid) {
        let mut img_buf = image::ImageBuffer::new(self.width as u32, self.height as u32);
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
