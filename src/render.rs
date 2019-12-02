use image;
use std::path::Path;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub filename: &'static str,
    pub output_dir: &'static str,
}

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    /// Width of the image
    pub width: u32,
    /// Height of the image
    pub height: u32,
}

impl Renderer {
    /// pixel_at computes the color of the input pixel
    pub fn write<F>(&self, pixel_at: F)
    where
        F: Fn(Pixel) -> [u8; 3],
    {
        let mut img_buf = image::ImageBuffer::new(self.width, self.height);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            *pixel = image::Rgb(pixel_at(Pixel {
                x,
                y: self.height - y,
                width: self.width,
                height: self.height,
            }));
        }

        let path = Path::new(self.output_dir).join(self.filename);
        img_buf.save(path).unwrap();
    }
}
