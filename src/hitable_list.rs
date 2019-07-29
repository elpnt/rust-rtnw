use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use std::sync::Arc;

pub struct HitableList {
    pub hitables: Vec<Arc<Hitable>>,
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
}
