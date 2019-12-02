mod ray;
mod render;
mod vector;

use render::Renderer;
use vector::Vec3;

const IMG_WIDTH: u32 = 200;
const IMG_HEIGHT: u32 = 100;

fn main() {
    let r = Renderer {
        width: IMG_WIDTH,
        height: IMG_HEIGHT,
        output_dir: "output",
        filename: "fractal2.png",
    };
    r.write(render)
}

fn render(x: u32, y: u32) -> [u8; 3] {
    Vec3 {
        x: (x as f64) / (IMG_WIDTH as f64),
        y: (y as f64) / (IMG_HEIGHT as f64),
        z: 0.2,
    }
    .to_rgb()
}
