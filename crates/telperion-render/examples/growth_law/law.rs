//! One growth law from trunk to twig (fn-190 R1 probe, round 2). Every bud
//! reads its light from the shadow-propagation grid and the tree shares it by
//! Borchert-Honda allocation (Pałubicki et al. 2009); every bud carries a continuous
//! physiological age phi (Barthélémy & Caraglio 2007) that sets its shoot's
//! length, its laterals and their place along it, its lean, and its fate.
//! Vigour moves phi: a starved axis ages, a vigorous one stays young or
//! rejuvenates (reiteration). Thick wood turns toward up over time
//! (straightening, Troll), and branches whose remembered resource runs out
//! are shed. Integer outcomes are taken in expectation with keyed
//! randomness, so a small change of any setting is a small change of the tree.
use crate::params::Params;
use crate::shadow::Shadow;
use std::time::Instant;
use telperion_core::envelope::Envelope;
use telperion_core::math::Vec3;

const NONE: u32 = u32::MAX;

#[derive(Clone)]
struct Node {
    parent: u32,
    pos: Vec3,
    dir: Vec3,
    len: f64,
    lateral: bool,
    alive: bool,
    /// Birth key: stable through compaction, seeds every keyed draw.
    key: u64,
    born: u32,
    /// The axis's horizontal outward direction (zero on the stem).
    out: Vec3,
    /// Pipes of shed wood (Pałubicki 4.5) and remembered resource per tip.
    memory: f64,
    qmem: f64,
    /// A lateral's elevation at birth, degrees (NaN elsewhere).
    elev0: f32,
}

#[derive(Clone, Copy)]
struct Bud {
    node: u32,
    /// Cycles since the bud last grew (the bud bank's age).
    dormant: u16,
    dir: Vec3,
    phi: f64,
    terminal: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Stats {
    pub cycles: usize,
    pub shed: usize,
    pub buds_max: usize,
    pub capped: bool,
    pub ms: [f64; 5],
    /// Of extend: the time spent casting shadows, and the voxels updated.
    pub cast_ms: f64,
    pub updates: u64,
    /// The tree's net balance each cycle.
    pub net: Vec<f64>,
}
pub const STAGES: [&str; 5] = ["light", "allocate", "shed", "extend", "straighten"];

/// The grown tree, parent before child.
pub struct Grown {
    pub pos: Vec<Vec3>,
    pub parent: Vec<Option<usize>>,
    pub lateral: Vec<bool>,
    pub radius: Vec<f64>,
    pub elev0: Vec<f64>,
    pub stats: Stats,
}

/// A uniform number in [0, 1) from a key and a salt.
fn hash(key: u64, salt: u64) -> f64 {
    let mut z = key.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ salt.wrapping_mul(0xD1B5_4A32_D192_ED03);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 11) as f64 / (1u64 << 53) as f64
}

/// `x` rounded to an integer in expectation: floor(x) plus one with the
/// probability of its fraction.
fn keyed_round(x: f64, key: u64, salt: u64) -> usize {
    let f = x.max(0.0);
    f.floor() as usize + usize::from(hash(key, salt) < f.fract())
}

pub struct World {
    pub shadow: Shadow,
    /// The authored crown, pulled toward with weight `envelope` (0 neutral).
    pub envelope: Envelope,
    pub height: f64,
    pub root_radius: f64,
    pub exponent: f64,
    pub seed: u64,
}

struct Grower {
    p: Params,
    w: World,
    nodes: Vec<Node>,
    buds: Vec<Bud>,
    next_key: u64,
    unit: f64,
    cast_ms: f64,
}

