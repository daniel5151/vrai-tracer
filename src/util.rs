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
