use crate::{random, Point3};

pub struct Perlin {
    ran_float: Vec<f32>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const POINT_COUNT: i32 = 256;

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = vec![0; Self::POINT_COUNT as usize];

        for i in 0..Self::POINT_COUNT {
            p[i as usize] = i;
        }

        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<i32>, n: i32) {
        for i in (1..n - 1).rev() {
            let target = random::<i32>(0, i);
            let temp = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = temp;
        }
    }

    pub fn new() -> Perlin {
        let mut ran_float: Vec<f32> = vec![0.0; Self::POINT_COUNT as usize];

        for i in 0..Self::POINT_COUNT {
            ran_float[i as usize] = random::<f32>(0.0, 1.0);
        }

        Perlin {
            ran_float,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: &Point3) -> f32 {
        let i = (4.0 * point.x()) as i32 & 255;
        let j = (4.0 * point.y()) as i32 & 255;
        let k = (4.0 * point.z()) as i32 & 255;

        self.ran_float[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}