pub fn grow(p: &Params, w: World) -> Grown {
    let unit = p.unit * w.height;
    let mut g = Grower { p: *p, nodes: vec![], buds: vec![], next_key: 1, unit, cast_ms: 0.0, w };
    g.nodes.push(Node {
        parent: NONE,
        pos: Vec3::ZERO,
        dir: Vec3::Y,
        len: 0.0,
        lateral: false,
        alive: true,
        key: g.w.seed.wrapping_mul(1_000_003),
        born: 0,
        out: Vec3::ZERO,
        memory: 0.0,
        qmem: 0.0,
        elev0: f32::NAN,
    });
    g.buds.push(Bud { node: 0, dormant: 0, dir: Vec3::Y, phi: 0.0, terminal: true });
    let mut st = Stats::default();
    for t in 0..p.cycles.round() as usize {
        st.cycles += 1;
        let t0 = Instant::now();
        let (q, guide) = g.perceive();
        st.ms[0] += ms(t0);
        let t0 = Instant::now();
        let n = g.nodes.len();
        let (qm, ql, cnt, lit, upk) = g.basipetal(&q);
        let v = g.allocate(&q, &qm, &ql, lit[0] - upk[0]);
        st.net.push(lit[0] - upk[0]);
        st.ms[1] += ms(t0);
        let t0 = Instant::now();
        st.shed += g.shed(t, &lit, &upk, &cnt);
        st.ms[2] += ms(t0);
        let t0 = Instant::now();
        let buds = std::mem::take(&mut g.buds);
        let mut kept = Vec::with_capacity(buds.len());
        for (b, (&vb, &gd)) in buds.into_iter().zip(v.iter().zip(&guide)) {
            if !g.nodes[b.node as usize].alive {
                continue;
            }
            if g.nodes.len() >= p.max_nodes as usize {
                st.capped = true;
                kept.push(b);
                continue;
            }
            let before = g.nodes.len();
            if let Some(mut rest) = g.extend(b, vb, gd, t) {
                if g.nodes.len() == before {
                    // Dormant: the bud bank decays, a little more each year.
                    rest.dormant = rest.dormant.saturating_add(1);
                    let die = p.bud_death * f64::from(rest.dormant);
                    if hash(g.nodes[rest.node as usize].key ^ u64::from(rest.dormant), 13 + t as u64) < die {
                        continue;
                    }
                }
                kept.push(rest);
            }
        }
        kept.append(&mut g.buds);
        g.buds = kept;
        st.buds_max = st.buds_max.max(g.buds.len());
        st.ms[3] += ms(t0);
        let t0 = Instant::now();
        if p.straighten > 0.0 && g.nodes.len() > n.min(1) {
            g.straighten();
        }
        st.ms[4] += ms(t0);
        if g.nodes.iter().filter(|x| !x.alive).count() * 4 > g.nodes.len() {
            g.compact();
        }
    }
    g.compact();
    st.cast_ms = g.cast_ms;
    st.updates = g.w.shadow.updates;
    g.finish(st)
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}

impl Grower {
    /// Each bud's light (one grid lookup) and the direction toward light.
    fn perceive(&self) -> (Vec<f64>, Vec<Vec3>) {
        let sh = &self.w.shadow;
        self.buds
            .iter()
            .map(|b| {
                let at = self.nodes[b.node as usize].pos;
                ((1.0 - sh.at(at) + self.p.shade).clamp(0.0, 1.0), sh.descent(at))
            })
            .unzip()
    }

    /// Resource gathered along the main line (qm) and the laterals (ql) of
    /// every node, and its tips (with the pipes of shed wood).
    /// Per node: the demand-weighted light of the main line (qm) and the
    /// laterals (ql) for allocation, the tips with shed pipes (cnt), and the
    /// subtree's light gathered (lit) and wood upkeep (upk): its net is
    /// lit - upk. A metamer one unit long carrying one tip's pipe costs
    /// the upkeep setting; cost grows with its volume.
    #[allow(clippy::type_complexity)]
    fn basipetal(&self, q: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let n = self.nodes.len();
        let z = || vec![0.0; n];
        let (mut qm, mut ql, mut cnt, mut lit, mut upk) = (z(), z(), z(), z(), z());
        for (b, &x) in self.buds.iter().zip(q) {
            let w = x * self.shoot_len(b.phi);
            if b.terminal { qm[b.node as usize] += w } else { ql[b.node as usize] += w }
            // Leaves are on the shoots that grew: a dormant bud gathers nothing.
            if b.dormant == 0 {
                lit[b.node as usize] += x;
            }
        }
        let e = if self.w.exponent > 0.0 { self.w.exponent } else { 2.5 };
        let mut kids = vec![false; n];
        for i in (1..n).rev() {
            if !self.nodes[i].alive {
                continue;
            }
            let p = self.nodes[i].parent as usize;
            kids[p] = true;
            let sub = qm[i] + ql[i];
            if self.nodes[i].lateral { ql[p] += sub } else { qm[p] += sub }
            cnt[i] += self.nodes[i].memory + if kids[i] { 0.0 } else { 1.0 };
            cnt[p] += cnt[i];
            upk[i] += self.p.upkeep * self.nodes[i].len / self.unit * cnt[i].powf(2.0 / e);
            upk[p] += upk[i];
            lit[p] += lit[i];
        }
        cnt[0] += if kids[0] { 0.0 } else { 1.0 };
        (qm, ql, cnt, lit, upk)
    }

