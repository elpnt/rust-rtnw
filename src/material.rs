use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::texture::*;
use crate::vec3::Vec3;

use rand;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Lambertian {
            albedo: Box::new(ConstantTexture::new(x, y, z)),
        }
    }

    pub fn new_with_texture(albedo: Box<dyn Texture>) -> Self {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let target: Vec3 = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray {
            origin: rec.p,
            direction: target - rec.p,
            time: r_in.time,
        };
        // let attenuation: Vec3 = self.albedo.value(0.0, 0.0, &rec.p);
        let attenuation: Vec3 = self.albedo.value(rec.u, rec.v, &rec.p);

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zeros()
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(v: (f32, f32, f32), fuzz: f32) -> Self {
        Metal {
            albedo: Vec3::new(v.0, v.1, v.2),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let f: f32 = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflected: Vec3 = reflect(r_in.direction.unit_vector(), rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + f * random_in_unit_sphere(),
            time: r_in.time,
        };
        let attenuation: Vec3 = self.albedo;

        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zeros()
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::make_unit_vector();
    while p.squared_length() > 1.0 {
        p =
            2.0 * Vec3::new(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
            ) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

#[derive(Clone)]
pub struct Dielectric {
    pub refract_idx: f32,
}

impl Dielectric {
    pub fn new(refract_idx: f32) -> Self {
        Dielectric { refract_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected: Vec3 = reflect(r_in.direction, rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;
        let reflect_prob: f32;

        if r_in.direction.dot(&rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.refract_idx;
            cosine = self.refract_idx * r_in.direction.dot(&rec.normal) / r_in.direction.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.refract_idx;
            cosine = -r_in.direction.dot(&rec.normal) / r_in.direction.length();
        }

        match refract(&r_in.direction, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                reflect_prob = schlick(cosine, self.refract_idx);
                if rand::random::<f32>() > reflect_prob {
                    Some(ScatterRecord {
                        attenuation,
                        scattered: Ray::new(rec.p, refracted, r_in.time),
                    })
                } else {
                    Some(ScatterRecord {
                        attenuation,
                        scattered: Ray::new(rec.p, reflected, r_in.time),
                    })
                }
            }
            None => Some(ScatterRecord {
                attenuation,
                scattered: Ray::new(rec.p, reflected, r_in.time),
            }),
        }
    }

    fn emitted(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::zeros()
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv: Vec3 = v.unit_vector();
    let dt: f32 = uv.dot(&n);
    let discriminant: f32 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted: Vec3 = ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, refract_idx: f32) -> f32 {
    let mut r0: f32 = (1.0 - refract_idx) / (1.0 + refract_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}

#[derive(Clone)]
pub struct DiffuseLight {
    pub emit: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        DiffuseLight {
            emit: Box::new(ConstantTexture::new(x, y, z)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, &p)
    }
}
