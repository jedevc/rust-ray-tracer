mod hit;
mod ray;
mod utils;
mod camera;

use std::io;
use rand::prelude::*;
use ray::Ray;
use camera::Camera;
use utils::{Color, Point, Vec};
use hit::{Hittable, HittableVec, Sphere};


fn ray_color(r: &Ray, world: &HittableVec) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return (rec.normal + Vec::new(1.0, 1.0, 1.0)) / 2.0;
    }

    let dir = r.dir.unit();
    let t = 0.5 * (dir.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLE_COUNT: u32 = 32;

    println!("P3");
    println!("{}, {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let cam = Camera::new();

    let mut world = HittableVec::new();
    world.push(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let mut stdout = io::stdout();
    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for s in 0..SAMPLE_COUNT {
                let u = (i as f64 + rng.gen_range(0.0, 1.0)) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64 + rng.gen_range(0.0, 1.0)) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.cast_ray(u, v);
                color += ray_color(&r, &world);
            }

            utils::write_color(&mut stdout, color, SAMPLE_COUNT).unwrap();
        }
    }

    eprintln!("\nDone!");
}
