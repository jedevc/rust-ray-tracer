use crate::utils::{Point, Vec};

pub struct Ray {
    pub origin: Point,
    pub dir: Vec,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir * t
    }
}