    /// A shoot's internode as a share of a young shoot's, by age.
    fn shoot_len(&self, phi: f64) -> f64 {
        1.0 - (1.0 - self.p.short) * phi
    }

    /// Borchert-Honda: the resource enters at the base and splits at every
    /// node, lambda to the main line and 1 - lambda to the laterals, each
    /// in proportion to what it gathered. Returns each bud's vigour.
    fn allocate(&self, q: &[f64], qm: &[f64], ql: &[f64], net: f64) -> Vec<f64> {
        let (l, n) = (self.p.lambda, self.nodes.len());
        let den = |i: usize| l * qm[i] + (1.0 - l) * ql[i];
        let mut vin = vec![0.0; n];
        vin[0] = self.p.alpha * net.max(0.0);
        for i in 1..n {
            let nd = &self.nodes[i];
            if !nd.alive {
                continue;
            }
            let p = nd.parent as usize;
            let d = den(p);
            if d > 0.0 {
                let wt = if nd.lateral { 1.0 - l } else { l };
                vin[i] = vin[p] * wt * (qm[i] + ql[i]) / d;
            }
        }
        self.buds
            .iter()
            .zip(q)
            .map(|(b, &x)| {
                let i = b.node as usize;
                let d = den(i);
                let wt = if b.terminal { l } else { 1.0 - l };
                if d > 0.0 { vin[i] * wt * x * self.shoot_len(b.phi) / d } else { 0.0 }
            })
            .collect()
    }

    /// Remembers every branch's relative net balance, (light - upkeep) /
    /// (light + upkeep), and sheds it with a chance that rises smoothly from
    /// 0 as the remembered balance falls past -tolerance to -(tolerance +
    /// shedWidth), times the shed rate; its pipes stay in the parent. No
    /// branch is spared: the main line's balance is the crown's.
    fn shed(&mut self, t: usize, lit: &[f64], upk: &[f64], cnt: &[f64]) -> usize {
        let p = self.p;
        let mut count = 0;
        for i in 1..self.nodes.len() {
            let par = self.nodes[i].parent as usize;
            if !self.nodes[i].alive {
                continue;
            }
            if !self.nodes[par].alive {
                self.nodes[i].alive = false;
                continue;
            }
            let bal = (lit[i] - upk[i]) / (lit[i] + upk[i]).max(1e-12);
            let nd = &mut self.nodes[i];
            nd.qmem = (1.0 - p.memory) * nd.qmem + p.memory * bal;
            if p.shed <= 0.0 || (t as f64 - f64::from(nd.born)) < p.shed_age {
                continue;
            }
            let x = ((-nd.qmem - p.tolerance) / p.shed_width.max(1e-9)).clamp(0.0, 1.0);
            let prob = p.shed * x * x * (3.0 - 2.0 * x);
            if hash(nd.key, 17 + t as u64) < prob {
                nd.alive = false;
                self.nodes[par].memory += cnt[i];
                count += 1;
            }
        }
        count
    }

    fn lean(&self, phi: f64) -> f64 {
        (self.p.lean0 + self.p.lean1 * phi).clamp(0.0, 1.0)
    }

