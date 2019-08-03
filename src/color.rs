use crate::hitable::*;
use crate::hitable_list::HitableList;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn color(r: &Ray, world: &HitableList, depth: u32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let emitted: Vec3 = rec.material.emitted(rec.u, rec.v, &rec.p);
        if let Some(scatter_record) = rec.material.scatter(&r, &rec) {
            if depth < 50 {
                let attenuation: Vec3 = scatter_record.attenuation;
                let scattered: Ray = scatter_record.scattered;
                emitted + attenuation * color(&scattered, &world, depth + 1)
            } else {
                emitted
            }
        } else {
            emitted
        }
    } else {
        /*
        let unit_direction: Vec3 = r.direction.unit_vector();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        */
        Vec3::zeros()
    }
}
