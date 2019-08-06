use crate::camera::Camera;
use crate::hittable::Hittable;

mod chapter;
mod random;

pub use chapter::Chapter;
pub use random::Random;

pub trait Scene: Send + Sync {
    type World: Hittable;

    fn get_camera(&self) -> &Camera;
    fn get_world(&self) -> &Self::World;

    /// Enables freecam, with specified camera position.
    fn enable_freecam(&mut self, cam: Camera);
    fn disable_freecam(&mut self);

    fn animate(&mut self, time: std::time::Duration) {
        let _ = time;
    }
}
