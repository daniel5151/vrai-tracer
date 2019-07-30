use std::ops::Range;

use rand::Rng;

use crate::camera::Camera;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Container for hit information
#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    /// Position along ray
    pub t: f32,
    /// Hit point
    pub p: Vec3,
    /// Hit Normal
    pub normal: Vec3,
}

/// Anything that can be Hit by a ray
pub trait Hittable: std::fmt::Debug {
    /// Check if object is hit by [[Ray]] `r`.
    /// Returns None if no hit occurred, or Some(HitRecord) otherwise.
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord>;
}

/// A Sphere. You know what a Sphere is, right?
#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    /// Create a new sphere with a specified `center` and `radius`
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = 2.0 * Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius.powf(2.);
        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant > 0.0 {
            macro_rules! check_root {
                ($sign:tt) => {
                    let root = (-b $sign discriminant.sqrt()) / (2.0 * a);
                    if t_range.contains(&root) {
                        let t = root;
                        let p = r.point_at_param(t);
                        let normal = (p - self.center) / self.radius;
                        return Some(HitRecord{ t, p, normal });
                    }
                };
            }
            check_root!(-);
            check_root!(+);
        }

        None
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    /// Returns the HitRecord of the closest hittable object
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_range.end;

        for hittable in self {
            if let Some(rec) = hittable.hit(r, t_range.start..closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}

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

use std::time::Duration;

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
