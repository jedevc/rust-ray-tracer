use std::vec;
use super::ray::Ray;
use super::utils::{Point, Vec};

mod record;
mod sphere;

pub use record::*;
pub use sphere::*;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HittableVec = vec::Vec<Box<dyn Hittable>>;

impl Hittable for HittableVec {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record = None;
        let mut closest = t_max;

        for hittable in self {
            if let Some(rec) = hittable.hit(ray, t_min, closest) {
                closest = rec.t;
                record = Some(rec);
            }
        }

        record
    }
}
