mod lambertian;
mod metal;

use super::Ray;
use super::hit::HitRecord;
use super::utils::Color;

pub use lambertian::*;
pub use metal::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}
