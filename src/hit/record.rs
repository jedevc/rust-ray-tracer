use super::{Vec, Point, Ray};
use crate::material::{Material};
use std::rc::Rc;

pub struct HitRecord {
    pub p: Point,
    pub t: f64,
    pub normal: Vec,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point, t: f64, r: &Ray, normal: Vec, material: Rc<dyn Material>) -> Self {
        let front_face = r.dir.dot(normal) < 0.0;
        Self {
            p,
            t,
            front_face,
            normal: if front_face { normal } else { -normal },
            material,
        }
    }
}
