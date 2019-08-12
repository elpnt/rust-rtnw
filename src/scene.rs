use crate::block::*;
use crate::flip::FlipNormals;
use crate::hitable::Hitable;
use crate::hitable_list::HitableList;
use crate::material::*;
use crate::medium::ConstantMedium;
use crate::rectangle::*;
use crate::sphere::{MovingSphere, Sphere};
use crate::texture::*;
use crate::translate::{Rotate, Translate};
use crate::vec3::Vec3;

use image;
use rand::prelude::*;

pub fn random_scene() -> HitableList {
    let mut hitables: Vec<Box<dyn Hitable>> = vec![];
    let mut rng = rand::thread_rng();

    // earth
    hitables.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::new(ConstantTexture::new(0.5, 1.0, 0.5)),
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
                        material: Lambertian::new(ConstantTexture::new(
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
                        material: Metal::new(
                            (
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        ),
                    }));
                } else {
                    // glass
                    hitables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric::new(1.5),
                    }));
                }
            }
        }
    }

    // big three spheres
    hitables.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric::new(1.5),
    }));
    hitables.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian::new(ConstantTexture::new(0.4, 0.2, 0.1)),
    }));
    hitables.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal::new((0.7, 0.6, 0.5), 0.0),
    }));

    HitableList { hitables }
}

pub fn random_scene_with_motion() -> HitableList {
    let mut hitables: Vec<Box<dyn Hitable>> = vec![];
    let mut rng = rand::thread_rng();

    // earth
    let checker = CheckerTexture::new(
        ConstantTexture::new(0.2, 0.2, 0.8),
        ConstantTexture::new(0.9, 0.9, 0.9),
    );
    hitables.push(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian::new(checker),
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
                        material: Lambertian::new(ConstantTexture::new(
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
                        material: Metal::new(
                            (
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        ),
                    }));
                } else {
                    // glass
                    hitables.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric::new(1.5),
                    }));
                }
            }
        }
    }

    // big three spheres
    hitables.push(Box::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Dielectric::new(1.5),
    }));
    hitables.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Lambertian::new(ConstantTexture::new(0.2, 0.6, 0.8)),
    }));
    hitables.push(Box::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Metal::new((0.9, 0.8, 0.8), 0.0),
    }));

    HitableList { hitables }
}

pub fn two_spheres() -> HitableList {
    let checker = CheckerTexture::new(
        ConstantTexture::new(0.8, 0.1, 0.1),
        ConstantTexture::new(0.0, 0.0, 0.0),
    );
    let hitables: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -10.0, 0.0),
            radius: 10.0,
            material: Lambertian::new(checker.clone()),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 10.0, 0.0),
            radius: 10.0,
            material: Lambertian::new(checker.clone()),
        }),
    ];

    HitableList { hitables }
}

pub fn two_perlin_spheres() -> HitableList {
    let pertext = NoiseTexture::new(20.0);
    let hitables: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Lambertian::new(pertext.clone()),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Lambertian::new(pertext.clone()),
        }),
    ];

    HitableList { hitables }
}

pub fn earth() -> HitableList {
    let image_data = image::open("./texture/earth.jpg").unwrap().to_rgb();
    let (nx, ny): (u32, u32) = image_data.dimensions();
    let texture_data: Vec<u8> = image_data.into_raw();
    let image_texture = ImageTexture::new(texture_data, nx, ny);
    let pertext = NoiseTexture::new(18.0);
    let hitables: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Lambertian::new(pertext),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Lambertian::new(image_texture),
        }),
    ];

    HitableList { hitables }
}

pub fn simple_light() -> HitableList {
    let pertext = NoiseTexture::new(4.0);
    let hitables: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(pertext.clone()),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian::new(pertext.clone()),
        )),
        Box::new(Rectangle::new(
            Plane::XY,
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            DiffuseLight::new(ConstantTexture::new(4.0, 4.0, 4.0)),
        )),
    ];

    HitableList { hitables }
}

pub fn cornell_box() -> HitableList {
    let red = Lambertian::new(ConstantTexture::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(ConstantTexture::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(ConstantTexture::new(15.0, 15.0, 15.0));

    let hitables: Vec<Box<dyn Hitable>> = vec![
        Box::new(FlipNormals::new(Rectangle::new(
            Plane::YZ,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            green,
        ))),
        Box::new(Rectangle::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red)),
        Box::new(Rectangle::new(
            Plane::ZX,
            227.0,
            332.0,
            213.0,
            343.0,
            554.0,
            light,
        )),
        Box::new(FlipNormals::new(Rectangle::new(
            Plane::ZX,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        ))),
        Box::new(Rectangle::new(
            Plane::ZX,
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )),
        Box::new(FlipNormals::new(Rectangle::new(
            Plane::XY,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        ))),
    ];
    HitableList { hitables }
}

pub fn blocks() -> HitableList {
    let red = Lambertian::new(ConstantTexture::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(ConstantTexture::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(ConstantTexture::new(15.0, 15.0, 15.0));

    let box1 = Translate::new(
        Rotate::new(
            Block::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            ),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    let box2 = Translate::new(
        Rotate::new(
            Block::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white.clone(),
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );
    let hitables: Vec<Box<dyn Hitable>> = vec![
        Box::new(FlipNormals::new(Rectangle::new(
            Plane::YZ,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            green,
        ))),
        Box::new(Rectangle::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red)),
        Box::new(Rectangle::new(
            Plane::ZX,
            127.0,
            432.0,
            113.0,
            443.0,
            554.0,
            light,
        )),
        Box::new(FlipNormals::new(Rectangle::new(
            Plane::ZX,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        ))),
        Box::new(Rectangle::new(
            Plane::ZX,
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )),
        Box::new(FlipNormals::new(Rectangle::new(
            Plane::XY,
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        ))),
        Box::new(ConstantMedium::new(
            box1,
            0.01,
            ConstantTexture::new(1.0, 1.0, 1.0),
        )),
        Box::new(ConstantMedium::new(
            box2,
            0.01,
            ConstantTexture::new(0.0, 0.0, 0.0),
        )),
    ];
    HitableList { hitables }
}
