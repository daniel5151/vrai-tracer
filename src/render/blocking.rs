use crate::scenes::Scene;

use super::{render_pixel, RenderOpts};

/// Synchronously render scene into buffer
pub fn trace_some_rays_blocking(buffer: &mut Vec<u32>, scene: &impl Scene, opts: RenderOpts) {
    buffer
        .chunks_mut(opts.width)
        .enumerate()
        .for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, px)| {
                let mut rng = rand::thread_rng();
                *px = render_pixel(&mut rng, x, y, &opts, scene);
            })
        });
}
