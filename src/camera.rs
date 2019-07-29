use crate::ray::Ray;
use crate::vec3::Vec3;
use rand;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let lens_radius: f32 = aperture / 2.0;
        let theta: f32 = vfov * PI / 180.0;
        let half_height: f32 = (theta / 2.0).tan();
        let half_width: f32 = aspect * half_height;
        let origin: Vec3 = lookfrom;

        let w: Vec3 = (lookfrom - lookat).unit_vector();
        let u: Vec3 = vup.cross(&w).unit_vector();
        let v: Vec3 = w.cross(&u);
        Camera {
            lower_left_corner: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3 = Vec3::new(1.1, 1.1, 1.1);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0)
            - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}
