//! Random scene from Chapter 12 of RTIOW
use rand::{thread_rng, Rng};

use crate::camera::{Camera, CameraOpts};
use crate::hittable::{HittableT, InfPlane, Sphere};
use crate::material;
use crate::vec3::Vec3;

use super::Scene;

/// The Scene that was gradually expanded upon throughout RTIOW.
pub struct Random {
    camera: Camera,
    scene: Vec<HittableT>,
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Random {
    /// Create a new Random scene
    // TODO?: add parameter to stage the scene as it appeared at chapter X?
    pub fn new() -> Random {
        let mut rng = thread_rng();

        let mut scene = Vec::new();
        // ground
        // scene.push(Sphere::new_hittable(
        //     Vec3::new(0., -1000., 0.),
        //     1000.,
        //     material::Lambertian::new_material(Vec3::new(0.5, 0.5, 0.5)),
        // ));

        scene.push(InfPlane::new_hittable(
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            material::Lambertian::new_material(Vec3::new(0.5, 0.5, 0.5)),
        ));

        // random spheres
        for a in -11..11 {
            for b in -11..11 {
                let a = a as f32;
                let b = b as f32;

                let material = match rng.gen::<f32>() {
                    r if r < 0.8 => {
                        (material::Lambertian::new_material(Vec3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )))
                    }
                    r if r < 0.95 => {
                        (material::Metal::new_material(
                            0.5 * Vec3::new(
                                rng.gen::<f32>() + 1.,
                                rng.gen::<f32>() + 1.,
                                rng.gen::<f32>() + 1.,
                            ),
                            0.5 * rng.gen::<f32>(),
                        ))
                    }
                    _ => (material::Dielectric::new_material(1.5)),
                };

                let center = Vec3::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());

                scene.push(Sphere::new_hittable(center, 0.2, material))
            }
        }

        scene.push(Sphere::new_hittable(
            Vec3::new(0., 1., 0.),
            1.,
            material::Dielectric::new_material(1.5),
        ));
        scene.push(Sphere::new_hittable(
            Vec3::new(-4., 1., 0.),
            1.,
            material::Lambertian::new_material(Vec3::new(0.4, 0.2, 0.1)),
        ));
        scene.push(Sphere::new_hittable(
            Vec3::new(4., 1., 0.),
            1.,
            material::Metal::new_material(Vec3::new(0.7, 0.6, 0.5), 0.0),
        ));

        let look_from = Vec3::new(13.0, 1.5, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);

        Random {
            camera: Camera::new(CameraOpts {
                origin: look_from,
                direction: (look_from - look_at).normalize(),
                vup: Vec3::new(0., 1., 0.),
                hfov: 40.0,
                aspect: 9999., // dummy value, should depend on output medium
                aperture: 0.25,
                focus_dist: 10.,
            }),
            scene,
        }
    }
}

impl Scene for Random {
    type World = Vec<HittableT>;

    fn get_camera(&self) -> &Camera {
        &self.camera
    }
    fn enable_freecam(&mut self, camera: Camera) {
        self.camera = camera;
    }
    fn disable_freecam(&mut self) {}

    fn get_world(&self) -> &Vec<HittableT> {
        &self.scene
    }
}
