use rand::prelude::*;

fn main() {
    let spheres = vec!(//Scene: radius, position, emission, color, material
        Sphere { radius: 1e5, position: Vector::new(1e5+1,40.8,81.6),   emission: Vector::blank(), color: Vector::new(0.75,0.25,0.25), material: ReflectanceType::Diffuse },//Left
        Sphere { radius: 1e5, position: Vector::new(-1e5+99,40.8,81.6), emission: Vector::blank(), color: Vector::new(0.25,0.25,0.75), material: ReflectanceType::Diffuse},//Rght
        Sphere { radius: 1e5, position: Vector::new(50.0,40.8, 1e5),      emission: Vector::blank(), color: Vector::new(0.75,0.75,0.75), material: ReflectanceType::Diffuse},//Back
        Sphere { radius: 1e5, position: Vector::new(50.0,40.8,-1e5+170.0),  emission: Vector::blank(), color: Vector::blank(),            material: ReflectanceType::Diffuse},//Frnt
        Sphere { radius: 1e5, position: Vector::new(50.0, 1e5, 81.6),     emission: Vector::blank(), color: Vector::new(0.75,0.75,0.75), material: ReflectanceType::Diffuse},//Botm
        Sphere { radius: 1e5, position: Vector::new(50.0,-1e5+81.6,81.6), emission: Vector::blank(), color: Vector::new(0.75,0.75,0.75), material: ReflectanceType::Diffuse},//Top
        Sphere { radius: 16.5, position: Vector::new(27.0,16.5,47.0),       emission: Vector::blank(), color: Vector::new(1.0,1.0,1.0).scale(0.999),  material: ReflectanceType::Specular},//Mirr
        Sphere { radius: 16.5, position: Vector::new(73.0,16.5,78.0),       emission: Vector::blank(), color: Vector::new(1.0,1.0,1.0).scale(0.999),  material: ReflectanceType::Refractive},//Glas
        Sphere { radius: 600.0, position: Vector::new(50.0,681.6-0.27,81.6), emission: Vector::new(12.0,12.0,12.0), color: Vector::blank(),  material: ReflectanceType::Diffuse},//Lite
    );

    let width = 1024;
    let height = 768;
    let samples = 1;

    let camera = Ray { position: Vec::new(50.0, 52.0, 295.6), direction: Vec::new(0.0, -0.042612, -1.0).norm() };
    let cx = Vec::new(width * 0.5135 / height, 0.0, 0.0);
    let cy = (cx.div(camera.direction)).norm().scale(0.5135);
    let mut r = Vec::blank();
    let mut output = Vec<Vector>::with_capacity(width*height);

    for y in 0..height {
        for x in 0..width {
            for sy in 0..2 {
                let i = (height - y - 1) * width + x;

                for sx in 0..2 {
                    for s in 0..samples {
                        let r1 = 2.0 * rand::random::rand::randomom<f32>();
                        let r2 = 2.0 * rand::random::rand::randomom<f32>();

                        let dx = if r1 < 1.0 {
                            r1.sqrt() - 1.0
                        } else {
                            1.0 - (2.0-r1).sqrt()
                        };

                        let dy = if r2 < 1.0 {
                            r2.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r2).sqrt()
                        };

                        let d = Vec::new(
                            cx.scale( ((sx + 0.5 + dx) / 2.0 + x) / width - 0.5 ).add(cy.scale(((sy + 0.5 + dy) / 2.0 + y) / height - 0.5)).add(camera.direction),
                            0.0,
                            0.0,
                        );

                        r = r.add(
                            radiance(
                                Ray { origin: camera.origin.add(d).scale(140.0), direction: d.norm() },
                                &spheres,
                                0
                            ).scale( 1.0 / samples );
                        );
                    }

                    output.index(i) = output.index(i).add(Vec::new(clamp(r.x), clamp(r.y), clamp(r.z)).scale(0.25));

                    r = Vec::blank();
                }
            }
        }
    }

    // Write out to a file.
}

fn radiance(ray: Ray, spheres: &Vec<Sphere>, depth: usize) -> Vector {
    let mut distance = 0.0;
    let mut id = 0;


    if !intersect(&ray, spheres, distance, id) {
        return Vector::blank();
    }

    let sphere = spheres[id];

    let x = ray.origin.add(ray.direction).scale(t);
    let n = x.sub(sphere.position).norm();
    let nl = if n.dot(ray.direction) < 0 {
        n
    } else {
        n.scale(-1.0)
    };

    let mut f = sphere.color;

    let p = if f.x > f.y && f.x > f.z {
        f.x
    } else if f.y > f.z {
        f.y
    } else {
        f.z
    };

    if depth > 5 {
        if rand::random() < p {
            f = f.scale(1.0 / p);
        } else {
            return sphere.emission;
        }
    }

    match sphere.material {
        ReflectanceType::Diffuse => {

        },
        ReflectanceType::Specular => {

        },
        _ => (),
    }

}

fn clamp(x: f32) {
    if x < 0.0 {
        0.0
    }

    if x > 1.0 {
        1.0
    } else {
        x
    }
}


fn intersect(ray: &Ray, scene: &Vec<Sphere>, distance: &mut f32, id: &mut usize) -> bool {
    let mut distance = std::f32::INFINITY;
    for index, sphere in scene.enumerate() {
        let d = sphere.intersect(ray);
        if d < distance {
            distance = d;
            id = index;
        }
    }

    distance < std::f32::INFINITY;
}

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

    fn blank() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn norm(&self) -> Vector {
        self.scale(
            1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        )
    }

    fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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
        let x = (self.x - other.x).powi(2);
        let y = (self.y - other.y).powi(2);
        let z = (self.z - other.z).powi(2);
        ( x+y+z ).sqrt()
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

    fn mult(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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

struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
}

enum ReflectanceType {
    Diffuse,
    Specular,
    Refractive,
}

struct Sphere {
    rad: f32,
    position: Vector,
    emission: Vector,
    color: Vector,
    material: ReflectanceType,
}

impl Sphere {
    fn intersect(&self, &ray: Ray) -> f32 {
        let op = self.position.sub(ray.origin);
        let epsilon = 0.00001;
        let b = op.dot(ray.direction);
        let mut det = b.mult(b) - op.dot(op) + self.rad.powi(2);

        if det < 0.0 {
            return 0.0;
        } else {
            det = det.sqrt();
        }

        if b - det > eps  {
            return b - det;
        }
        
        if b + det > eps {
            return b + det;
        }

        0.0
    }
}
