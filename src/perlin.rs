use crate::{
    utils::{random_f64, random_range_int},
    vec3::{Vec3, dot},
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x() - f64::floor(p.x());
        let v = p.y() - f64::floor(p.y());
        let w = p.z() - f64::floor(p.z());

        let i = f64::floor(p.x()) as usize;
        let j = f64::floor(p.y()) as usize;
        let k = f64::floor(p.z()) as usize;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Vec3, depth: usize) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        f64::abs(acc)
    }
}

impl Default for Perlin {
    fn default() -> Self {
        // SAFETY: Our initialization here assumes nothing about the contents
        // of `perlin`, and puts `perlin` into a valid state before returning.
        let mut perlin: Perlin = unsafe { std::mem::zeroed() };
        perlin.randvec = std::array::from_fn(|_| Vec3::random_range(-1.0, 1.0));
        perlin_generate_perm(&mut perlin.perm_x);
        perlin_generate_perm(&mut perlin.perm_y);
        perlin_generate_perm(&mut perlin.perm_z);
        perlin
    }
}

fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
    for i in (1..n).rev() {
        let target: usize = random_range_int(0, i as i32) as usize;
        p.swap(target, i);
    }
}

fn perlin_generate_perm(p: &mut [usize; POINT_COUNT]) {
    for i in 0..POINT_COUNT {
        p[i] = i;
    }
    permute(p, POINT_COUNT);
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                acc += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                    * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                    * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                    * dot(c[i][j][k], weight_v);
            }
        }
    }
    acc
}
