use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::utils::{vec, Color};

use super::Material;

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_dir = hit.normal + vec::random_lambertian_point(&mut rand::thread_rng());
        let scattered = Ray::new(hit.p, scatter_dir);
        Some((self.albedo, scattered))
    }
}
