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
