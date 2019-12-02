use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::vector::Vec3;
use image;
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
    /// pixel_at computes the color of the input pixel
    pub fn write<F>(&self, pixel_at: F)
    where
        F: Fn(&Ray) -> Color,
    {
        let mut img_buf = image::ImageBuffer::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let mut pixel_color = Vec3::new_uniform(0.0);
            for _ in 0..self.samples {
                let rx: f64 = rng.gen();
                let ry: f64 = rng.gen();
                let u = (x as f64 + rx) / (self.width as f64);
                let v = ((self.height - y) as f64 + ry) / (self.height as f64);
                let ray = self.camera.ray_to_point(u, v);
                pixel_color += pixel_at(&ray).vec();
            }
            pixel_color = pixel_color.scale(1.0 / self.samples as f64);

            let color: Color = pixel_color.into();
            *pixel = image::Rgb(color.to_rgb());
        }

        let path = Path::new(self.output_dir).join(self.filename);
        img_buf.save(path).unwrap();
    }
}
