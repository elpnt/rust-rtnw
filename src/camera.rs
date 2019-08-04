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
    pub time0: f32, // shutter open time
    pub time1: f32, // shutter close time
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
        t0: f32,
        t1: f32,
    ) -> Self {
        let time0: f32 = t0;
        let time1: f32 = t1;
        let lens_radius: f32 = aperture / 2.0;
        let theta: f32 = vfov * PI / 180.0;
        let half_height: f32 = (theta / 2.0).tan();
        let half_width: f32 = aspect * half_height;
        let origin: Vec3 = lookfrom;

        let w: Vec3 = (lookfrom - lookat).unit_vector();
        let u: Vec3 = vup.cross(&w).unit_vector();
        let v: Vec3 = w.cross(&u);
        let lower_left_corner: Vec3 =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal: Vec3 = 2.0 * half_width * focus_dist * u;
        let vertical: Vec3 = 2.0 * half_height * focus_dist * v;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time0,
            time1,
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
            time: self.time0 + rand::random::<f32>() * (self.time1 - self.time0),
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

pub fn camera_for_random_spheres(nx: u32, ny: u32) -> Camera {
    let lookfrom: Vec3 = Vec3::new(10.0, 1.7, 3.0);
    let lookat: Vec3 = Vec3::new(0.0, 0.8, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let vfov: f32 = 30.0;
    let aspect: f32 = nx as f32 / ny as f32;
    let aperture: f32 = 0.05;
    let dist_to_focus: f32 = (lookfrom - lookat).length();
    let time0: f32 = 0.0;
    let time1: f32 = 1.0;
    Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect,
        aperture,
        dist_to_focus,
        time0,
        time1,
    )
}

pub fn camera_for_two_spheres(nx: u32, ny: u32) -> Camera {
    let lookfrom = Vec3::new(20.0, 5.0, 10.0);
    let lookat = Vec3::new(0.0, 2.0, 0.0);
    let dist_to_focus: f32 = 10.0;
    let aperture: f32 = 0.0;

    Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        dist_to_focus,
        0.0,
        0.0,
    )
}

pub fn camera_for_cornell_box(nx: u32, ny: u32) -> Camera {
    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus: f32 = 10.0;
    let aperture: f32 = 0.0;
    let vfov: f32 = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aspect = nx as f32 / ny as f32;

    Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    )
}