    /// Grows one bud's shoot: keyed-round(v) metamers whose length falls with
    /// age; laterals set along it by the position profile and rhythm, fewer
    /// with age; the terminal bud persists by keyed chance. Returns the
    /// shoot's terminal bud, or the bud itself when it did not grow.
    fn extend(&mut self, b: Bud, v: f64, guide: Vec3, t: usize) -> Option<Bud> {
        let p = self.p;
        let base = &self.nodes[b.node as usize];
        let (bkey, bphi) = (base.key ^ if b.terminal { 0 } else { hash(b.dir.x.to_bits(), 7).to_bits() }, b.phi);
        let ratio = v / p.v_ref;
        let phi = (bphi + p.drift * (1.0 - ratio.min(1.0)) - p.reiteration * (ratio - 1.0).max(0.0)).clamp(0.0, 1.0);
        // v buys v units of wood, in internodes as long as the age allows.
        let n = keyed_round(v / self.shoot_len(phi), bkey, 11 + t as u64);
        if n == 0 {
            return Some(b);
        }
        let len = self.unit * v / n as f64;
        let out = if b.terminal {
            base.out
        } else {
            let f = Vec3::new(b.dir.x, 0.0, b.dir.z);
            if f.length() > 1e-9 { f.normalized() } else { Vec3::ZERO }
        };
        let lean = self.lean(phi);
        let trop = if out.length() > 0.0 { (Vec3::Y * (1.0 - lean) + out * lean).normalized() } else { Vec3::Y };
        // Lateral weights along the shoot: the position profile, then the
        // rhythm's share set at the distal node; mean 1 over the shoot.
        let prof: Vec<f64> = (0..n).map(|k| (p.acrotony * 3.0 * ((k as f64 + 1.0) / n as f64 - 0.5)).exp()).collect();
        let mean = prof.iter().sum::<f64>() / n as f64;
        // Fate follows vigour and age: a weak or old shoot is short and bears
        // few laterals (a short shoot), a vigorous young one is long and branches.
        let dev = p.branching * (1.0 - phi).powf(p.fate) * (v / p.v_ref).min(1.0).powf(p.vigour_fate);
        let phi_l = (phi + p.phi_step).clamp(0.0, 1.0);
        let angle = (p.angle0 + (p.angle1 - p.angle0) * phi_l).to_radians();
        let mut d = b.dir;
        let mut at = b.node;
        for k in 0..n {
            let here = self.nodes[at as usize].pos;
            d = (d + guide * p.xi + trop * p.eta + self.inward(here) * p.envelope).normalized();
            let pos = self.nodes[at as usize].pos + d * len;
            let key = self.next_key;
            self.next_key += 1;
            let idx = self.nodes.len() as u32;
            self.nodes.push(Node {
                parent: at,
                pos,
                dir: d,
                len,
                lateral: !b.terminal && k == 0,
                alive: true,
                key: key ^ self.w.seed.rotate_left(17),
                born: t as u32,
                out,
                memory: 0.0,
                qmem: 0.0,
                elev0: if !b.terminal && k == 0 { d.y.clamp(-1.0, 1.0).asin().to_degrees() as f32 } else { f32::NAN },
            });
            let tc = Instant::now();
            self.w.shadow.cast(pos, 1.0);
            self.cast_ms += ms(tc);
            let wk = (1.0 - p.rhythm) * prof[k] / mean + p.rhythm * if k + 1 == n { n as f64 } else { 0.0 };
            let nk = self.nodes[idx as usize].key;
            let laterals = keyed_round(dev * wk, nk, 3);
            for j in 0..laterals {
                let dir = self.bud_dir(d, nk, j, laterals, k, angle);
                self.buds.push(Bud { node: idx, dormant: 0, dir, phi: phi_l, terminal: false });
            }
            at = idx;
        }
        let tk = self.nodes[at as usize].key;
        (hash(tk, 5) < p.persistence).then_some(Bud { node: at, dormant: 0, dir: d, phi, terminal: true })
    }

    /// Toward the authored crown from outside it: inward and down, growing
    /// with the distance outside; zero inside.
    fn inward(&self, q: Vec3) -> Vec3 {
        if self.p.envelope == 0.0 {
            return Vec3::ZERO;
        }
        let e = &self.w.envelope;
        let r = q.x.hypot(q.z);
        let out = (r - e.radius_toward(q, self.w.seed as u32)).max(0.0) + (q.y - e.height).max(0.0);
        if out <= 0.0 || r < 1e-9 {
            return Vec3::ZERO;
        }
        let toward = Vec3::new(-q.x / r, if q.y > e.height { -1.0 } else { 0.0 }, -q.z / r).normalized();
        toward * (out / self.unit).min(1.0)
    }

    /// A lateral bud's direction: `angle` off the shoot, its bearing the
    /// golden angle by birth key blended toward two ranks about the shoot's
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

