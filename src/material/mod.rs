use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod dielectric;
mod lambertian;
mod metal;
mod void;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use void::Void;

pub trait Material: Send + Sync + std::fmt::Debug {
    /// Given a incoming [Ray] and a [HitRecord], returns None if the Ray is
    /// absorbed, or Some((Attentuation, Scattered Ray))
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "enum_dispatch")] {
        pub type MaterialT = Materials;
    } else {
        pub type MaterialT = Box<dyn Material>;
    }
}

macro_rules! impl_ref {
    ($type:ty) => {
        impl Material for $type {
            fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
                (**self).scatter(r_in, rec)
            }
        }
    };
}

cfg_if::cfg_if! {
    if #[cfg(feature = "enum_dispatch")] {
        // When using enum dispatch:
        // - create the enum
        // - Implements Material for &enum and &mut enum
        // - Implements Into<enum> for each enum variant
        // - Implements enum dispatch by implementing Material on the enum
        macro_rules! materials {
            (
                $(#[$meta:meta])*
                $(pub)? enum $enum_name:ident {
                    $($mat_name:ident($mat_type:ty),)*
                }
            ) => {
                $(#[$meta])*
                pub enum $enum_name {
                    $($mat_name($mat_type),)*
                }

                impl_ref!(&$enum_name);
                impl_ref!(&mut $enum_name);

                $(
                    impl Into<$enum_name> for $mat_type {
                        fn into(self) -> $enum_name {
                            $enum_name::$mat_name(self)
                        }
                    }
                )*

                impl Material for $enum_name {
                    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
                        use self::$enum_name::*;
                        match self {
                            $($mat_name(x) => x.scatter(r_in, rec),)*
                        }
                    }
                }
            };
        }
    } else {
        // When using dynamic dispatch:
        // - Implement Material for various Box<dyn>, &dyn, and &mut dyn Material
        // - Implement Into<Box<dyn Material>> for each Material type
        macro_rules! materials {
            (
                $(#[$meta:meta])*
                $(pub)? enum $enum_name:ident {
                    $($mat_name:ident($mat_type:ty),)*
                }
            ) => {
                impl_ref!(Box<dyn Material>);
                impl_ref!(&dyn Material);
                impl_ref!(&mut dyn Material);

                $(
                    impl Into<Box<dyn Material>> for $mat_type {
                        fn into(self) -> Box<dyn Material> {
                            Box::new(self)
                        }
                    }
                )*
            };
        }
    }
}

materials! {
    #[derive(Debug)]
    pub enum Materials {
        Dielectric(Dielectric),
        Lambertian(Lambertian),
        Metal(Metal),
        Void(Void),
    }
}
