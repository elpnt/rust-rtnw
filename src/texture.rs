use crate::perlin::*;
use crate::vec3::Vec3;
use std::clone::Clone;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
    fn box_clone(&self) -> Box<dyn Texture>;
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Self {
        self.as_ref().box_clone()
    }
}

#[derive(Clone)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        ConstantTexture {
            color: Vec3::new(x, y, z),
        }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Self {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines: f32 = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
    }
    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        // Vec3::make_unit_vector() * self.noise.noise(&(*p * self.scale))

        // marble-like texture
        Vec3::make_unit_vector()
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(&p, 7)).sin())
    }
    fn box_clone(&self) -> Box<dyn Texture> {
        Box::new((*self).clone())
    }
}
