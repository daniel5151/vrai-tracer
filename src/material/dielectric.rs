use rand::Rng;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

use super::Material;

/// Material that scatters incoming rays in random directions.
#[derive(Debug)]
pub struct Dielectric {
    /// Refractive index
    ref_idx: f32,
}

impl Dielectric {
    /// Return a new Dielectric material, given it's Refractive Index
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

/// Returns a vector's refraction through a surface with a normal `n` and
/// refractive index of `ni_over_nt`
fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    // TODO: derive this on paper
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt.powf(2.) * (1. - dt.powf(2.));
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * f32::sqrt(discriminant))
    } else {
        None
    }
}

/// Polynomial approximation of reflectivity that varies by angle
/// by Christophe Schlick
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r_in.direction.reflect_through(&rec.normal);

        let outward_normal;
        let ni_over_nt;
        let cosine;
        if r_in.direction.dot(&rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(&rec.normal) / r_in.direction.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction.dot(&rec.normal) / r_in.direction.length();
        }

        let scattered = match refract(&r_in.direction, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_idx);
                if rand::thread_rng().gen::<f32>() < reflect_prob {
                    Ray::new(rec.p, reflected)
                } else {
                    Ray::new(rec.p, refracted)
                }
            }
            None => Ray::new(rec.p, reflected),
        };

        let attenuation = Vec3::new(1., 1., 1.); // doesn't absorb anything
        return Some((attenuation, scattered));
    }
}
