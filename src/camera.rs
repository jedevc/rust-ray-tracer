use crate::ray::Ray;
use crate::utils::{Point, Vec, vec};

pub struct Camera {
    pub origin: Point,
    horizontal: Vec,
    vertical: Vec,
    llcorner: Vec,
    u: Vec,
    v: Vec,
    w: Vec,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec, vfov: f64, ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let hheight = (vfov / 2.0).tan();
        let hwidth = ratio * hheight;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let lens_radius = aperture / 2.0;
        let llcorner = origin - u * hwidth * focus_dist - v * hheight * focus_dist - w * focus_dist;
        let horizontal = u * hwidth * focus_dist * 2.0;
        let vertical = v * hheight * focus_dist * 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            llcorner,
            u, v, w,
            lens_radius,
        }
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
        let rd = vec::random_disk_point(&mut rand::thread_rng()) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.llcorner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
