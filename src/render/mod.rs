use rand::Rng;

mod blocking;
mod nonblocking;

pub use blocking::trace_some_rays_blocking;
pub use nonblocking::{trace_some_rays_nonblocking, RenderProgress};

use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::scenes::Scene;
use crate::vec3::Vec3;

/// Container for various render options
#[derive(Debug, Clone, Copy)]
pub struct RenderOpts {
    /// image width
    pub width: usize,
    /// image height
    pub height: usize,
    /// samples per-pixel
    pub samples: usize,
}

const MAX_DEPTH: usize = 50;

/// Core ray-tracing method.
fn color(r: &Ray, world: &impl Hittable, depth: usize) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.001..std::f32::MAX) {
        if depth >= MAX_DEPTH {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * color(&scattered, world, depth + 1);
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    // Background gradient
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

/// Core per-pixel render method.
/// Shared between various buffer drivers.
#[inline]
fn render_pixel(
    rng: &mut impl Rng,
    x: usize,
    y: usize,
    opts: &RenderOpts,
    scene: &impl Scene,
) -> u32 {
    let y = (opts.height - y) as f32;
    let x = x as f32;

    let avg_color = (0..opts.samples).fold(Vec3::new(0.0, 0.0, 0.0), |col, _| {
        let u = (x + rng.gen::<f32>()) / opts.width as f32;
        let v = (y + rng.gen::<f32>()) / opts.height as f32;

        let r = scene.get_camera().get_ray(u, v);

        col + color(&r, scene.get_world(), 0)
    }) / opts.samples as f32;

    let avg_color = Vec3::new(avg_color.x.sqrt(), avg_color.y.sqrt(), avg_color.z.sqrt());

    avg_color.as_color()
}

/// Extension trait for returning self as a u32 RGBA value.
trait AsColorExt {
    fn as_color(self) -> u32;
}

impl AsColorExt for Vec3 {
    fn as_color(mut self) -> u32 {
        self *= 255.99;
        u32::from_le_bytes([self.z as u8, self.y as u8, self.x as u8, 0])
    }
}