    /// Thick wood turns toward up by `straighten` times its share of the
    /// root's pipe radius each cycle; the subtree turns with it.
    fn straighten(&mut self) {
        let n = self.nodes.len();
        let mut tips = vec![0.0f64; n];
        let mut kids = vec![false; n];
        for i in (1..n).rev() {
            if !self.nodes[i].alive {
                continue;
            }
            let p = self.nodes[i].parent as usize;
            kids[p] = true;
            tips[i] += if kids[i] { 0.0 } else { 1.0 };
            tips[p] += tips[i];
        }
        let e = if self.w.exponent > 0.0 { self.w.exponent } else { 2.5 };
        let root = tips[0].max(1.0);
        let mut rot: Vec<Quat> = vec![Quat::ID; n];
        for i in 1..n {
            if !self.nodes[i].alive {
                continue;
            }
            let p = self.nodes[i].parent as usize;
            let share = (tips[i] / root).powf(1.0 / e);
            let world = rot[p].apply(self.nodes[i].dir);
            let axis = world.cross(Vec3::Y);
            let gap = world.dot(Vec3::Y).clamp(-1.0, 1.0).acos();
            let a = (self.p.straighten * share).min(gap);
            let own = if axis.length() > 1e-9 && a > 0.0 { Quat::axis_angle(axis.normalized(), a) } else { Quat::ID };
            rot[i] = own.mul(rot[p]);
            let nd = rot[i].apply(self.nodes[i].dir).normalized();
            self.nodes[i].dir = nd;
            self.nodes[i].pos = self.nodes[p].pos + nd * self.nodes[i].len;
        }
        for b in &mut self.buds {
            b.dir = rot[b.node as usize].apply(b.dir).normalized();
        }
    }

    fn compact(&mut self) {
        let mut map = vec![NONE; self.nodes.len()];
        let mut out = Vec::with_capacity(self.nodes.len());
        for (i, nd) in self.nodes.iter().enumerate() {
            let alive = nd.alive && (nd.parent == NONE || map[nd.parent as usize] != NONE);
            if alive {
                map[i] = out.len() as u32;
                out.push(nd.clone());
            }
        }
        for nd in &mut out {
            if nd.parent != NONE {
                nd.parent = map[nd.parent as usize];
            }
        }
        self.buds.retain_mut(|b| {
            b.node = map[b.node as usize];
            b.node != NONE
        });
        self.nodes = out;
    }

    /// Pipe-model radii: tips plus the pipes of shed wood, the root at its
    /// given radius.
    fn finish(self, stats: Stats) -> Grown {
        let n = self.nodes.len();
        let mut d = vec![0.0; n];
        let mut kids = vec![false; n];
        for i in (1..n).rev() {
            let p = self.nodes[i].parent as usize;
            kids[p] = true;
            d[i] += self.nodes[i].memory + if kids[i] { 0.0 } else { 1.0 };
            d[p] += d[i];
        }
        let e = if self.p.exponent > 0.0 { self.p.exponent } else { self.w.exponent };
        let k = self.w.root_radius / d[0].max(1.0).powf(1.0 / e);
        Grown {
            pos: self.nodes.iter().map(|x| x.pos).collect(),
            parent: self.nodes.iter().map(|x| (x.parent != NONE).then_some(x.parent as usize)).collect(),
            lateral: self.nodes.iter().map(|x| x.lateral).collect(),
            radius: d.iter().map(|x| k * x.max(1.0).powf(1.0 / e)).collect(),
            elev0: self.nodes.iter().map(|x| f64::from(x.elev0)).collect(),
            stats,
        }
    }
}

#[derive(Clone, Copy)]
struct Quat {
    w: f64,
    v: Vec3,
}

impl Quat {
    const ID: Self = Self { w: 1.0, v: Vec3::ZERO };
    fn axis_angle(axis: Vec3, a: f64) -> Self {
        Self { w: (a / 2.0).cos(), v: axis * (a / 2.0).sin() }
    }
    /// self after other: (self * other).
    fn mul(self, o: Self) -> Self {
        Self { w: self.w * o.w - self.v.dot(o.v), v: o.v * self.w + self.v * o.w + self.v.cross(o.v) }
    }
    fn apply(self, x: Vec3) -> Vec3 {
        let t = self.v.cross(x) * 2.0;
        x + t * self.w + self.v.cross(t)
    }
}
