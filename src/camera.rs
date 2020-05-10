use crate::ray::Ray;
use crate::utils::{Point, Vec};

pub struct Camera {
    pub origin: Point,
    pub horizontal: Vec,
    pub vertical: Vec,
    pub llcorner: Vec,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec, vfov: f64, ratio: f64) -> Self {
        let hheight = (vfov / 2.0).tan();
        let hwidth = ratio * hheight;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = lookfrom;
        let llcorner = origin - u * hwidth - v * hheight - w;
        let horizontal = u * hwidth * 2.0;
        let vertical = v * hheight * 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            llcorner,
        }
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.llcorner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
