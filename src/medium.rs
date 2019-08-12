use crate::hitable::{Hitable, HitRecord};
use crate::vec3::Vec3;
use crate::texture::Texture;
use crate::material::{Material, ScatterRecord, Isotropic};

pub struct ConstantMedium {
    pub boundary: Box<dyn Hitable>,
    pub density: f32,
    pub phase_function: Box<dyn Material>
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hitable>, density: f32, texture: Box<dyn Texture>) -> Self {
        ConstantMedium {
            boundary,
            density,
            phase_function: Box::new(Isotropic::new(texture))
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(&r, -std::f32::MAX, -std::f32::MAX) {
            if let Some(mut rec2) = self.boundary.hit(&r, rec1.t+0.0001, std::f32::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    None
                }
                if rec1.t < 0 {
                    rec1.t = 0;
                }
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(self.boundary.bounding_box(t0, t1))
    }
}
