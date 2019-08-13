use rand;
use rayon::prelude::*;
use std::fs;
use std::io::{BufWriter, Write};
use std::time::Instant;

mod aabb;
mod block;
mod camera;
mod color;
mod flip;
mod hitable;
mod hitable_list;
mod material;
mod medium;
mod perlin;
mod ray;
mod rectangle;
mod scene;
mod sphere;
mod texture;
mod translate;
mod vec3;

use color::color;
use ray::Ray;
use vec3::Vec3;

fn main() {
    let nx: u32 = 300;
    let ny: u32 = 300;
    let ns: u32 = 40;

    // Objects setup
    let world = scene::cornell_box();

    // Camera setup
    let cam = camera::camera_for_cornell_box(nx, ny);

    // Parallell process
    let start = Instant::now();

    let mut f = BufWriter::new(fs::File::create("./output/debug.ppm").unwrap());
    f.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())
        .unwrap();

    let par_vec: Vec<(u32, u32)> = {
        let mut v: Vec<(u32, u32)> = vec![];

        for j in (0..ny).rev() {
            for i in 0..nx {
                v.push((i, j));
            }
        }
        v
    };

    let pixels: Vec<Vec3> = par_vec
        .par_iter()
        .cloned()
        .map(|(i, j)| {
            let mut col: Vec3 = (0..ns)
                .into_par_iter()
                .map(|_| {
                    let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                    let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                    let r: Ray = cam.get_ray(u, v);
                    color(&r, &world, 0)
                })
                .reduce(|| Vec3::new(0.0, 0.0, 0.0), |sum, x| sum + x);
            col /= ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            col
        })
        .collect();

    for pix in pixels {
        let ir = (255.99 * pix.x.max(0.0).min(1.0)) as i32;
        let ig = (255.99 * pix.y.max(0.0).min(1.0)) as i32;
        let ib = (255.99 * pix.z.max(0.0).min(1.0)) as i32;
        f.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())
            .unwrap();
    }

    let duration = start.elapsed();
    println!("Elapsed time: {:?}", duration);
}
