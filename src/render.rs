use std::time::Duration;

use rand::Rng;

use crate::camera::Camera;
use crate::hittable::{sphere::Sphere, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

fn rand_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1., 1., 1.);
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

fn color(r: &Ray, world: &Vec<Box<dyn Hittable>>) -> Vec3 {
    // Color is based off returned Normal
    if let Some(rec) = world.hit(r, 0.001..std::f32::MAX) {
        let target = rec.p + rec.normal + rand_in_unit_sphere();
        return 0.5 * color(&Ray::new(rec.p, target - rec.p), world);
    }

    // Background gradient
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

trait AsColor {
    fn as_color(self) -> u32;
}

impl AsColor for Vec3 {
    fn as_color(mut self) -> u32 {
        self = self * 255.99;
        u32::from_le_bytes([self.z as u8, self.y as u8, self.x as u8, 0])
    }
}

/// Container for various render options
pub struct RenderOpts {
    /// image width
    pub width: usize,
    /// image height
    pub height: usize,
    /// samples per-pixel
    pub samples: usize,
}

pub fn trace_some_rays(buffer: &mut Vec<u32>, opts: RenderOpts, time: Duration) {
    let mut rng = rand::thread_rng();

    let camera = Camera::new();
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    for (y, row) in buffer.chunks_exact_mut(opts.width).enumerate() {
        for (x, px) in row.iter_mut().enumerate() {
            let y = (opts.height - y) as f32;
            let x = x as f32;

            let camera_offset = (time.as_millis() as f32 / 1000.).sin() / 3.;

            let avg_color = (0..opts.samples).fold(Vec3::new(0.0, 0.0, 0.0), |col, _| {
                let u = (x + rng.gen::<f32>()) / opts.width as f32;
                let v = (y + rng.gen::<f32>()) / opts.height as f32;

                let r = camera.get_ray(u + camera_offset, v);

                col + color(&r, &world)
            }) / opts.samples as f32;

            let avg_color = Vec3::new(avg_color.x.sqrt(), avg_color.y.sqrt(), avg_color.z.sqrt());

            *px = avg_color.as_color();
        }
    }
}
