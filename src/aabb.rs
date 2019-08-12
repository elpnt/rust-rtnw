use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d: f32 = 1.0 / r.direction[a];
            let mut t0: f32 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1: f32 = (self.max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let tmin: f32 = if t0 > tmin { t0 } else { tmin };
            let tmax: f32 = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(bbox0: AABB, bbox1: AABB) -> AABB {
    let small = Vec3::new(
        fmin(bbox0.min.x, bbox1.min.x),
        fmin(bbox0.min.y, bbox1.min.y),
        fmin(bbox0.min.z, bbox1.min.z),
    );
    let big = Vec3::new(
        fmax(bbox0.max.x, bbox1.max.x),
        fmax(bbox0.max.y, bbox1.max.y),
        fmax(bbox0.max.z, bbox1.max.z),
    );
    AABB::new(small, big)
}

fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn fmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
