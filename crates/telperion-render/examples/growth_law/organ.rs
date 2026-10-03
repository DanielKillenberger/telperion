//! R1a (fn-190 probe, scratch): one organogenesis rule from trunk to twig,
//! driven by a continuous physiological age phi (Barthélémy & Caraglio
//! 2007's axis categories as one variable; GreenLab-style organogenesis).
//! Every bud grows one unit a year whose metamers, internodes, laterals,
//! their phi and place along it, lean and fate follow from phi alone; short
//! shoots are the same rule at phi near 1. No light, no allocation, no
//! shedding, no envelope: three passes in all (the yearly bud sweep, one
//! pipe count, one straightening). Integer outcomes are taken in expectation
//! with keyed randomness, so a small change is a small change of the tree.
use crate::params::Params;
use std::collections::HashSet;
use std::time::Instant;
use telperion_core::math::Vec3;

#[derive(Clone, Copy)]
struct Bud {
    node: u32,
    dir: Vec3,
    /// The axis's horizontal outward direction (zero on the stem).
    out: Vec3,
    phi: f64,
    /// Years this axis has grown (drift).
    years: u16,
    terminal: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub cycles: usize,
    pub buds_max: usize,
    /// Cold-run ms of the three passes.
    pub ms: [f64; 3],
    pub laterals: usize,
    /// Distinct (phi, birth year) keys among laterals (factorisation probe).
    pub distinct: usize,
    pub distinct_phi: usize,
}
pub const STAGES: [&str; 3] = ["sweep", "pipe", "straighten"];

/// The grown tree, parent before child.
pub struct Grown {
    pub pos: Vec<Vec3>,
    pub parent: Vec<Option<usize>>,
    pub lateral: Vec<bool>,
    pub radius: Vec<f64>,
    pub elev0: Vec<f64>,
    pub stats: Stats,
}

pub struct World {
    pub height: f64,
    pub root_radius: f64,
    pub exponent: f64,
    pub seed: u64,
}

/// A uniform number in [0, 1) from a key and a salt.
fn hash(key: u64, salt: u64) -> f64 {
    let mut z = key.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ salt.wrapping_mul(0xD1B5_4A32_D192_ED03);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 53) as f64
}

