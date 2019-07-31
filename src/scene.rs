use crate::hitable::*;
use crate::hitable_list::HitableList;
use crate::material::*;
use crate::sphere::{MovingSphere, Sphere};
use crate::texture::*;
use crate::vec3::Vec3;

use rand::prelude::*;

pub fn random_scene() -> HitableList {
    let mut hitables: Vec<Box<Hitable>> = vec![];
    let mut rng = rand::thread_rng();

    // earth
    hitables.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian::new(0.5, 1.0, 0.5)),
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
                    hitables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    hitables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal::new(
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
                    hitables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }

    // big three spheres
    hitables.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric::new(1.5)),
    }));
    hitables.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian::new(0.4, 0.2, 0.1)),
    }));
    hitables.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal::new((0.7, 0.6, 0.5), 0.0)),
    }));

    HitableList { hitables }
}

pub fn random_scene_with_motion() -> HitableList {
    let mut hitables: Vec<Box<Hitable>> = vec![];
    let mut rng = rand::thread_rng();

    // earth
    let checker = CheckerTexture::new(
        Box::new(ConstantTexture::new(0.2, 0.2, 0.8)),
        Box::new(ConstantTexture::new(0.9, 0.9, 0.9)),
    );
    hitables.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian::new_with_texture(Box::new(checker))),
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
                    hitables.push(Box::new(MovingSphere {
                        center0: center,
                        center1: center + Vec3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Box::new(Lambertian::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    hitables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal::new(
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
                    hitables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }

    // big three spheres
    hitables.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Dielectric::new(1.5)),
    }));
    hitables.push(Box::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian::new(0.2, 0.6, 0.8)),
    }));
    hitables.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal::new((0.9, 0.8, 0.8), 0.0)),
    }));

    HitableList { hitables }
}

pub fn two_spheres() -> HitableList {
    let checker0 = Box::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(0.8, 0.1, 0.1)),
        Box::new(ConstantTexture::new(0.1, 0.1, 0.1)),
    ));
    let checker1 = Box::new(CheckerTexture::new(
        Box::new(ConstantTexture::new(0.7, 0.7, 0.1)),
        Box::new(ConstantTexture::new(0.1, 0.1, 0.1)),
    ));
    let hitables: Vec<Box<Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -10.0, 0.0),
            radius: 10.0,
            material: Box::new(Lambertian::new_with_texture(checker0)),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 10.0, 0.0),
            radius: 10.0,
            material: Box::new(Lambertian::new_with_texture(checker1)),
        }),
    ];

    HitableList { hitables }
}
