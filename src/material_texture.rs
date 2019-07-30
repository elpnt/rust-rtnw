use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::texture::*;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Arc<Texture>,
}

impl Lambertian {
    pub fn new(x: f32, y: f32, z: f32) -> Texture {
        Lambertian {
            albedo: Arc::new(Vec3::new(x, y, z)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let target: Vec3 = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray {
            origin: rec.p,
            direction: target - rec.p,
        };
        let attenuation: Vec3 = self.albedo.value(0.0, 0.0, rec.p);

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
