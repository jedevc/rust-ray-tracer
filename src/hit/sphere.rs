use super::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::{Point, Vec};
use std::rc::Rc;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let hb = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = hb * hb - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let root = discriminant.sqrt();
        let solve = (-hb - root) / a;
        if solve > t_min && solve < t_max {
            let p = ray.at(solve);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord::new(
                p,
                solve,
                ray,
                normal,
                Rc::clone(&self.material),
            ));
        }

        let solve = (-hb + root) / a;
        if solve > t_min && solve < t_max {
            let p = ray.at(solve);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord::new(
                p,
                solve,
                ray,
                normal,
                Rc::clone(&self.material),
            ));
        }

        None
    }
}
