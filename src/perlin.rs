use crate::vec3::Vec3;
use rand::prelude::*;

#[derive(Clone)]
pub struct Perlin {
    pub ranfloat: Vec<f32>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranfloat: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let i: usize = (4 * p.x as usize) & 255;
        let j: usize = (4 * p.y as usize) & 255;
        let k: usize = (4 * p.z as usize) & 255;
        println!("{}, {}, {}", i, j, k);
        self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

fn perlin_generate() -> Vec<f32> {
    let mut rng = thread_rng();
    let p: Vec<f32> = (0..256).map(|_| rng.gen::<f32>()).collect();
    p
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut rng = thread_rng();
    let mut p: Vec<usize> = (0..256).collect();
    p.shuffle(&mut rng);
    p
}
