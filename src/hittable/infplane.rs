use std::ops::Range;

use crate::material::MaterialT;
use crate::ray::Ray;
use crate::vec3::Vec3;

use super::{HitRecord, Hittable, HittableT};

/// A infinitely flat plane.
#[derive(Debug)]
pub struct InfPlane {
    pub center: Vec3,
    pub normal: Vec3,
    pub material: MaterialT,
}

impl InfPlane {
    /// Create a new infinite plane with a specified `center` and `normal`
    pub fn new_hittable(center: Vec3, normal: Vec3, material: MaterialT) -> HittableT {
        InfPlane {
            center,
            normal,
            material,
        }
        .into()
    }
}

impl Hittable for InfPlane {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        // lightly modified from
        // https://samsymons.com/blog/math-notes-ray-plane-intersection/
        let denominator = self.normal.dot(&r.direction);
        let t = (self.center - r.origin).dot(&self.normal) / denominator;
        if t_range.contains(&t) {
            return Some(HitRecord {
                t,
                p: r.point_at_param(t),
                normal: self.normal,
                material: &self.material,
            });
        }

        None
    }
}
