use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::metrics::Metrics;
use crate::ray::Ray;
use crate::vector::Vec3;
use image;
use rand::rngs::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};
use rayon::prelude::*;
use std::path::Path;
use std::time;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub filename: &'static str,
    pub output_dir: &'static str,
    pub camera: Camera,
    /// Number of samples to use for antialiasing.
    pub samples: u32,
}

impl Renderer {
    /// `color_hit_by` computes the color of the object the ray hits.
    pub fn write<F>(&self, scene: &Hittable, color_hit_by: F) -> Metrics
    where
        F: Fn(&Ray, &Hittable) -> Color,
        F: std::marker::Sync,
    {
        let mut img_buf = image::ImageBuffer::new(self.width, self.height);
        let mut metrics = Metrics::new();
        let start = time::Instant::now();

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            // Sample a number of points inside the pixel, get each of their colors, and average them
            // all together. This is called "antialiasing" and helps the image look smoother.
            let avg_color: Vec3 = (0..self.samples)
                .into_par_iter()
                .map(|_| {
                    let mut rng = thread_rng();
                    let rx = rng.gen::<f64>();
                    let ry = rng.gen::<f64>();

                    // Choose a random point inside this pixel
                    let u = (x as f64 + rx) / (self.width as f64);
                    let v = ((self.height - y) as f64 + ry) / (self.height as f64);

                    // Then get the ray from the camera to that point,
                    // check what color it hits.
                    let ray = self.camera.ray_to_point(u, v);
                    let color_at_this_point = color_hit_by(&ray, &scene);

                    // To do antialiasing, average this color with all the other points inside this pixel.
                    color_at_this_point.vec()
                })
                .sum::<Vec3>()
                .scale(1.0 / self.samples as f64);
            *pixel = image::Rgb(Color::from(avg_color).to_rgb_gamma_corrected());
        }

        metrics.time_spent = start.elapsed();
        metrics.rays_traced_total = (self.width * self.height * self.samples).into();

        // Write the image to disk
        let path = Path::new(self.output_dir).join(self.filename);
        img_buf.save(path).unwrap();
        metrics
    }
}
