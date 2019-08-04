use crate::aabb::AABB;
use crate::hitable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::borrow::Borrow;

pub enum Plane {
    XY,
    YZ,
    ZX,
}

pub struct Rectangle {
    pub plane: Plane,
    pub a0: f32,
    pub a1: f32,
    pub b0: f32,
    pub b1: f32,
    pub k: f32,
    pub material: Box<dyn Material>,
}

impl Rectangle {
    pub fn new(
        plane: Plane,
        a0: f32,
        a1: f32,
        b0: f32,
        b1: f32,
        k: f32,
        material: Box<dyn Material>,
    ) -> Self {
        Rectangle {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        }
    }
}

impl Hitable for Rectangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (a_axis, b_axis, k_axis) = match &self.plane {
            Plane::XY => (0, 1, 2),
            Plane::YZ => (1, 2, 0),
            Plane::ZX => (2, 0, 1),
        };
        let t: f32 = (self.k - r.origin[k_axis]) / r.direction[k_axis];
        if t < t_min || t > t_max {
            None
        } else {
            let a: f32 = r.origin[a_axis] + t * r.direction[a_axis];
            let b: f32 = r.origin[b_axis] + t * r.direction[b_axis];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let u: f32 = (a - self.a0) / (self.a1 - self.a0);
                let v: f32 = (b - self.b0) / (self.b1 - self.b0);
                let p: Vec3 = r.point_at_parameter(t);
                let normal = match k_axis {
                    0 => Vec3::new(1.0, 0.0, 0.0),
                    1 => Vec3::new(0.0, 1.0, 0.0),
                    2 => Vec3::new(0.0, 0.0, 1.0),
                    _ => panic!(),
                };
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

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let bbox = AABB::new(
            Vec3::new(self.a0, self.a0, self.k - 0.0001),
            Vec3::new(self.b1, self.b1, self.k + 0.0001),
        );
        Some(bbox)
    }
}
