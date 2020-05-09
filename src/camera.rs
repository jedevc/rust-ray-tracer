use super::utils::{Point, Vec};
use super::ray::Ray;

pub struct Camera {
    pub origin: Point,
    pub horizontal: Vec,
    pub vertical: Vec,
    pub llcorner: Vec,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec::new(4.0, 0.0, 0.0);
        let vertical = Vec::new(0.0, 2.25, 0.0);
        let llcorner = origin - horizontal / 2.0 - vertical / 2.0 - Vec::new(0.0, 0.0, 1.0);

        Camera {
            origin, horizontal, vertical, llcorner
        } 
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.llcorner + self.horizontal * u + self.vertical * v)
    }
}