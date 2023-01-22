mod vec3;
mod color;
mod ray;
mod sphere;
mod hit;

use std::io::{stderr, Write};
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);


    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    const WIDTH_DENOM: f64 = (IMAGE_WIDTH - 1) as f64;
    println!("{}", WIDTH_DENOM);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Remaining lines: {:3}", j);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin,
                             lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = r.color();

            println!("{}", pixel_color.format());
        }
    }

    eprintln!("Rendering is completed");
}
