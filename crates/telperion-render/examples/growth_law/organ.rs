//! fn-190 probe (scratch): one organogenesis rule from trunk to twig, driven
//! by a continuous physiological age phi (Barthélémy & Caraglio 2007), with
//! stage 1's axis rule: Troll's model as Millet et al. 1998 describe it in
//! *Fagus*. Every axis is a module whose tip tilts toward its plagiotropic
//! heading as it ages; the wood before its curvature zone is its erect base
//! and straightens; a bud in the curvature zone, on the upper side, relays
//! the module and takes its dominance, and the head becomes a branch; near
//! maturity a relay event forks. No light, no allocation, no envelope: the
//! yearly bud sweep, one pipe count, one straightening, one prune. Integer
//! outcomes are taken in expectation with keyed randomness.
use crate::params::Params;
use std::collections::HashSet;
use std::time::Instant;
use telperion_core::math::Vec3;

mod module;

const NONE: u32 = u32::MAX;

#[derive(Clone, Copy)]
struct Bud {
    node: u32,
    dir: Vec3,
    /// The axis's horizontal heading: where its plagiotropic tip leans.
    out: Vec3,
    /// The axis's phi at birth (drift adds its years).
    phi: f64,
    /// Years this axis (this module) has grown: drift and tilt.
    years: u16,
    /// The next node starts a new axis; `cont` keeps it on the parent's
    /// line (the relay that continues the stem).
    fresh: bool,
    cont: bool,
    /// The axis's death record (the head's is replaced at its relay).
    axis: u32,
    /// The curvature zone: the module's last erect node and the next one.
    bend: u32,
    bend_next: u32,
    relayed: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub cycles: usize,
    pub buds_max: usize,
    /// Cold-run ms of the passes.
    pub ms: [f64; 4],
    pub laterals: usize,
    /// Distinct (phi, birth year) keys among laterals (factorisation probe).
    pub distinct: usize,
    pub distinct_phi: usize,
    /// Nodes pruned by lifespan.
    pub pruned: usize,
    /// Relay events, and those that forked.
    pub relays: usize,
    pub forks: usize,
}
pub const STAGES: [&str; 4] = ["sweep", "pipe", "straighten", "prune"];

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

fn elev(d: Vec3) -> f64 {
    d.y.clamp(-1.0, 1.0).asin().to_degrees()
}

fn horizontal(d: Vec3) -> Vec3 {
    let f = Vec3::new(d.x, 0.0, d.z);
    if f.length() > 1e-9 { f.normalized() } else { Vec3::ZERO }
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
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
    /// The axis's phi at birth (its straightening rate), whether the node
    /// is a module's erect base (only that straightens), its death record.
    phi: Vec<f32>,
    base: Vec<bool>,
    axis: Vec<u32>,
    /// The year each death record falls due (a head's is set at its relay).
    die: Vec<f32>,
    /// The node each record's axis was born on (it dies with that node).
    up: Vec<u32>,
    buds: Vec<Bud>,
    keys: Option<Vec<(u64, u16)>>,
    relays: usize,
    forks: usize,
}

