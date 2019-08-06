//! Misc. utility functions that are used throughout the codebase

use rand::Rng;

use crate::vec3::Vec3;

/// Return a random point within the unit sphere
pub fn rand_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1., 1., 1.);
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

/// Return a random point within the unit sphere
pub fn rand_in_unit_circle() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.) - Vec3::new(1., 1., 0.);
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

const AVG_SIZE: usize = 1;

#[derive(Default)]
pub struct SmoothAvg {
    i: usize,
    e: [f32; AVG_SIZE],
}

impl SmoothAvg {
    pub fn new() -> SmoothAvg {
        SmoothAvg {
            i: 0,
            e: [0.; AVG_SIZE],
        }
    }

    pub fn update(&mut self, v: f32) {
        if !v.is_normal() {
            // INF, -INF, or zero
            return;
        }
        self.e[self.i] = v;
        self.i = (self.i + 1) % AVG_SIZE;
    }

    pub fn get(&self) -> f32 {
        self.e
            .iter()
            .filter(|x| **x > 0.00001)
            .fold(0., |a, x| a + x)
            / AVG_SIZE as f32
    }
}
