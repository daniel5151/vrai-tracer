use crate::camera::CameraOpts;
use crate::hittable::Hittable;

mod chapter;
mod random;

pub use chapter::Chapter;
pub use random::Random;

pub trait Scene<T: Hittable> {
    fn init_camopts(&self) -> CameraOpts;
    fn get_world(&self) -> &Vec<T>;
    fn animate(&mut self, time: std::time::Duration) {
        let _ = time;
    }
}
