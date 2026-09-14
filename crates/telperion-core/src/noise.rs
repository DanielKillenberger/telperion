use crate::{math::Vec3, rng::Rng};

#[derive(Clone)]
pub struct Noise {
    table: [u8; 512],
}
impl Noise {
    pub fn new(seed: u32) -> Self {
        let mut rng = Rng::new(seed);
        let mut table = [0; 512];
        for (i, v) in table[..256].iter_mut().enumerate() {
            *v = i as u8;
        }
        for i in (1..256).rev() {
            let j = (rng.next_f64() * (i + 1) as f64) as usize;
            table.swap(i, j);
        }
        for i in 0..256 {
            table[256 + i] = table[i];
        }
        Self { table }
    }
    pub fn at(&self, p: Vec3) -> f64 {
        let xi = p.x.floor();
        let yi = p.y.floor();
        let zi = p.z.floor();
        let x = p.x - xi;
        let y = p.y - yi;
        let z = p.z - zi;
        let cx = xi.rem_euclid(256.0) as usize;
        let cy = yi.rem_euclid(256.0) as usize;
        let cz = zi.rem_euclid(256.0) as usize;
        let t = &self.table;
        let a = t[cx] as usize + cy;
        let b = t[cx + 1] as usize + cy;
        let aa = t[a] as usize + cz;
        let ab = t[a + 1] as usize + cz;
        let ba = t[b] as usize + cz;
        let bb = t[b + 1] as usize + cz;
        let u = fade(x);
        let v = fade(y);
        let w = fade(z);
        lerp(
            lerp(
                lerp(gradient(t[aa], x, y, z), gradient(t[ba], x - 1.0, y, z), u),
                lerp(
                    gradient(t[ab], x, y - 1.0, z),
                    gradient(t[bb], x - 1.0, y - 1.0, z),
                    u,
                ),
                v,
            ),
            lerp(
                lerp(
                    gradient(t[aa + 1], x, y, z - 1.0),
                    gradient(t[ba + 1], x - 1.0, y, z - 1.0),
                    u,
                ),
                lerp(
                    gradient(t[ab + 1], x, y - 1.0, z - 1.0),
                    gradient(t[bb + 1], x - 1.0, y - 1.0, z - 1.0),
                    u,
                ),
                v,
            ),
            w,
        )
    }
    pub fn fbm(&self, p: Vec3) -> f64 {
        (self.at(p) + 0.5 * self.at(p * 2.0)) / 1.5
    }
    pub fn curl(&self, p: Vec3, wavelength: f64) -> Vec3 {
        let p = p / if wavelength > 0.0 { wavelength } else { 1.0 };
        let offsets = [
            Vec3::ZERO,
            Vec3::new(31.416, 17.271, 5.772),
            Vec3::new(-11.331, 43.5, 27.183),
        ];
        let derivative = |component: usize, axis: Vec3| {
            (self.fbm(p + offsets[component] + axis * 0.001)
                - self.fbm(p + offsets[component] - axis * 0.001))
                * 500.0
        };
        Vec3::new(
            derivative(2, Vec3::Y) - derivative(1, Vec3::Z),
            derivative(0, Vec3::Z) - derivative(2, Vec3::X),
            derivative(1, Vec3::X) - derivative(0, Vec3::Y),
        )
    }
}
/// The same gradient noise without a permutation table: every lattice corner's
/// gradient is hashed from the corner and the seed, so a caller that holds a
/// seed and no state of its own samples it. `Noise` keeps its table for the
/// bias field, which samples one instance millions of times; the envelope's
/// outline is queried from plain values and cannot carry one.
pub fn seeded(seed: u32, p: Vec3) -> f64 {
    let (xi, yi, zi) = (p.x.floor(), p.y.floor(), p.z.floor());
    let (x, y, z) = (p.x - xi, p.y - yi, p.z - zi);
    let corner = |dx: i64, dy: i64, dz: i64| {
        let hash = corner_hash(seed, xi as i64 + dx, yi as i64 + dy, zi as i64 + dz);
        gradient(hash, x - dx as f64, y - dy as f64, z - dz as f64)
    };
    let (u, v, w) = (fade(x), fade(y), fade(z));
    lerp(
        lerp(
            lerp(corner(0, 0, 0), corner(1, 0, 0), u),
            lerp(corner(0, 1, 0), corner(1, 1, 0), u),
            v,
        ),
        lerp(
            lerp(corner(0, 0, 1), corner(1, 0, 1), u),
            lerp(corner(0, 1, 1), corner(1, 1, 1), u),
            v,
        ),
        w,
    )
}
/// One gradient index per lattice corner: integer mixing only, so the same
/// seed and the same corner give the same gradient on every target.
fn corner_hash(seed: u32, x: i64, y: i64, z: i64) -> u8 {
    let mut h = seed ^ 0x9e37_79b9;
    for v in [x, y, z] {
        h = (h ^ v as u32).wrapping_mul(0x85eb_ca6b);
        h ^= h >> 13;
    }
    (h ^ (h >> 16)) as u8
}
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
fn gradient(hash: u8, x: f64, y: f64, z: f64) -> f64 {
    const G: [[f64; 3]; 12] = [
        [1.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0],
        [1.0, -1.0, 0.0],
        [-1.0, -1.0, 0.0],
        [1.0, 0.0, 1.0],
        [-1.0, 0.0, 1.0],
        [1.0, 0.0, -1.0],
        [-1.0, 0.0, -1.0],
        [0.0, 1.0, 1.0],
        [0.0, -1.0, 1.0],
        [0.0, 1.0, -1.0],
        [0.0, -1.0, -1.0],
    ];
    let g = G[hash as usize % 12];
    g[0] * x + g[1] * y + g[2] * z
}
