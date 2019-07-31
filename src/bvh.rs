use crate::aabb::*;
use crate::hitable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct BVHNode {
    pub left: Box<Hitable>,
    pub right: Box<Hitable>,
    pub bbox: AABB,
}

impl BVHNode {
    // pub fn new(n: u32, time0: f32, time1: f32) -> Self {

    // }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(hit_rec) = self.bbox.hit(&r, t_min, t_max) {
            let left_rec = self.left.hit(&r, t_min, t_max);
            let right_rec = self.right.hit(&r, t_min, t_max);

            None
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}
