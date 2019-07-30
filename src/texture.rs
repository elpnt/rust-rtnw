use crate::vec3::Vec3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    pub color: Vec3,
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.color
    }
}

impl ConstantTexture {
    fn new(x: f32, y: f32, z: f32) -> Self {
        ConstantTexture {
            color: Vec3::new(x, y, z),
        }
    }
}

pub struct CheckerTexture<'a> {
    pub odd: &'a dyn Texture,
    pub even: &'a dyn Texture,
}

impl Texture for CheckerTexture<'_> {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines: f32 = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
    }
}
