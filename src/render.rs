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

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b.powf(2.) - 4. * a * c;

    discriminant > 0.0
}

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::new_unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

pub fn trace_some_rays(buffer: &mut Vec<u32>, width: usize, height: usize) {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for (y, row) in buffer.chunks_exact_mut(width).enumerate() {
        for (x, px) in row.iter_mut().enumerate() {
            let y = (height - y) as f32;
            let x = x as f32;

            let u = x / width as f32;
            let v = y / height as f32;

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            *px = color(&r).as_color();
        }
    }
}
