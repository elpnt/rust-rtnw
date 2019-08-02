use crate::vec3::Vec3;
use rand::prelude::*;

#[derive(Clone)]
pub struct Perlin {
    pub ranvec: Vec<Vec3>,
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranvec: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::zeros(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        perlin_interp(&c, u, v, w)
    }

    pub fn turbulence(&self, p: &Vec3, depth: usize) -> f32 {
        let mut accum: f32 = 0.0;
        let mut temp_p: Vec3 = *p;
        let mut weight: f32 = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}

fn perlin_generate() -> Vec<Vec3> {
    let mut rng = thread_rng();
    let mut p = vec![];
    for _ in 0..256 {
        p.push(Vec3::new(
            -1.0 + 2.0 * rng.gen::<f32>(),
            -1.0 + 2.0 * rng.gen::<f32>(),
            -1.0 + 2.0 * rng.gen::<f32>(),
        ));
    }
    p
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut rng = thread_rng();
    let mut p: Vec<usize> = (0..256).collect();
    p.shuffle(&mut rng);
    p
}

fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu: f32 = u * u * (3.0 - 2.0 * u);
    let vv: f32 = v * v * (3.0 - 2.0 * v);
    let ww: f32 = w * w * (3.0 - 2.0 * w);
    let mut accum: f32 = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                    * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                    * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                    * c[i][j][k].dot(&weight_v);
            }
        }
    }
    accum
}

/*
fn trilinear_interp(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum: f32 = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f32 * u + (1.0 - i as f32) * (1.0 - u))
                    * (j as f32 * v + (1.0 - j as f32) * (1.0 - v))
                    * (k as f32 * w + (1.0 - k as f32) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    accum
}
*/
