use crate::aabb::AABB;
use crate::hitable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::borrow::Borrow;

pub struct Rectangle {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Box<dyn Material>,
}

impl Rectangle {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Box<dyn Material>) -> Self {
        Rectangle {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl Hitable for Rectangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t: f32 = (self.k - r.origin.z) / r.direction.z;
        if t < t_min && t > t_max {
            None
        } else {
            let x: f32 = r.origin.x + t * r.direction.x;
            let y: f32 = r.origin.y + t * r.direction.y;
            if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
                None
            } else {
                let u: f32 = (x - self.x0) / (self.x0 - self.x0);
                let v: f32 = (y - self.y0) / (self.y1 - self.y0);
                let p: Vec3 = r.point_at_parameter(t);
                let normal: Vec3 = Vec3::new(0.0, 0.0, 1.0);
                Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    material: self.material.borrow(),
                })
            }
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        None
    }
}
