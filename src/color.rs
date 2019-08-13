use crate::hitable::*;
use crate::hitable_list::HitableList;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn color(r: &Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let emitted: Vec3 = rec.material.emitted(rec.u, rec.v, &rec.p);
        if depth < 50 {
            if let Some(scatter_record) = rec.material.scatter(&r, &rec) {
                let attenuation: Vec3 = scatter_record.attenuation;
                let scattered: Ray = scatter_record.scattered;
                return emitted + attenuation * color(&scattered, &world, depth + 1);
            }
        }
        emitted
    } else {
        Vec3::zeros()
    }
}
