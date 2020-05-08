mod ray;
mod utils;

use std::io;
use utils::{Vec, Color, Point};
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Vec::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec::new(0.0, 0.0, -1.0)).unit();
        return (n + Vec::new(1.0, 1.0, 1.0)) / 2.0;
    }

    let dir = r.dir.unit();
    let t = 0.5 * (dir.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: Vec, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.dir.length_squared();
    let hb = oc.dot(ray.dir);
    let c = oc.length_squared() - radius * radius;

    let discriminant = hb * hb - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-hb - discriminant.sqrt()) / a
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    println!("P3");
    println!("{}, {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec::new(4.0, 0.0, 0.0);
    let vertical = Vec::new(0.0, 2.25, 0.0);
    let llcorner = origin - horizontal / 2.0 - vertical / 2.0 - Vec::new(0.0, 0.0, 1.0);

    let mut stdout = io::stdout();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin, llcorner + horizontal * u + vertical * v);

            utils::write_color(&mut stdout, ray_color(&r)).unwrap();
        }
    }

    eprintln!("\nDone!");
}
