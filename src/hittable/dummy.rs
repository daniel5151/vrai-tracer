use crate::ray::Ray;
use std::ops::Range;

use super::{HitRecord, Hittable};

/// Dummy hittable that's never actually hit.
/// Primarily created to benchmark various static / dynamic dispatch scenarios
/// until such a time that I implement a hittable that isn't a sphere.
pub struct Dummy;

impl Hittable for Dummy {
    fn hit(&self, _: &Ray, _: Range<f32>) -> Option<HitRecord> {
        None
    }
}
