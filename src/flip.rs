use crate::aabb::AABB;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct FlipNormals<H: Hitable> {
    pub hitable: H,
}

impl<H: Hitable> FlipNormals<H> {
    pub fn new(hitable: H) -> Self {
        FlipNormals { hitable }
    }
}

impl<H: Hitable> Hitable for FlipNormals<H> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec) = self.hitable.hit(&r, t_min, t_max) {
            //rec.normal = -rec.normal;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hitable.bounding_box(t0, t1)
    }
}
