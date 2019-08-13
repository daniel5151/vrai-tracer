use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util::rand_in_unit_sphere;
use crate::vec3::Vec3;

use super::{Material, MaterialT};

/// Material that scatters incoming rays in random directions.
#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    /// Return a new Lambertian material
    pub fn new_material(albedo: Vec3) -> MaterialT {
        Lambertian { albedo }.into()
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let _ = r_in; // unused, since rays are reflected randomly

        let target = rec.p + rec.normal + rand_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
