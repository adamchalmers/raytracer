use crate::color::Color;
use crate::grid::Point;
use crate::hittable::Hittable;
use crate::metrics::Metrics;
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::{camera::Camera, grid::Grid};
use rand::{thread_rng, Rng};
use std::path::Path;
use std::time;

pub struct Renderer {
    pub filename: &'static str,
    pub output_dir: &'static str,
    pub camera: Camera,
    /// Number of samples to use for antialiasing.
    pub samples: usize,
}

impl Renderer {
    /// Computes the image and then writes it to the filesystem as a .png image.
    /// The `color_hit_by` arg computes the color of the object the ray hits.
    pub fn render_img<F, const W: usize, const H: usize>(
        &self,
        scene: &Hittable,
        color_hit_by: F,
        mut pixels: Grid<[u8; 3], W, H>,
    ) -> Metrics
    where
        F: Sync + Fn(&Ray, &Hittable, u8) -> Color,
        [u8; W * H]: Sized,
    {
        let metrics = self.render(scene, color_hit_by, &mut pixels);
        self.output_img(pixels);
        metrics
    }

    /// Perform the actual ray tracing of the scene.
    /// `scene` is a composition of all objects in the scene.
    /// `color_hit_by` computes the color of whichever object the ray hits.
    /// `pixels` is the 2d pixel grid.
    pub fn render<F, const W: usize, const H: usize>(
        &self,
        scene: &Hittable,
        color_hit_by: F,
        pixels: &mut Grid<[u8; 3], W, H>,
    ) -> Metrics
    where
        F: Sync + Fn(&Ray, &Hittable, u8) -> Color,
        [u8; W * H]: Sized,
    {
        let total_rays = pixels.size() * self.samples;
        let mut metrics = Metrics::new(total_rays);

        let start = time::Instant::now();
        let height = pixels.height();
        let width = pixels.width();
        pixels.set_all_parallel(|Point { x, y }| {
            let mut rng = thread_rng();
            let x = x as f64;
            let dy = (height - y) as f64;

            // Sample a number of points inside the pixel, get each of their colors, and average them
            // all together. This is called "antialiasing" and helps the image look smoother.
            let sample_rays = (0..self.samples).into_iter().map(|_| {
                // Choose a random point inside this pixel
                let u = (x + rng.gen::<f64>()) / width as f64;
                let v = (dy + rng.gen::<f64>()) / height as f64;

                // Then get the ray from the camera to that point,
                // check what color it hits.
                let ray = self.camera.ray_to_point(u, v);
                let color_at_this_point = color_hit_by(&ray, &scene, 0);

                color_at_this_point.vec()
            });

            // Average the colour of all the points sampled from inside the pixel
            let avg_color = sample_rays.sum::<Vec3>().scale(1.0 / self.samples as f64);
            Color::from(avg_color).to_rgb_gamma_corrected()
        });
        metrics.time_spent = start.elapsed();
        metrics
    }

    fn output_img<const W: usize, const H: usize>(&self, pixels: Grid<[u8; 3], W, H>)
    where
        [u8; W * H]: Sized,
    {
        let mut img_buf = image::ImageBuffer::new(pixels.width() as u32, pixels.height() as u32);
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let color = pixels.get_at(&Point {
                x: x as usize,
                y: y as usize,
            });
            *pixel = image::Rgb(*color);
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
