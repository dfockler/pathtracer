extern crate image;

use image::{ImageBuffer};
use image::math::utils;
use std::f32;

fn main() {
    let scene = vec!(
        Sphere { pos: (0f32, 0f32, 400f32), radius: 200f32, color: (255, 0, 0) }
    );

    let mut img = ImageBuffer::new(512, 512);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dir_x = (x as f32 - 511.0 / 2.0).round();
        let dir_y = (-(y as f32) + 511.0 / 2.0).round();

        *pixel = image::Rgba(pathtraced(Ray{ pos: (0.0, 0.0, 0.0), dir: (dir_x / 256.0, dir_y / 256.0, 1.0) }, &scene));
    }

    img.save("output.png").unwrap();
}

fn dist(x_1: f32, y_1: f32, z_1: f32, x_2: f32, y_2: f32, z_2: f32) -> f32 {
    ((x_1 - x_2).powi(2) + (y_1 - y_2).powi(2) + (z_1 - z_2).powi(2)).sqrt()
}

fn pathtraced(ray: Ray, scene: &Vec<Sphere>) -> [u8; 4] {
    let sphere = scene.first().unwrap();

    // Distance between the ray origin and the sphere center
    // for x
    let u = (sphere.pos.0 - ray.pos.0, sphere.pos.1 - ray.pos.1, sphere.pos.2 - ray.pos.2);
    let v_dot_u = u.0 * ray.dir.0 + u.1 * ray.dir.1 + u.2 * ray.dir.2;
    let puv = (ray.dir.0 * v_dot_u, ray.dir.1 * v_dot_u, ray.dir.2 * v_dot_u);
    let distance = dist(puv.0, puv.1, puv.2, sphere.pos.0, sphere.pos.1, sphere.pos.2);

    if distance < sphere.radius {
        [sphere.color.0, sphere.color.1, sphere.color.2, 255]
    } else {
        [0, 0, 0, 255]
    }
}

// Just use spheres for now
struct Sphere {
    pos: (f32, f32, f32),
    radius: f32,
    color: (u8, u8, u8)
}

#[derive(Debug)]
struct Ray {
    pos: (f32, f32, f32),
    dir: (f32, f32, f32)
}

// Probably more information need for this
struct Light {
    pos: (u32, u32, u32),
    mag: u32
}
