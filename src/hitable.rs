use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

// `Hitalbe` trait needs `Send` and `Sync` for `rayon` parallel processing.
pub trait Hitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
