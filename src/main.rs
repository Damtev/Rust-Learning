mod vec3;
mod color;

use std::io::{stderr, Write};
use crate::color::Color;

fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    const WIDTH_DENOM: f64 = (IMAGE_WIDTH - 1) as f64;
    const HEIGHT_DENOM: f64 = (IMAGE_HEIGHT - 1) as f64;
    println!("{}", WIDTH_DENOM);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Remaining lines: {:3}", j);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / WIDTH_DENOM;
            let g = (j as f64) / HEIGHT_DENOM;
            let b = 0.25;

            let color = Color::new(r, g, b);

            println!("{}", color.format_color());
        }
    }

    eprintln!("Rendering is completed");
}
