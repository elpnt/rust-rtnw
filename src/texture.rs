use crate::perlin::*;
use crate::vec3::Vec3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
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
}

#[derive(Clone)]
pub struct CheckerTexture<T: Texture, U: Texture> {
    pub odd: T,
    pub even: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(odd: T, even: U) -> Self {
        CheckerTexture { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines: f32 = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
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
}

#[derive(Clone)]
pub struct ImageTexture {
    pub data: Vec<u8>,
    pub nx: u32,
    pub ny: u32,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, nx: u32, ny: u32) -> Self {
        ImageTexture { data, nx, ny }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let nx = self.nx as usize;
        let ny = self.ny as usize;
        let mut i = (u * nx as f32) as usize;
        let mut j = ((1.0 - v) * ny as f32) as usize;
        if i > nx - 1 {
            i = nx - 1
        };
        if j > ny - 1 {
            j = ny - 1
        };
        let idx: usize = 3 * i + 3 * nx * j;
        let r = self.data[idx] as f32 / 255.0;
        let g = self.data[idx + 1] as f32 / 255.0;
        let b = self.data[idx + 2] as f32 / 255.0;
        Vec3::new(r, g, b)
    }
}

fn clamp(x: f32, max: f32) -> f32 {
    x.min(0.0).max(max)
}
