use crate::hitable::*;
use crate::hitable_list::HitableList;
use crate::material::*;
use crate::sphere::{MovingSphere, Sphere};
use crate::vec3::Vec3;

use rand::prelude::*;
use std::sync::Arc;

pub fn random_scene() -> HitableList {
    let mut hitables: Vec<Arc<Hitable>> = vec![];
    let mut rng = rand::thread_rng();

    // earth
    hitables.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::new(0.5, 0.5, 0.5)),
    }));

    // a lot of small spheres
    for a in -5..5 {
        for b in -5..5 {
            let choose_mat: f32 = rng.gen();
            let center: Vec3 = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    hitables.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Lambertian::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    hitables.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(
                            (
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    }));
                } else {
                    // glass
                    hitables.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }

    // big three spheres
    hitables.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric::new(1.5)),
    }));
    hitables.push(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(0.4, 0.2, 0.1)),
    }));
    hitables.push(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new((0.7, 0.6, 0.5), 0.0)),
    }));

    HitableList { hitables }
}

pub fn random_scene_with_motion() -> HitableList {
    let mut hitables: Vec<Arc<Hitable>> = vec![];
    let mut rng = rand::thread_rng();

    // earth
    hitables.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian::new(0.5, 0.5, 0.5)),
    }));

    // a lot of small spheres
    for a in -5..5 {
        for b in -5..5 {
            let choose_mat: f32 = rng.gen();
            let center: Vec3 = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    hitables.push(Arc::new(MovingSphere {
                        center0: center,
                        center1: center + Vec3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    hitables.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(
                            (
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    }));
                } else {
                    // glass
                    hitables.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }

    // big three spheres
    hitables.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric::new(1.5)),
    }));
    hitables.push(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian::new(0.2, 0.6, 0.8)),
    }));
    hitables.push(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new((0.3, 0.2, 0.2), 0.0)),
    }));

    HitableList { hitables }
}
