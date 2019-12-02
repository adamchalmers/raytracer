use crate::average::Average;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vector::Vec3;
use image;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::path::Path;

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
    pub fn write<F>(&self, scene: &Hittable, color_hit_by: F)
    where
        F: Fn(&Ray, &Hittable, &mut ThreadRng, u8) -> Color,
    {
        let mut img_buf = image::ImageBuffer::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            // Sample a number of points inside the pixel, get each of their colors, and average them
            // all together. This is called "antialiasing" and helps the image look smoother.
            let mut avg_color = Average::<Vec3>::new();
            for _ in 0..self.samples {
                // Choose a random point inside this pixel
                let rx: f64 = rng.gen();
                let ry: f64 = rng.gen();
                let u = (x as f64 + rx) / (self.width as f64);
                let v = ((self.height - y) as f64 + ry) / (self.height as f64);

                // Then get the ray from the camera to that point,
                // check what color it hits.
                let ray = self.camera.ray_to_point(u, v);
                let color_at_this_point = color_hit_by(&ray, &scene, &mut rng, 0);

                // To do antialiasing, average this color with all the other points inside this pixel.
                avg_color.push(color_at_this_point.vec());
            }

            // Write the final pixel color into the image.
            let antialiased_pixel_color = avg_color.average();
            *pixel = image::Rgb(Color::from(antialiased_pixel_color).to_rgb_gamma_corrected());
        }

        // Write the image to disk
        let path = Path::new(self.output_dir).join(self.filename);
        img_buf.save(path).unwrap();
    }
}
