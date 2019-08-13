use std::ops::Range;

use crate::material::MaterialT;
use crate::ray::Ray;
use crate::vec3::Vec3;

use super::{HitRecord, Hittable, HittableT};

/// A Sphere. You know what a Sphere is, right?
#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: MaterialT,
}

impl Sphere {
    /// Create a new sphere with a specified `center` and `radius`
    pub fn new_hittable(center: Vec3, radius: f32, material: MaterialT) -> HittableT {
        Sphere {
            center,
            radius,
            material,
        }
        .into()
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = 2.0 * Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius.powf(2.);
        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant > 0.0 {
            macro_rules! check_root {
                ($sign:tt) => {
                    let root = (-b $sign discriminant.sqrt()) / (2.0 * a);
                    if t_range.contains(&root) {
                        let t = root;
                        let p = r.point_at_param(t);
                        let normal = (p - self.center) / self.radius;
                        return Some(HitRecord{ t, p, normal, material: &self.material });
                    }
                };
            }
            check_root!(-);
            check_root!(+);
        }

        None
    }
}
