use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util::rand_in_unit_sphere;
use crate::vec3::Vec3;

use super::{Material, MaterialT};

/// Material that reflects incoming rays through the hit-point's normal
#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    /// Return a new Metal material
    pub fn new_material(albedo: Vec3, fuzz: f32) -> MaterialT {
        Metal { albedo, fuzz }.into()
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r_in.direction.normalize().reflect_through(&rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * rand_in_unit_sphere());
        let attenuation = self.albedo;
        // TODO: do some personal reasearch into why this check is used
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