/// Grows the tree; `diag` also records the factorisation probe (untimed
/// runs only). A tree past `maxNodes` is an error.
pub fn grow(p: &Params, w: World, diag: bool) -> Result<Grown, String> {
    let unit = p.unit * w.height;
    let cap = p.max_nodes as usize;
    let a = hash(w.seed, 77) * std::f64::consts::TAU;
    let out = Vec3::new(a.cos(), 0.0, a.sin());
    let seed = Bud { node: 0, dir: Vec3::Y, out, phi: 0.0, years: 0, fresh: false, cont: false, axis: 0, bend: NONE, bend_next: NONE, relayed: false };
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
        phi: vec![0.0],
        base: vec![true],
        axis: vec![0],
        die: vec![f32::INFINITY],
        up: vec![NONE],
        buds: vec![seed],
        keys: diag.then(Vec::new),
        relays: 0,
        forks: 0,
    };
    let mut st = Stats::default();
    let t0 = Instant::now();
    for t in 0..p.cycles.round() as usize {
        st.cycles += 1;
        let mut memo = vec![0u8; g.die.len()];
        let mut chain = vec![];
        for b in std::mem::take(&mut g.buds) {
            if !g.dead(b.axis, t as f32, &mut memo, &mut chain) {
                g.unit_of(b, t as u16);
            }
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
    st.relays = g.relays;
    st.forks = g.forks;
    Ok(g.finish(st))
}

impl Grower {
    fn key(&self, i: usize) -> u64 {
        (i as u64).wrapping_mul(0x2545_F491_4F6C_DD1D) ^ self.w.seed.rotate_left(17)
    }

    fn push(&mut self, at: u32, d: Vec3, len: f64, t: u16, b: &Bud, lateral: bool) -> u32 {
        let idx = self.pos.len() as u32;
        self.pos.push(self.pos[at as usize] + d * len);
        self.dir.push(d);
        self.len.push(len);
        self.parent.push(at);
        self.lateral.push(lateral);
        self.born.push(t);
        self.elev0.push(if lateral { elev(d) as f32 } else { f32::NAN });
        self.phi.push(b.phi as f32);
        self.base.push(b.bend == NONE && elev(d) >= self.p.bend_elev);
        self.axis.push(b.axis);
        idx
    }

    /// One bud's yearly growth unit.
    fn unit_of(&mut self, mut b: Bud, t: u16) {
        let p = self.p;
        let bkey = self.key(b.node as usize) ^ if b.fresh { b.dir.x.to_bits() } else { 0 };
        if b.bend != NONE && !b.relayed && hash(bkey, 41 + u64::from(t)) < p.relay * (1.0 - b.phi).powf(p.relay_shape) {
            self.relay(&mut b, bkey, t);
        }
        // phi moves the fraction `drift` of the way to 1 each year it grows.
        let phi = 1.0 - (1.0 - b.phi) * (1.0 - p.drift).powi(i32::from(b.years));
        // Vigour: the axis's own (falling with phi) times the tree's
        // establishment curve; it sets the unit's metamers.
        let aged = phi.powf(p.vigour_shape);
        let vigour = lerp(1.0, p.n1 / p.n0, aged) * self.establishment(t);
        let n = keyed_round(p.n0 * vigour, bkey, 11 + u64::from(t));
        if n == 0 {
            self.buds.push(Bud { years: b.years + 1, ..b });
            return;
        }
        let len = self.unit * lerp(1.0, p.short, aged);
        if b.out.length() == 0.0 {
            b.out = horizontal(b.dir);
        }
        // Lean by phi, plus the module's tilt as it ages (Troll's tip).
        let tilt = p.tilt * (1.0 - (-f64::from(b.years) / p.tilt_years.max(1e-9)).exp());
        let lean = (p.lean0 + p.lean1 * phi + tilt).clamp(0.0, p.plagio);
        let trop = if b.out.length() > 0.0 { (Vec3::Y * (1.0 - lean) + b.out * lean).normalized() } else { Vec3::Y };
        let dev = p.branching * (1.0 - phi).powf(p.fate);
        let mean = (0..n).map(|k| self.profile(k, n)).sum::<f64>() / n as f64;
        let mut d = b.dir;
        let mut at = b.node;
        for k in 0..n {
            // Gravitropism near the base: a pull up that is 1 at the ground
            // and falls as exp(-height / (ground x internode)); 0 is off.
            let y = self.pos[at as usize].y;
            let lift = if p.ground > 0.0 { (-y.max(0.0) / (p.ground * len)).exp() } else { 0.0 };
            d = (d + trop * p.eta + Vec3::Y * lift).normalized();
            let prev = at;
            at = self.push(at, d, len, t, &b, b.fresh && !b.cont);
            // The curvature zone: the last erect node of a module that had
            // an erect base (a branch born plagiotropic has none).
            if b.bend == NONE && !b.fresh && elev(d) < p.bend_elev {
                b.bend = prev;
                b.bend_next = at;
            }
            b.fresh = false;
            self.laterals(at, d, k, n, phi, vigour, mean, dev, t);
        }
        self.buds.push(Bud { node: at, dir: d, years: b.years + 1, ..b });
    }

    /// The laterals of metamer `k` of `n`: how many by the position profile
    /// and phi, their birth phi by place and vigour, angle, lifespan.
    #[allow(clippy::too_many_arguments)]
    fn laterals(&mut self, at: u32, d: Vec3, k: usize, n: usize, phi: f64, vigour: f64, mean: f64, dev: f64, t: u16) {
        let p = self.p;
        let wk = (1.0 - p.rhythm) * self.profile(k, n) / mean + p.rhythm * if k + 1 == n { n as f64 } else { 0.0 };
        let nk = self.key(at as usize);
        let count = keyed_round(dev * wk, nk, 3);
        if count == 0 {
            return;
        }
        let u = (k + 1) as f64 / n as f64;
        // The birth jump, by place along the unit and the shoot's lack of
        // vigour, moves phi toward 1 by the fraction 1 - exp(-jump).
        let jump = p.phi_step + p.zone * (1.0 - u) + p.vigour_jump * (1.0 - vigour).max(0.0);
        let phi_l = 1.0 - (1.0 - phi) * (-jump).exp();
        // Near maturity the distal laterals of a vigorous axis reiterate it
        // ([M98]: the upper branches "reproduce the structure of the young
        // tree"): their phi falls back toward the carrier's.
        let phi_l = lerp(phi_l, phi, p.reiterate * self.maturity(t) * u * vigour.min(1.0));
        let angle = lerp(p.angle0, p.angle1, phi_l).to_radians();
        // Lifespan falls with phi; an axis dies with the one carrying it.
        let life = self.life(phi_l);
        for j in 0..count {
            let dir = self.bud_dir(d, nk, j, count, k, angle);
            let dies = f64::from(t) + 1.0 + keyed_round(life, nk ^ j as u64, 29) as f64;
            let axis = self.record(dies, at);
            self.buds.push(Bud { node: at, dir, out: Vec3::ZERO, phi: phi_l, years: 0, fresh: true, cont: false, axis, bend: NONE, bend_next: NONE, relayed: false });
            if let Some(keys) = &mut self.keys {
                keys.push((phi_l.to_bits(), t));
            }
        }
    }


    /// The tree's establishment curve: vigour est0 at age 0, rising
    /// smoothly toward 1 with time constant estYears (est0 1 is flat).
    fn establishment(&self, t: u16) -> f64 {
        let p = self.p;
        if p.est_years <= 0.0 {
            return 1.0;
        }
        1.0 - (1.0 - p.est0) * (-f64::from(t) / p.est_years).exp()
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
        // Two ranks: the rotation axis is the shoot's own up, so laterals
        // swing to its horizontal sides.
        let side = d.cross(Vec3::Y);
        let two = if side.length() > 1e-9 {
            let s = side.cross(d).normalized();
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

    /// A segment's direction after `years` of straightening: an erect base
    /// turns toward up at straighten x (1 - phi at birth) a year, stopping
    /// at vertical; other wood keeps its direction.
    fn turned(&self, i: usize, years: f64) -> Vec3 {
        let w = self.dir[i];
        if !self.base[i] {
            return w;
        }
        let axis = w.cross(Vec3::Y);
        let gap = w.dot(Vec3::Y).clamp(-1.0, 1.0).acos();
        let young = 1.0 - f64::from(self.phi[i]);
        let a = (self.p.straighten * young * years).min(gap * young.powf(self.p.straighten_shape));
        if axis.length() > 1e-9 && a > 0.0 { Quat::axis_angle(axis.normalized(), a).apply(w) } else { w }
    }

    /// Pipe counts over all wood, the pruned included (their pipes stay),
    /// radii, straightening, then the pruned axes dropped.
    fn finish(mut self, mut st: Stats) -> Grown {
        let n = self.pos.len();
        let end = self.p.cycles.round();
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
        let k = if self.p.tip_radius > 0.0 { self.p.tip_radius } else { self.w.root_radius / tips[0].max(1.0).powf(1.0 / e) };
        st.ms[1] = ms(t0);
        let t0 = Instant::now();
        if self.p.straighten > 0.0 {
            for i in 1..n {
                let p = self.parent[i] as usize;
                let d = self.turned(i, end - f64::from(self.born[i]));
                self.pos[i] = self.pos[p] + d * self.len[i];
            }
        }
        st.ms[2] = ms(t0);
        let t0 = Instant::now();
        // A node lives while its own record and every carrier's live.
        let mut eff = vec![f32::INFINITY; n];
        let mut map = vec![u32::MAX; n];
        let mut g = Grown { pos: vec![], parent: vec![], lateral: vec![], radius: vec![], elev0: vec![], stats: Stats::default() };
        for i in 0..n {
            let own = self.die[self.axis[i] as usize];
            eff[i] = if i == 0 { own } else { own.min(eff[self.parent[i] as usize]) };
            if f64::from(eff[i]) <= end {
                st.pruned += 1;
                continue;
            }
            map[i] = g.pos.len() as u32;
            g.pos.push(self.pos[i]);
            g.parent.push((i > 0).then(|| map[self.parent[i] as usize] as usize));
            g.lateral.push(self.lateral[i]);
            g.radius.push(k * tips[i].max(1.0).powf(1.0 / e));
            g.elev0.push(f64::from(self.elev0[i]));
        }
        st.ms[3] = ms(t0);
        g.stats = st;
        g
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
