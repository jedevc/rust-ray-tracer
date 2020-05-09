mod lambertian;
mod metal;
mod dielectric;

use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::utils::Color;

pub use lambertian::*;
pub use metal::*;
pub use dielectric::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}