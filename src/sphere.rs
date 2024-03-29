use crate::aabb::*;
use crate::hitable::{HitRecord, Hitable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::borrow::Borrow;
use std::f32::consts::PI;

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f32, material: M) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hitable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a: f32 = r.direction.dot(&r.direction);
        let b: f32 = oc.dot(&r.direction);
        let c: f32 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;

        if discriminant > 0. {
            // ray crosses the sphere at least once
            let temp1: f32 = (-b - discriminant.sqrt()) / a;
            let temp2: f32 = (-b + discriminant.sqrt()) / a;
            let b1: bool = t_max > temp1 && temp1 > t_min;
            let b2: bool = t_max > temp2 && temp2 > t_min;

            if discriminant > 0. && (b1 || b2) {
                // temp1の方がスクリーンに近い
                let t: f32 = if b1 { temp1 } else { temp2 };
                let p: Vec3 = r.point_at_parameter(t);
                let normal: Vec3 = (p - self.center) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    material: &self.material,
                })
            } else {
                None
            }
        } else {
            // ray never crosses the sphere
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(bbox)
    }
}

pub struct MovingSphere<M: Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: M,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center_at_time(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Material> Hitable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center_at_time(r.time);
        let a: f32 = r.direction.dot(&r.direction);
        let b: f32 = oc.dot(&r.direction);
        let c: f32 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;

        if discriminant > 0. {
            // ray crosses the sphere at least once
            let temp1: f32 = (-b - discriminant.sqrt()) / a; // close
            let temp2: f32 = (-b + discriminant.sqrt()) / a; // far
            let b1: bool = t_max > temp1 && temp1 > t_min;
            let b2: bool = t_max > temp2 && temp2 > t_min;

            if b1 || b2 {
                let t: f32 = if b1 { temp1 } else { temp2 };
                let p: Vec3 = r.point_at_parameter(t);
                let normal: Vec3 = (p - self.center_at_time(r.time)) / self.radius;
                let (u, v) = get_sphere_uv(&normal);
                Some(HitRecord {
                    t,
                    u,
                    v,
                    p,
                    normal,
                    material: self.material.borrow(),
                })
            } else {
                // the solution `t` doesn't exist between t_min & t_max
                None
            }
        } else {
            // ray never crosses the sphere
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox0 = AABB::new(
            self.center_at_time(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at_time(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let bbox1 = AABB::new(
            self.center_at_time(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at_time(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let sbox = surrounding_box(bbox0, bbox1);
        Some(sbox)
    }
}

fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
    let phi: f32 = p.z.atan2(p.x);
    let thera: f32 = p.y.asin();
    let u: f32 = 1.0 - (phi + PI) / (2.0 * PI);
    let v: f32 = (thera + PI / 2.0) / PI;
    (u, v)
}
