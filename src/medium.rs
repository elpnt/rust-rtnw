use crate::aabb::AABB;
use crate::hitable::*;
use crate::material::Isotropic;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

use rand::prelude::*;

pub struct ConstantMedium<H: Hitable, T: Texture> {
    pub boundary: H,
    pub density: f32,
    pub phase_function: Isotropic<T>,
}

impl<H: Hitable, T: Texture> ConstantMedium<H, T> {
    pub fn new(boundary: H, density: f32, texture: T) -> Self {
        ConstantMedium {
            boundary,
            density,
            phase_function: Isotropic::new(texture),
        }
    }
}

impl<H: Hitable, T: Texture> Hitable for ConstantMedium<H, T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(&r, -std::f32::MAX, std::f32::MAX) {
            if let Some(mut rec2) = self.boundary.hit(&r, rec1.t + 0.0001, std::f32::MAX) {
                if rec1.t < t_min {
                    rec1.t = t_min
                };
                if rec2.t > t_max {
                    rec2.t = t_max
                };
                if rec1.t < rec2.t {
                    if rec1.t < 0.0 {
                        rec1.t = 0.0
                    };
                    let mut rng = thread_rng();
                    let distance_inside_boundary: f32 = (rec2.t - rec1.t) * r.direction.length();
                    let hit_distance: f32 = -(1.0 / self.density) / (rng.gen::<f32>()).ln();
                    if hit_distance < distance_inside_boundary {
                        let t = rec1.t + hit_distance / r.direction.length();
                        return Some(HitRecord {
                            t,
                            u: 0.0,
                            v: 0.0,
                            p: r.point_at_parameter(t),
                            normal: Vec3::new(1.0, 0.0, 0.0),
                            material: &self.phase_function,
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
