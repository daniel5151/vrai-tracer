use crate::ray::Ray;
use crate::util::rand_in_unit_circle;
use crate::vec3::Vec3;

/// Camera Construction parameters
#[derive(Debug, Copy, Clone)]
pub struct CameraOpts {
    pub origin: Vec3,
    pub direction: Vec3,
    pub vup: Vec3,
    /// in degrees (TODO: newtype?)
    pub hfov: f32,
    /// width:height ratio
    pub aspect: f32,
    pub aperture: f32,
    pub focus_dist: f32,
}

#[derive(Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    #[allow(dead_code)]
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    /// Returns a new Camera
    pub fn new(opts: CameraOpts) -> Camera {
        let CameraOpts {
            origin,
            direction,
            vup,
            hfov,
            aspect,
            aperture,
            focus_dist,
        } = opts;

        let theta = hfov * std::f32::consts::PI / 180.;
        let half_width = f32::tan(theta / 2.);
        let half_height = half_width / aspect;

        let w = direction.normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: origin - focus_dist * (half_width * u + half_height * v + w),
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            origin,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
        }
    }

    /// Return a ray corresponsing to a particular point along the camera's
    /// conceptual "window" into the world.
    pub fn get_ray(&self, du: f32, dv: f32) -> Ray {
        let rd = self.lens_radius * rand_in_unit_circle();
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        Ray::new(
            origin,
            self.lower_left_corner + du * self.horizontal + dv * self.vertical - origin,
        )
    }
}
