use crate::hitable::*;
use crate::hitable_list::HitableList;
use crate::material::*;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

use rand::prelude::*;
use std::sync::Arc;

pub fn room_scene() -> HitableList {
    let hitables: Vec<Arc<Hitable>> = vec![
        // Left wall
        Arc::new(Sphere {
            center: Vec3::new(1e5 + 1.0, 40.8, 81.6),
            radius: 1e5,
            material: Arc::new(Lambertian::new(0.75, 0.25, 0.25)),
        }),
        // Right wall
        Arc::new(Sphere {
            center: Vec3::new(-1e5 + 99.0, 40.8, 81.6),
            radius: 1e5,
            material: Arc::new(Lambertian::new(0.25, 0.25, 0.75)),
        }),
        // Back wall
        Arc::new(Sphere {
            center: Vec3::new(50.0, 40.8, 1e5),
            radius: 1e5,
            material: Arc::new(Lambertian::new(0.75, 0.75, 0.75)),
        }),
        // Front wall
        // Arc::new(Sphere {
        //     center: Vec3::new(50.0, 40.8, -1e5 + 170.0),
        //     radius: 1e5,
        //     material: Arc::new(Lambertian::new(0.0, 0.0, 0.0)),
        // }),
        // Floor
        // Arc::new(Sphere {
        //     center: Vec3::new(50.0, 1e5, 81.6),
        //     radius: 1e5,
        //     material: Arc::new(Lambertian::new(0.75, 0.75, 0.75)),
        // }),
        // Ceil
        Arc::new(Sphere {
            center: Vec3::new(50.0, -1e5 + 81.6, 81.6),
            radius: 1e5,
            material: Arc::new(Lambertian::new(0.75, 0.75, 0.75)),
        }),
        // small sphere
        Arc::new(Sphere {
            center: Vec3::new(50.0, 50.0, -10.0),
            radius: 25.0,
            material: Arc::new(Lambertian::new(1.0, 0., 0.)),
        }),
    ];

    HitableList { hitables }
}

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
    for a in -10..10 {
        for b in -10..10 {
            let choose_mat: f32 = rng.gen();
            let center: Vec3 = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.5 {
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
                } else if choose_mat < 0.9 {
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
