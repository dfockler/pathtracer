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
    [x as u8, y as u8, 0, 255]
}

struct Camera {
    pos: (u32, u32),
    dir: (u32, u32)
}

// Just use spheres for now
struct Sphere {
    pos: (u32, u32),
    radius: u32,
    color: (u8, u8, u8)
}

struct Ray {
    pos: (u32, u32),
    dir: (u32, u32),
    mag: u32
}

// Probably more information need for this
struct Light {
    pos: (u32, u32),
    mag: u32
}
