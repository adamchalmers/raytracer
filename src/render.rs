use image;
use std::path::Path;

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub filename: &'static str,
    pub output_dir: &'static str,
}

impl Renderer {
    pub fn write<F>(&self, pixel_at: F)
    where
        F: Fn(u32, u32) -> [u8; 3],
    {
        let mut img_buf = image::ImageBuffer::new(self.width, self.height);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            *pixel = image::Rgb(pixel_at(x, y));
        }

        let path = Path::new(self.output_dir).join(self.filename);
        img_buf.save(path).unwrap();
    }
}
