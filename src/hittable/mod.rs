use std::ops::Range;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub mod sphere;

pub use sphere::Sphere;

/// Container for hit information
#[derive(Debug, Clone)]
pub struct HitRecord<'m> {
    /// Position along ray
    pub t: f32,
    /// Hit point
    pub p: Vec3,
    /// Hit Normal
    pub normal: Vec3,
    /// Material
    pub material: &'m dyn Material,
}

/// Anything that can be Hit by a ray
pub trait Hittable {
    /// Check if object is hit by [Ray] `r`.
    /// Returns None if no hit occurred, or Some(HitRecord) otherwise.
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

impl Hittable for &[&dyn Hittable] {
    /// Returns the HitRecord of the closest hittable object
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_range.end;

        for hittable in self.iter() {
            if let Some(rec) = hittable.hit(r, t_range.start..closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
