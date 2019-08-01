use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    /// Returns a new Camera
    ///
    /// `hfov` is in degrees
    /// `aspect` is width:height ratio
    pub fn new(origin: Vec3, direction: Vec3, vup: Vec3, hfov: f32, aspect: f32) -> Camera {
        let theta = hfov * std::f32::consts::PI / 180.;
        let half_width = f32::tan(theta / 2.);
        let half_height = half_width / aspect;

        let w = direction.normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: origin - half_width * u - half_height * v - w,
            horizontal: 2. * half_width * u,
            vertical: 2. * half_height * v,
            origin,
        }
    }

    /// Return a ray corresponsing to a particular point along the camera's
    /// conceptual "window" into the world.
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
