use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// General purpose 3D Vector class
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Create a new Vec3
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Returns vector's absolute length
    #[inline]
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    /// Returns vector's absolute length squared
    #[inline]
    pub fn squared_length(&self) -> f32 {
        // fun fact, the compiler is good enough to optimize these powf calls
        // into simple multiplications: https://godbolt.org/z/mUezdU
        self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)
    }

    /// Returns vector's corresponding unit vector
    #[inline]
    pub fn normalize(&self) -> Vec3 {
        let k = 1.0 / self.length();
        *self * k
    }

    /// Dot product operator
    #[inline]
    pub fn dot(&self, v2: &Vec3) -> f32 {
        let v1 = self;
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    /// Cross product operator
    #[inline]
    pub fn cross(&self, v2: &Vec3) -> Vec3 {
        let v1 = self;
        Vec3 {
            x: v1.y * v2.z - v1.z * v2.y,
            y: -(v1.x * v2.z - v1.z * v2.x),
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
}

macro_rules! impl_Op {
    ($name:ident, $function:ident, $operator:tt) => {
        impl $name for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $function(self, v2: Vec3) -> Vec3 {
                Vec3 {
                    x: self.x $operator v2.x,
                    y: self.y $operator v2.y,
                    z: self.z $operator v2.z,
                }
            }
        }
    };
}

impl_Op!(Add, add, +);
impl_Op!(Sub, sub, -);
impl_Op!(Mul, mul, *);
impl_Op!(Div, div, /);

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        self * -1.
    }
}

macro_rules! impl_OpAssign {
    ($name:ident, $function:ident, $operator:tt) => {
        impl $name for Vec3 {
            #[inline]
            fn $function(&mut self, v2: Vec3) {
                *self = *self * v2;
            }
        }
    };
}

impl_OpAssign!(AddAssign, add_assign, +);
impl_OpAssign!(SubAssign, sub_assign, -);
impl_OpAssign!(MulAssign, mul_assign, *);
impl_OpAssign!(DivAssign, div_assign, /);

macro_rules! impl_f32Op {
    ($name:ident, $function:ident, $operator:tt) => {
        impl $name<f32> for Vec3 {
            type Output = Vec3;

            #[inline]
            fn $function(self, f: f32) -> Vec3 {
                Vec3 {
                    x: self.x $operator f,
                    y: self.y $operator f,
                    z: self.z $operator f,
                }
            }
        }

        impl $name<Vec3> for f32 {
            type Output = Vec3;

            #[inline]
            fn $function(self, v: Vec3) -> Vec3 {
                Vec3 {
                    x: v.x $operator self,
                    y: v.y $operator self,
                    z: v.z $operator self,
                }
            }
        }
    };
}

impl_f32Op!(Mul, mul, *);
impl_f32Op!(Div, div, /);

macro_rules! impl_f32OpAssign {
    ($name:ident, $function:ident, $operator:tt) => {
        impl $name<f32> for Vec3 {
            #[inline]
            fn $function(&mut self, f: f32) {
                *self = *self * f;
            }
        }
    };
}

impl_f32OpAssign!(MulAssign, mul_assign, *);
impl_f32OpAssign!(DivAssign, div_assign, /);
