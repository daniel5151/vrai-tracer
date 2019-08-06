use rand::Rng;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::vec3::Vec3;

use super::{color, AsColorExt, RenderOpts};

/// Synchronously render scene into buffer
pub fn trace_some_rays_blocking(
    buffer: &mut Vec<u32>,
    world: &impl Hittable,
    camera: Camera,
    opts: RenderOpts,
) {
    buffer
        .chunks_mut(opts.width)
        .enumerate()
        .for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, px)| {
                let mut rng = rand::thread_rng();

                let y = (opts.height - y) as f32;
                let x = x as f32;

                let avg_color = (0..opts.samples).fold(Vec3::new(0.0, 0.0, 0.0), |col, _| {
                    let u = (x + rng.gen::<f32>()) / opts.width as f32;
                    let v = (y + rng.gen::<f32>()) / opts.height as f32;

                    let r = camera.get_ray(u, v);

                    col + color(&r, world, 0)
                }) / opts.samples as f32;

                let avg_color =
                    Vec3::new(avg_color.x.sqrt(), avg_color.y.sqrt(), avg_color.z.sqrt());

                *px = avg_color.as_color();
            })
        });
}
