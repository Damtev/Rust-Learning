mod vec3;
mod color;
mod ray;
mod sphere;
mod hit;
mod world;
mod camera;
mod material;

use std::io::{stderr, Write};
use std::rc::Rc;
use rand::Rng;
use crate::color::Color;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use crate::world::World;
use crate::camera::Camera;
use crate::material::{Dielectric, Lambertian, Metal};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    // World
    let r: f64 = (std::f64::consts::PI / 4.0).cos();
    let mut world = World::new();

    let mat_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let sphere_left = Sphere::new(Point3::new(-r, 0.0, -1.0), r, mat_left);
    let sphere_right = Sphere::new(Point3::new(r, 0.0, -1.0), r, mat_right);

    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    // Camera
    let cam = Camera::new(90.0, ASPECT_RATIO);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    const WIDTH_DENOM: f64 = (IMAGE_WIDTH - 1) as f64;
    println!("{}", WIDTH_DENOM);

    let mut random = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Remaining lines: {:3}", j);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = random.gen();
                let random_v: f64 = random.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = cam.get_ray(u, v);
                pixel_color += ray.color(&world, MAX_DEPTH);
            }

            println!("{}", pixel_color.format(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Rendering is completed");
}
