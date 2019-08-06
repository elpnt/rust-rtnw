use crate::aabb::AABB;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::f32::consts::PI;

pub struct Translate {
    pub hitable: Box<dyn Hitable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(hitable: Box<dyn Hitable>, offset: Vec3) -> Self {
        Translate { hitable, offset }
    }
}

impl Hitable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut rec) = self.hitable.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(mut bbox) = self.hitable.bounding_box(t0, t1) {
            bbox.min += self.offset;
            bbox.max += self.offset;
            Some(bbox)
        } else {
            None
        }
    }
}

pub struct Rotate {
    pub hitable: Box<dyn Hitable>,
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub bbox: Option<AABB>,
}

impl Rotate {
    pub fn new(hitable: Box<dyn Hitable>, angle: f32) -> Self {
        let radians: f32 = (PI / 180.0) * angle;
        let sin_theta: f32 = radians.sin();
        let cos_theta: f32 = radians.cos();
        let bbox = hitable.bounding_box(0.0, 1.0).unwrap();
        let mut min = Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut max = Vec3::new(-std::f32::MAX, -std::f32::MAX, -std::f32::MAX);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x: f32 = i as f32 * bbox.max.x + (1.0 - i as f32) * bbox.min.x;
                    let y: f32 = j as f32 * bbox.max.y + (1.0 - j as f32) * bbox.min.y;
                    let z: f32 = k as f32 * bbox.max.z + (1.0 - k as f32) * bbox.min.z;
                    let new_x: f32 = cos_theta * x + sin_theta * z;
                    let new_z: f32 = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                        }
                        if tester[c] < min[c] {
                            min[c] = tester[c]
                        }
                    }
                }
            }
        }

        let bbox = AABB::new(min, max);
        Rotate {
            hitable,
            sin_theta,
            cos_theta,
            bbox: Some(bbox),
        }
    }
}

impl Hitable for Rotate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin: Vec3 = r.origin;
        let mut direction: Vec3 = r.direction;
        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;
        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;
        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;
        let rotated_r = Ray::new(origin, direction, r.time);
        if let Some(mut rec) = self.hitable.hit(&rotated_r, t_min, t_max) {
            let mut p: Vec3 = rec.p;
            let mut normal: Vec3 = rec.normal;
            p.x = self.cos_theta * rec.p.x - self.sin_theta * rec.p.z;
            p.z = self.sin_theta * rec.p.x + self.sin_theta * rec.p.z;
            normal.x = self.cos_theta * rec.normal.x - self.sin_theta * rec.normal.z;
            normal.z = self.sin_theta * rec.normal.z + self.cos_theta * rec.normal.z;
            rec.p = p;
            rec.normal = normal;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.bbox
    }
}
