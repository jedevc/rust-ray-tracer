use super::{Hittable, HitRecord, Vec, Point, Ray};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec, radius: f64) -> Self {
        Sphere {
            center,
            radius
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
            return None
        }

        let root = discriminant.sqrt();
        let solve = (-hb - root) / a;
        if solve > t_min && solve < t_max {
            let p = ray.at(solve);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord::new(p, solve, ray, normal));
        }

        let solve = (-hb + root) / a;
        if solve > t_min && solve < t_max {
            let p = ray.at(solve);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord::new(p, solve, ray, normal));
        }

        None
    }
}