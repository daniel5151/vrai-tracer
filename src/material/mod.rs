use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: std::fmt::Debug {
    /// Given a incoming [Ray] and a [HitRecord], returns None if the Ray is
    /// absorbed, or Some((Attentuation, Scattered Ray))
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

/// Material that absorbs all incoming rays
#[derive(Debug, Clone)]
pub struct Void;

impl Material for Void {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }
}
