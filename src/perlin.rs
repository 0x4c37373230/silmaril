use crate::{random, vec3, Point3, Vec3};

pub struct Perlin {
    ran_vec: Vec<Vec3>,
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
        let mut ran_vec: Vec<Vec3> = vec![Vec3::new(None, None, None); Self::POINT_COUNT as usize];

        for i in 0..Self::POINT_COUNT {
            ran_vec[i as usize] = Vec3::unit_vector(Vec3::random(Some(-1.0), Some(1.0)));
        }

        Perlin {
            ran_vec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: &Point3) -> f32 {
        let mut u = point.x() - f32::floor(point.x());
        let mut v = point.y() - f32::floor(point.y());
        let mut w = point.z() - f32::floor(point.z());
        let i = f32::floor(point.x()) as i32;
        let j = f32::floor(point.y()) as i32;
        let k = f32::floor(point.z()) as i32;
        let mut c = [[[Vec3::new(None, None, None); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ran_vec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(&mut c, u, v, w)
    }

    fn trilinear_interp(c: &mut [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        // Hermite cubic to round off the interpolation
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0f32;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight =
                        Vec3::new(Some(u - i as f32), Some(v - j as f32), Some(w - k as f32));
                    accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                        * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                        * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                        * c[i][j][k].dot(&weight);
                }
            }
        }

        accum
    }
}
