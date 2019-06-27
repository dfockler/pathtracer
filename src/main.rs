extern crate image;

use image::{ImageBuffer};

fn main() {
    let mut img = ImageBuffer::new(512, 512);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba(pathtraced(x, y));
    }

    img.save("output.png").unwrap();
}

fn pathtraced(x: u32, y: u32) -> [u8; 4] {
    let disc = Sphere { pos: (256, 256, 256), radius: 30, color: (255, 0, 0) };
    [x as u8, y as u8, 0, 255]
}

fn follow_path(ray: Ray, depth: u32) -> [u8; 3] {
    let max_depth = 30;

    if depth > max_depth {
        return [0, 0, 0];
    }

    if ray.intersects_object() {
        let object = ray.object_intersected();
        object.color
    } else {
        [0, 0, 0]
    }
}

// Just use spheres for now
struct Sphere {
    pos: (u32, u32, u32),
    radius: u32,
    color: (u8, u8, u8)
}

struct Ray {
    pos: (u32, u32)
}

impl Ray {
    fn intersects_object() -> bool {
        for object in scene.objects {
            if 
        }
    }
}

// Probably more information need for this
struct Light {
    pos: (u32, u32, u32),
    mag: u32
}
