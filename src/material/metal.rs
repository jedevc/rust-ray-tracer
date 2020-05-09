use crate::utils::{vec, Color};
use crate::ray::Ray;
use crate::hit::HitRecord;

use super::Material;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.dir.unit().reflect(hit.normal);

        let offset = vec::random_sphere_point(&mut rand::thread_rng()) * self.fuzz;
        let scattered = Ray::new(hit.p, reflected + offset);

        if scattered.dir.dot(hit.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}