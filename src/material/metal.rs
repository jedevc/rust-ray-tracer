use crate::utils::{vec, Color};
use crate::ray::Ray;
use crate::hit::HitRecord;

use super::Material;

pub struct Metal {
    pub albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.dir.unit().reflect(hit.normal);
        let scattered = Ray::new(hit.p, reflected);

        if scattered.dir.dot(hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}