/// `x` rounded to an integer in expectation.
fn keyed_round(x: f64, key: u64, salt: u64) -> usize {
    let f = x.max(0.0);
    f.floor() as usize + usize::from(hash(key, salt) < f.fract())
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

struct Grower {
    p: Params,
    w: World,
    unit: f64,
    pos: Vec<Vec3>,
    dir: Vec<Vec3>,
    len: Vec<f64>,
    parent: Vec<u32>,
    lateral: Vec<bool>,
    born: Vec<u16>,
    elev0: Vec<f32>,
    buds: Vec<Bud>,
    keys: Option<Vec<(u64, u16)>>,
}

/// Grows the tree; `diag` also records the factorisation probe (untimed
/// runs only). A tree past `maxNodes` is an error.
pub fn grow(p: &Params, w: World, diag: bool) -> Result<Grown, String> {
    let unit = p.unit * w.height;
    let cap = p.max_nodes as usize;
    let mut g = Grower {
        p: *p,
        w,
        unit,
        pos: vec![Vec3::ZERO],
        dir: vec![Vec3::Y],
        len: vec![0.0],
        parent: vec![u32::MAX],
        lateral: vec![false],
        born: vec![0],
        elev0: vec![f32::NAN],
        buds: vec![Bud { node: 0, dir: Vec3::Y, out: Vec3::ZERO, phi: 0.0, years: 0, terminal: true }],
        keys: diag.then(Vec::new),
    };
    let mut st = Stats::default();
    let t0 = Instant::now();
    for t in 0..p.cycles.round() as usize {
        st.cycles += 1;
        for b in std::mem::take(&mut g.buds) {
            g.unit_of(b, t as u16);
        }
        st.buds_max = st.buds_max.max(g.buds.len());
        if g.pos.len() > cap {
            return Err(format!("grew past maxNodes ({cap}) in year {t}"));
        }
    }
    st.ms[0] = ms(t0);
    if let Some(k) = &g.keys {
        st.laterals = k.len();
        st.distinct = k.iter().copied().collect::<HashSet<_>>().len();
        st.distinct_phi = k.iter().map(|x| x.0).collect::<HashSet<_>>().len();
    }
    Ok(g.finish(st))
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}

impl Grower {
    fn key(&self, i: usize) -> u64 {
        (i as u64).wrapping_mul(0x2545_F491_4F6C_DD1D) ^ self.w.seed.rotate_left(17)
    }

    fn push(&mut self, at: u32, d: Vec3, len: f64, lateral: bool, t: u16) -> u32 {
        let idx = self.pos.len() as u32;
        self.pos.push(self.pos[at as usize] + d * len);
        self.dir.push(d);
        self.len.push(len);
        self.parent.push(at);
        self.lateral.push(lateral);
        self.born.push(t);
        self.elev0.push(if lateral { d.y.clamp(-1.0, 1.0).asin().to_degrees() as f32 } else { f32::NAN });
        idx
    }

    /// One bud's yearly growth unit.
    fn unit_of(&mut self, b: Bud, t: u16) {
        let p = self.p;
        let base = b.node as usize;
        let bkey = self.key(base) ^ if b.terminal { 0 } else { b.dir.x.to_bits() };
        let phi = (b.phi + p.drift * f64::from(b.years)).clamp(0.0, 1.0);
        let n = keyed_round(lerp(p.n0, p.n1, phi), bkey, 11 + u64::from(t));
        if n == 0 {
            self.buds.push(Bud { years: b.years + 1, ..b });
            return;
        }
        let len = self.unit * lerp(1.0, p.short, phi);
        let persists = hash(bkey, 5 + u64::from(t)) < lerp(p.persist0, p.persist1, phi.powf(p.persist_shape));
        let out = if b.terminal {
            b.out
        } else {
            let f = Vec3::new(b.dir.x, 0.0, b.dir.z);
            if f.length() > 1e-9 { f.normalized() } else { Vec3::ZERO }
        };
        let lean = (p.lean0 + p.lean1 * phi).clamp(0.0, 1.0);
        let trop = if out.length() > 0.0 { (Vec3::Y * (1.0 - lean) + out * lean).normalized() } else { Vec3::Y };
        let dev = p.branching * (1.0 - phi).powf(p.fate);
        let mean = (0..n).map(|k| self.profile(k, n)).sum::<f64>() / n as f64;
        let mut d = b.dir;
        let mut at = b.node;
        for k in 0..n {
            d = (d + trop * p.eta).normalized();
            at = self.push(at, d, len, !b.terminal && k == 0, t);
            let wk = (1.0 - p.rhythm) * self.profile(k, n) / mean + p.rhythm * if k + 1 == n { n as f64 } else { 0.0 };
            let nk = self.key(at as usize);
            let count = keyed_round(dev * wk, nk, 3);
            if count == 0 {
                continue;
            }
            let u = (k + 1) as f64 / n as f64;
            let mut phi_l = phi + p.phi_step + p.zone * (1.0 - u);
            if !persists && k + 1 == n {
                // The terminal aborts: the distal laterals relay it, as
                // reiterates of the axis (toward its phi at birth), as
                // readily as the axis is young: (1 - phi at birth)^reiterShape.
                phi_l = lerp(phi_l, b.phi, p.reiteration * (1.0 - b.phi).powf(p.reiter_shape));
            }
            let phi_l = phi_l.clamp(0.0, 1.0);
            let angle = lerp(p.angle0, p.angle1, phi_l).to_radians();
            for j in 0..count {
                let dir = self.bud_dir(d, nk, j, count, k, angle);
                self.buds.push(Bud { node: at, dir, out: Vec3::ZERO, phi: phi_l, years: 0, terminal: false });
                if let Some(keys) = &mut self.keys {
                    keys.push((phi_l.to_bits(), t));
                }
            }
        }
        if persists {
            self.buds.push(Bud { node: at, dir: d, out, phi: b.phi, years: b.years + 1, terminal: true });
        }
    }

    fn profile(&self, k: usize, n: usize) -> f64 {
        (self.p.acrotony * 3.0 * ((k as f64 + 1.0) / n as f64 - 0.5)).exp()
    }

    /// A lateral bud's direction: `angle` off the shoot, its bearing the
    /// golden angle by key blended toward two ranks about the shoot's
    /// horizontal side; buds of one node spread evenly around it.
    fn bud_dir(&self, d: Vec3, key: u64, j: usize, of: usize, k: usize, angle: f64) -> Vec3 {
        let a0 = d.perpendicular().normalized();
        let b0 = d.cross(a0);
        let golden = (key % 4096) as f64 * 137.5_f64.to_radians() + j as f64 * std::f64::consts::TAU / of as f64;
        let side = d.cross(Vec3::Y);
        let two = if side.length() > 1e-9 {
            let s = side.normalized();
            s.dot(b0).atan2(s.dot(a0)) + (k + j) as f64 % 2.0 * std::f64::consts::PI
        } else {
            golden
        };
        let tau = std::f64::consts::TAU;
        let mut diff = (two - golden).rem_euclid(tau);
        if diff > std::f64::consts::PI {
            diff -= tau;
        }
        let axis = a0.rotate(d, golden + self.p.distich * diff);
        d.rotate(axis, angle).normalized()
    }

    /// Pipe counts and radii, then straightening: each segment turns toward
    /// up by straighten x its share of the root's cross-section x its years,
    /// its own turn only (the base erects, the distal part keeps its lean).
    fn finish(mut self, mut st: Stats) -> Grown {
        let n = self.pos.len();
        let t0 = Instant::now();
        let mut tips = vec![0.0f64; n];
        let mut kids = vec![false; n];
        for i in (1..n).rev() {
            let p = self.parent[i] as usize;
            kids[p] = true;
            tips[i] += if kids[i] { 0.0 } else { 1.0 };
            tips[p] += tips[i];
        }
        let e = if self.p.exponent > 0.0 { self.p.exponent } else { self.w.exponent };
        let k = self.w.root_radius / tips[0].max(1.0).powf(1.0 / e);
        let radius: Vec<f64> = tips.iter().map(|x| k * x.max(1.0).powf(1.0 / e)).collect();
        st.ms[1] = ms(t0);
        let t0 = Instant::now();
        if self.p.straighten > 0.0 {
            let end = self.p.cycles.round();
            for i in 1..n {
                let p = self.parent[i] as usize;
                let share = (radius[i] / radius[0]).powi(2);
                let w = self.dir[i];
                let axis = w.cross(Vec3::Y);
                let gap = w.dot(Vec3::Y).clamp(-1.0, 1.0).acos();
                let a = (self.p.straighten * share * (end - f64::from(self.born[i]))).min(gap);
                let d = if axis.length() > 1e-9 && a > 0.0 { Quat::axis_angle(axis.normalized(), a).apply(w) } else { w };
                self.pos[i] = self.pos[p] + d * self.len[i];
            }
        }
        st.ms[2] = ms(t0);
        Grown {
            parent: self.parent.iter().map(|&x| (x != u32::MAX).then_some(x as usize)).collect(),
            lateral: self.lateral,
            radius,
            elev0: self.elev0.iter().map(|&x| f64::from(x)).collect(),
            pos: self.pos,
            stats: st,
        }
    }
}

#[derive(Clone, Copy)]
struct Quat {
    w: f64,
    v: Vec3,
}

impl Quat {
    fn axis_angle(axis: Vec3, a: f64) -> Self {
        Self { w: (a / 2.0).cos(), v: axis * (a / 2.0).sin() }
    }
    fn apply(self, x: Vec3) -> Vec3 {
        let t = self.v.cross(x) * 2.0;
        x + t * self.w + self.v.cross(t)
    }
}
