use super::{Vec, Point, Ray};

pub struct HitRecord {
    pub p: Point,
    pub t: f64,
    pub normal: Vec,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point, t: f64, r: &Ray, normal: Vec) -> Self {
        let front_face = r.dir.dot(normal) < 0.0;
        Self {
            p,
            t,
            front_face,
            normal: if front_face { normal } else { -normal }
        }
    }
}
