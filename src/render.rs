use std::ops::Range;

use crate::ray::Ray;
use crate::vec3::Vec3;

trait AsColor {
    fn as_color(self) -> u32;
}

impl AsColor for Vec3 {
    fn as_color(mut self) -> u32 {
        self = self * 255.99;
        u32::from_le_bytes([self.z as u8, self.y as u8, self.x as u8, 0])
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    /// Position along ray
    pub t: f32,
    /// Hit point
    pub p: Vec3,
    /// Hit Normal
    pub normal: Vec3,
}

trait Hitable: std::fmt::Debug {
    fn hit(&self, r: &Ray, t_range: Range<f32>, rec: &mut HitRecord) -> bool;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_range: Range<f32>, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = 2.0 * Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius.powf(2.);
        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant > 0.0 {
            macro_rules! check_root {
                ($sign:tt) => {
                    let root = (-b $sign discriminant.sqrt()) / (2.0 * a);
                    if t_range.contains(&root) {
                        rec.t = root;
                        rec.p = r.point_at_param(rec.t);
                        rec.normal = (rec.p - self.center) / self.radius;
                        return true;
                    }
                };
            }
            check_root!(-);
            check_root!(+);
        }

        false
    }
}

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, r: &Ray, t_range: Range<f32>, rec: &mut HitRecord) -> bool {
        // TODO: this be ugly
        let mut temp_rec = HitRecord {
            t: 0.0,
            p: Vec3::new(0., 0., 0.),
            normal: Vec3::new(1., 0., 0.),
        };
        let mut hit_anything = false;

        let mut closest_so_far = t_range.end;
        for hitable in self {
            if hitable.hit(r, t_range.start..closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}

fn color(r: &Ray, world: &Vec<Box<dyn Hitable>>) -> Vec3 {
    // TODO: this be ugly
    let mut rec = HitRecord {
        t: 0.0,
        p: Vec3::new(0., 0., 0.),
        normal: Vec3::new(1., 0., 0.),
    };

    if world.hit(r, 0.0..std::f32::MAX, &mut rec) {
        let n = rec.normal;
        return 0.5 * Vec3::new(n.x + 1., n.y + 1., n.z + 1.);
    }

    // Background gradient
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

pub fn trace_some_rays(buffer: &mut Vec<u32>, width: usize, height: usize) {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    for (y, row) in buffer.chunks_exact_mut(width).enumerate() {
        for (x, px) in row.iter_mut().enumerate() {
            let y = (height - y) as f32;
            let x = x as f32;

            let u = x / width as f32;
            let v = y / height as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            *px = color(&r, &world).as_color();
        }
    }
}
