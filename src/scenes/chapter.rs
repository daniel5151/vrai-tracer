use crate::camera::{Camera, CameraOpts};
use crate::hittable::Sphere;
use crate::material;
use crate::vec3::Vec3;

use super::Scene;

/// The Scene that was gradually expanded upon throughout RTIOW.
pub struct Chapter {
    camera: Camera,
    spheres: Vec<Sphere>,
}

impl Chapter {
    /// Create a new Chapter scene
    // TODO?: add parameter to stage the scene as it appeared at chapter X?
    pub fn new() -> Chapter {
        let spheres = vec![
            Sphere::new(
                Vec3::new(0.0, 0.0, -2.0),
                0.25,
                Box::new(material::Lambertian::new(Vec3::new(1., 0., 0.))),
            ),
            Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Box::new(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
            ),
            Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Box::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
            ),
            Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Box::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.25)),
            ),
            Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Box::new(material::Dielectric::new(1.5)),
            ),
            Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                -0.45,
                Box::new(material::Dielectric::new(1.5)),
            ),
        ];

        let look_from = Vec3::new(3.0, 3., 2.);
        let look_at = Vec3::new(0., 0., -1.);

        Chapter {
            camera: Camera::new(CameraOpts {
                origin: look_from,
                direction: (look_from - look_at).normalize(),
                vup: Vec3::new(0., 1., 0.),
                hfov: 45.0,
                aspect: 9999., // dummy value, should depend on output medium
                aperture: 2.0,
                focus_dist: (look_from - look_at).length(),
            }),
            spheres,
        }
    }
}

impl Scene for Chapter {
    type World = Vec<Sphere>;

    fn get_world(&self) -> &Vec<Sphere> {
        &self.spheres
    }

    fn get_camera(&self) -> &Camera {
        &self.camera
    }
    fn enable_freecam(&mut self, camera: Camera) {
        self.camera = camera;
    }
    fn disable_freecam(&mut self) {}

    fn animate(&mut self, time: std::time::Duration) {
        self.spheres.get_mut(0).map(|s| {
            s.center.x = (time.as_millis() as f32 / 1000.).sin();
        });
    }
}
