use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

use super::{Material, MaterialT};

/// Material that absorbs all incoming rays
#[derive(Debug, Clone)]
pub struct Void;

impl Void {
    /// Return a new Void material
    pub fn new_material() -> MaterialT {
        Void.into()
    }
}

impl Material for Void {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }
}
