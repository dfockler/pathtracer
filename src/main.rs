extern crate image;
extern crate rand;

use rand::prelude::*;

use image::{ImageBuffer};
use std::f32;

const SIZE: f32 = 512.0;
const PI: u8 = 3;

fn main() {
    let scene = vec!(
        Sphere { pos: Vector::new(0f32, 10f32, 50f32), radius: 10f32, reflectance: [1, 1, 1, 6], color: [1, 1, 1, 0] },
        Sphere { pos: Vector::new(0f32, 0f32, 100f32), radius: 50f32, reflectance: [1, 1, 1, 4], color: [1, 1, 3, 255] },
    );

    let mut img = ImageBuffer::new(SIZE as u32, SIZE as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x_shift = (x as f32 - 511.0 / 2.0).round();
        let y_shift = (-(y as f32) + 511.0 / 2.0).round();

        let start = Vector::new(0.0, 0.0, 0.0);
        let end = Vector::new(x_shift, y_shift, 256.0);
        let dist = start.dist(&end);

        *pixel = image::Rgba(trace_path(Ray{ 
            pos: Vector::new(x_shift, y_shift, 0.0), 
            dir: end.div(dist),
        }, 0, &scene));
    }

    img.save("output.png").unwrap();
}

fn trace_path(ray: Ray, depth: u8, scene: &Vec<Sphere>) -> [u8; 4] {
    if depth >= 8 {
        return [0, 0, 0, 255];
    }

    let value = ray.hit(&scene);

    if value.is_none() {
        return [0, 0, 0, 255];
    }

    let (sphere, new_ray) = value.unwrap();

    // @TODO: Figure this part out now
    let brdf = [
        sphere.reflectance[0] / PI,
        sphere.reflectance[1] / PI,
        sphere.reflectance[2] / PI,
        sphere.reflectance[3] / PI,
    ];

    let emittance = sphere.color;

    let incoming = trace_path(new_ray, depth + 1, &scene);

    [
        emittance[0].saturating_add(brdf[0].saturating_mul(incoming[0])),
        emittance[1].saturating_add(brdf[1].saturating_mul(incoming[1])),
        emittance[2].saturating_add(brdf[2].saturating_mul(incoming[2])),
        emittance[3].saturating_add(brdf[3].saturating_mul(incoming[3])),
    ]
}

fn intersect<'a>(ray: &Ray, sphere: &'a Sphere) -> Option<(&'a Sphere, Ray)> {
    // Vector between the sphere center and the ray position
    let proj_length = sphere.pos.dot(&ray.dir);
    let intercept = ray.dir.scale(proj_length);
    let intercept_length = sphere.pos.dist(&intercept);

    if intercept_length <= sphere.radius {
        let ray_intercept = ray.pos.dist(&intercept);
        let intercept_side = (sphere.radius.powi(2) - intercept_length.powi(2)).sqrt();
        let t1 = ray_intercept - intercept_side;
        // let t2 = ray_intercept + intercept_side;
        let t1v = ray.dir.scale(t1);
        let dir = t1v.cross(&sphere.pos).div(sphere.radius);
        // let t2v = ray.dir.scale(t2);
        
        // @TODO: Verify the normal of the new ray is correct
        Some((&sphere, Ray { pos: t1v, dir: dir }))
    } else {
        None
    }
}

// Just use spheres for now
#[derive(Debug)]
struct Sphere {
    pos: Vector,
    radius: f32,
    reflectance: [u8; 4],
    color: [u8; 4]
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

    fn div(&self, factor: f32) -> Vector {
        Vector {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
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

    fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

#[derive(Debug)]
struct Ray {
    pos: Vector,
    dir: Vector,
}

impl Ray {
    fn hit<'a>(&self, scene: &'a Vec<Sphere>) -> Option<(&'a Sphere, Ray)> {
        scene.iter().map(|sphere| intersect(self, &sphere)).find(|data| data.is_some()).unwrap_or(None)
    }
}

// Probably more information need for this
struct Light {
    pos: (u32, u32, u32),
    mag: u32
}
