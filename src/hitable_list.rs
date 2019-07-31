use crate::aabb::*;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use std::sync::Arc;

pub struct HitableList {
    pub hitables: Vec<Arc<Hitable>>,
}

impl HitableList {
    pub fn size(&self) -> usize {
        self.hitables.len()
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far: f32 = t_max;
        let mut hit_anything: Option<HitRecord> = None;
        for hitable in &self.hitables {
            if let Some(hit_record) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some(hit_record);
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.size() > 0 {
            let first_box = self.hitables[0].bounding_box(t0, t1);
            if let Some(bbox) = first_box {
                for i in 1..self.size() {
                    if let Some(temp_box) = self.hitables[i].bounding_box(t0, t1) {
                        let bbox = surrounding_box(bbox, temp_box);
                    } else {
                        return None;
                    }
                }
                Some(bbox)
            } else {
                None
            }
        } else {
            None
        }
    }
}
