use crate::aabb::AABB;
use crate::flip::FlipNormals;
use crate::hitable::*;
use crate::hitable_list::HitableList;
use crate::material::*;
use crate::ray::Ray;
use crate::rectangle::{Plane, Rectangle};
use crate::vec3::Vec3;

pub struct Block {
    pub pmin: Vec3,
    pub pmax: Vec3,
    pub hitablelist: HitableList,
}

impl Block {
    pub fn new<M: Material + Clone + 'static>(pmin: Vec3, pmax: Vec3, material: M) -> Self {
        let hitables: Vec<Box<dyn Hitable>> = vec![
            Box::new(Rectangle::new(
                Plane::XY,
                pmin.x,
                pmax.x,
                pmin.y,
                pmax.y,
                pmax.z,
                material.clone(),
            )),
            Box::new(FlipNormals::new(Rectangle::new(
                Plane::XY,
                pmin.x,
                pmax.x,
                pmin.y,
                pmax.y,
                pmin.z,
                material.clone(),
            ))),
            Box::new(Rectangle::new(
                Plane::ZX,
                pmin.z,
                pmax.z,
                pmin.x,
                pmax.x,
                pmax.y,
                material.clone(),
            )),
            Box::new(FlipNormals::new(Rectangle::new(
                Plane::ZX,
                pmin.z,
                pmax.z,
                pmin.x,
                pmax.x,
                pmin.y,
                material.clone(),
            ))),
            Box::new(Rectangle::new(
                Plane::YZ,
                pmin.y,
                pmax.y,
                pmin.z,
                pmax.z,
                pmax.x,
                material.clone(),
            )),
            Box::new(FlipNormals::new(Rectangle::new(
                Plane::YZ,
                pmin.y,
                pmax.y,
                pmin.z,
                pmax.z,
                pmin.x,
                material.clone(),
            ))),
        ];

        Block {
            pmin,
            pmax,
            hitablelist: HitableList { hitables },
        }
    }
}

impl Hitable for Block {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitablelist.hit(&r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox = AABB::new(self.pmin, self.pmax);
        Some(bbox)
    }
}
