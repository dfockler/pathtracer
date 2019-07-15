extern crate image;

use image::{ImageBuffer};
use std::f32;

const SIZE: f32 = 512.0;

fn main() {
    let scene = vec!(
        Sphere { pos: Vector::new(0f32, 0f32, 400f32), radius: 50f32, color: (255, 0, 0) },
        // Sphere { pos: Vector::new(20f32, 45f32, 200f32), radius: 40f32, color: (0, 255, 0) },
    );

    let mut img = ImageBuffer::new(SIZE as u32, SIZE as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dir_x = (x as f32 - 511.0 / 2.0).round();
        let dir_y = (-(y as f32) + 511.0 / 2.0).round();

        *pixel = image::Rgba(pathtraced(Ray{ pos: Vector::new(0.0, 0.0, 0.0), dir: Vector::new(dir_x / 256.0, dir_y / 256.0, 1.0) }, &scene));
    }

    img.save("output.png").unwrap();
}

fn pathtraced(ray: Ray, scene: &Vec<Sphere>) -> [u8; 4] {
    let mut color = [0, 0, 0, 255];

    for sphere in scene {
        match intersect(&ray, &sphere) {
            Some(new_ray) => {
                color = [sphere.color.0, sphere.color.1, sphere.color.2, 255]
            },
            None => (),
        }
    }

    color
}

fn intersect(ray: &Ray, sphere: &Sphere) -> Option<Vector> {
    // Vector between the sphere center and the ray position
    let u = sphere.pos.sub(&ray.pos);

    // Dot product of the distance vector and the ray direction
    let v_dot_u = u.dot(&ray.dir);

    // Scale the normalized direction vector by the dot product
    let puv = ray.dir.scale(v_dot_u);

    // Find the distance between the sphere and the ray
    let distance = puv.dist(&sphere.pos);

    if distance < sphere.radius {
        // Distance between the projection and the sphere
        let c_2 = (distance.powi(2) + sphere.radius.powi(2)).sqrt();
        let new_ray = puv.sub(&ray.dir.scale(c_2));
        
        Some(new_ray)
    } else {
        None
    }
}

// Just use spheres for now
struct Sphere {
    pos: Vector,
    radius: f32,
    color: (u8, u8, u8)
}

#[derive(Debug)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z,
        }
    }

    fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn scale(&self, factor: f32) -> Vector {
        Vector {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    fn dist(&self, other: &Vector) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug)]
struct Ray {
    pos: Vector,
    dir: Vector,
}

// Probably more information need for this
struct Light {
    pos: (u32, u32, u32),
    mag: u32
}
