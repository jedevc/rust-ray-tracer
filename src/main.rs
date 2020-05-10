mod camera;
mod hit;
mod material;
mod ray;
mod utils;

use std::f64::consts::PI;
use camera::Camera;
use hit::{Hittable, HittableVec, Sphere};
use material::{Material, Lambertian, Metal, Dielectric};
use rand::prelude::*;
use ray::Ray;
use std::io;
use std::rc::Rc;
use utils::{random_lambertian_point, Color, Point, Vec};

fn ray_color(r: &Ray, world: &HittableVec, depth: u32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        return if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::zero()
        };
    }

    let dir = r.dir.unit();
    let t = 0.5 * (dir.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLE_COUNT: u32 = 150;
    const MAX_DEPTH: u32 = 50;

    println!("P3");
    println!("{}, {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let vup = Vec::new(0.0, 1.0, 0.0);
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let cam = Camera::new(
        lookfrom, lookat,
        vup, (20f64).to_radians(), ASPECT_RATIO,
        0.1, 10.0,
    );

    let world = cover_scene();

    let mut stdout = io::stdout();
    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}  ", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLE_COUNT {
                let u = (i as f64 + rng.gen_range(0.0, 1.0)) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64 + rng.gen_range(0.0, 1.0)) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.cast_ray(u, v);
                color += ray_color(&r, &world, MAX_DEPTH);
            }

            utils::write_color(&mut stdout, color, SAMPLE_COUNT).unwrap();
        }
    }

    eprintln!("\nDone!");
}

fn cover_scene() -> HittableVec {
    let mut world = HittableVec::new();

    // floor
    world.push(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    )));

    // scatters
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let mat_choice = rng.gen_range(0.0, 1.0);
            let center = Point::new(
                a as f64 + rng.gen_range(0.0, 0.9),
                0.2,
                b as f64 + rng.gen_range(0.0, 0.9));

            let offset = center - Vec::new(4.0, 0.2, 0.0);
            if offset.length() < 0.9 {
                continue;
            }

            let material: Rc<dyn Material>;
            if mat_choice < 0.8 {
                // diffuse
                let min = Color::new(0.0, 0.0, 0.0);
                let max = Color::new(1.0, 1.0, 1.0);
                let albedo = rng.gen_range(min, max) * rng.gen_range(min, max);
                material = Rc::new(Lambertian::new(albedo));
            } else if mat_choice < 0.95 {
                // metal
                let min = Color::new(0.5, 0.5, 0.5);
                let max = Color::new(1.0, 1.0, 1.0);
                let albedo = rng.gen_range(min, max);
                let fuzz = rng.gen_range(0.0, 0.5);
                material = Rc::new(Metal::new(albedo, fuzz));
            } else {
                // glass
                material = Rc::new(Dielectric::new(1.5));
            }

            let obj = Box::new(Sphere::new(center, 0.2, material));
            world.push(obj);
        }
    }

    // centerpieces
    world.push(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}