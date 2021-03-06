mod record;
mod sphere;

use crate::ray::Ray;
use crate::utils::{Point, Vec};
use std::vec;

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
