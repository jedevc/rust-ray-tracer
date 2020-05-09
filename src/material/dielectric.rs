use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::utils::Color;

use super::Material;

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let etai_etat = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let color = Color::new(1.0, 1.0, 1.0);

        let direction = ray.dir.unit();
        let cos_theta = f64::min(1.0, hit.normal.dot(-direction));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if (etai_etat * sin_theta > 1.0) {
            // reflect
            let reflected = direction.reflect(hit.normal);
            Some((color, Ray::new(hit.p, reflected)))
        } else {
            // refract
            let refracted = direction.refract(hit.normal, etai_etat);
            Some((color, Ray::new(hit.p, refracted)))
        }
    }
}
