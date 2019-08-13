use std::ops::Range;

use crate::material::MaterialT;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod infplane;
mod sphere;

pub use infplane::InfPlane;
pub use sphere::Sphere;

cfg_if::cfg_if! {
    if #[cfg(feature = "enum_dispatch")] {
        pub type HittableT = Hittables;
    } else {
        pub type HittableT = Box<dyn Hittable>;
    }
}

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
    pub material: &'m MaterialT,
}

/// Anything that can be Hit by a ray
pub trait Hittable: Send + Sync {
    /// Check if object is hit by [Ray] `r`.
    /// Returns None if no hit occurred, or Some(HitRecord) otherwise.
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

macro_rules! impl_ref {
    ($type:ty) => {
        impl Hittable for $type {
            fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
                (**self).hit(r, t_range)
            }
        }
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature = "enum_dispatch")] {
        // When using enum dispatch:
        // - create the enum
        // - Implements Hittable for &enum and &mut enum
        // - Implements Into<enum> for each enum variant
        // - Implements enum dispatch by implementing Hittable on the enum
        macro_rules! hittables {
            (
                $(#[$meta:meta])*
                $(pub)? enum $enum_name:ident {
                    $($hit_name:ident($hit_type:ty),)*
                }
            ) => {
                $(#[$meta])*
                pub enum $enum_name {
                    $($hit_name($hit_type),)*
                }

                impl_ref!(&$enum_name);
                impl_ref!(&mut $enum_name);

                $(
                    impl Into<$enum_name> for $hit_type {
                        fn into(self) -> $enum_name {
                            $enum_name::$hit_name(self)
                        }
                    }
                )*

                impl Hittable for $enum_name {
                    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
                        use self::$enum_name::*;
                        match self {
                            $($hit_name(x) => x.hit(r, t_range),)*
                        }
                    }
                }
            };
        }
    } else {
        // When using dynamic dispatch:
        // - Implement Hittable for various Box<dyn>, &dyn, and &mut dyn Hittable
        // - Implement Into<Box<dyn Hittable>> for each Hittable type
        macro_rules! hittables {
            (
                $(#[$meta:meta])*
                $(pub)? enum $enum_name:ident {
                    $($hit_name:ident($hit_type:ty),)*
                }
            ) => {
                impl_ref!(Box<dyn Hittable>);
                impl_ref!(&dyn Hittable);
                impl_ref!(&mut dyn Hittable);

                $(
                    impl Into<Box<dyn Hittable>> for $hit_type {
                        fn into(self) -> Box<dyn Hittable> {
                            Box::new(self)
                        }
                    }
                )*
            };
        }
    }
}

hittables! {
    #[derive(Debug)]
    pub enum Hittables {
        Sphere(Sphere),
        InfPlane(InfPlane),
    }
}

// Instead of creating a new Hittable container, implement Hittable as an
// extension trait right on some common Rust contianers :D

impl<H: Hittable> Hittable for &[H] {
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

impl<H: Hittable> Hittable for Vec<H> {
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
