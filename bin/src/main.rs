use raytracer::{
    camera::Camera,
    grid::Grid,
    hittable::{Hittable, Sphere},
    material::Material,
    render::{color_hit_by, Renderer},
    vector::Vec3,
};

const NUM_ANTIALIAS_SAMPLES: usize = 200;
const FILENAME: &str = "fractal15.png";
const OUTPUT_DIR: &str = "output";
const IMG_SCALE: usize = 400;
const WIDTH: usize = 2 * IMG_SCALE;
const HEIGHT: usize = IMG_SCALE;

fn main() {
    let camera = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::zero(),
    };

    let r = Renderer {
        output_dir: OUTPUT_DIR,
        filename: FILENAME,
        camera,
        samples: NUM_ANTIALIAS_SAMPLES,
    };
    let pixels: Grid<[u8; 3], WIDTH, HEIGHT> = Default::default();
    let metrics = r.render_img(&scene(), color_hit_by, pixels);
    eprintln!("{}", metrics.describe());
    eprintln!("{:?}", metrics);
}

fn scene() -> Hittable {
    // Let's put a sphere in the middle of the scene.
    let little_sphere = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Material::Diffuse {
            albedo: Vec3::new(0.8, 0.3, 0.8),
        },
    });
    // And a big grassy plain
    let big_sphere = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: Material::Diffuse {
            albedo: Vec3::new(0.2, 0.2, 0.2),
        },
    });
    let right = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Material::Metal {
            albedo: Vec3::new(0.3, 0.7, 0.7),
            fuzz: 0.9,
        },
    });
    let left = Hittable::Sphere(Sphere {
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Material::Metal {
            albedo: Vec3::new(0.8, 0.8, 0.8),
            fuzz: 0.1,
        },
    });
    Hittable::Many(vec![big_sphere, little_sphere, left, right])
}
