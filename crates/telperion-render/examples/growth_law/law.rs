//! One growth law from trunk to twig (fn-190 R1 probe, round 2). Every bud
//! reads its light from above (Beer–Lambert over the full height) and the tree shares it by
//! Borchert-Honda allocation (Pałubicki et al. 2009); every bud carries a continuous
//! physiological age phi (Barthélémy & Caraglio 2007) that sets its shoot's
//! length, its laterals and their place along it, its lean, and its fate.
//! Vigour moves phi: a starved axis ages, a vigorous one stays young or
//! rejuvenates (reiteration). Thick wood turns toward up over time
//! (straightening, Troll), and branches whose remembered resource runs out
//! are shed. Integer outcomes are taken in expectation with keyed
//! randomness, so a small change of any setting is a small change of the tree.
use crate::params::Params;
use crate::light::Light;
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
    /// Path length from the root (hydraulic limit).
    path: f64,
    /// The short shoots this long shoot carries, as one record: their
    /// count, total length, and remembered balance. Geometry is emitted at
    /// the end.
    rec_n: f64,
    rec_len: f64,
    rec_mem: f64,
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
    /// The light field's downward passes: time and cells visited.
    pub cast_ms: f64,
    pub updates: u64,
    /// The tree's net balance each cycle.
    pub net: Vec<f64>,
    pub reserve_end: f64,
    pub records: usize,
    pub short_shoots: f64,
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
    pub light: Light,
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
    /// The stored resource: the seed reserve, topped up from surplus.
    reserve: f64,
}

pub fn grow(p: &Params, w: World) -> Grown {
    let unit = p.unit * w.height;
    let mut g = Grower { p: *p, nodes: vec![], buds: vec![], next_key: 1, unit, cast_ms: 0.0, reserve: p.reserve, w };
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
        path: 0.0,
        rec_n: 0.0,
        rec_len: 0.0,
        rec_mem: 0.0,
    });
    g.buds.push(Bud { node: 0, dormant: 0, dir: Vec3::Y, phi: 0.0, terminal: true });
    let mut st = Stats::default();
    for t in 0..p.cycles.round() as usize {
        st.cycles += 1;
        let tc = Instant::now();
        g.relight();
        g.cast_ms += ms(tc);
        let t0 = Instant::now();
        let (q, guide) = g.perceive();
        st.ms[0] += ms(t0);
        let t0 = Instant::now();
        let n = g.nodes.len();
        let (qm, ql, cnt, lit, upk, wr) = g.basipetal(&q);
        // The reserve: this year's net plus the stock is what the tree has;
        // it covers upkeep first (the share `cover` of every branch's
        // upkeep it pays), a share is stored again, the rest is spent.
        let net = lit[0] - upk[0];
        let cover = (g.reserve / upk[0].max(1e-12)).min(1.0);
        let have = (net + g.reserve).max(0.0);
        g.reserve = p.store * have;
        let (v, vr) = g.allocate(&q, &qm, &ql, &wr, (1.0 - p.store) * have);
        g.grow_records(&vr);
        st.net.push(net);
        st.ms[1] += ms(t0);
        let t0 = Instant::now();
        st.shed += g.shed(t, &lit, &upk, &cnt, cover);
        g.shed_records(t, cover);
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
    st.records = g.nodes.iter().filter(|n| n.rec_n > 0.0).count();
    st.short_shoots = g.nodes.iter().map(|n| n.rec_n).sum();
    g.emit_records();
    st.cast_ms = g.cast_ms;
    st.updates = g.w.light.updates;
    st.reserve_end = g.reserve;
    g.finish(st)
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}

impl Grower {
    /// Each bud's light (one grid lookup) and the direction toward light.
    fn perceive(&self) -> (Vec<f64>, Vec<Vec3>) {
        let lt = &self.w.light;
        self.buds
            .iter()
            .map(|b| {
                let at = self.nodes[b.node as usize].pos;
                (lt.light(at), lt.toward(at))
            })
            .unzip()
    }

    /// The light field from this cycle's leaves: every living tip.
    fn relight(&mut self) {
        let mut kids = vec![false; self.nodes.len()];
        for nd in &self.nodes[1..] {
            if nd.alive {
                kids[nd.parent as usize] = true;
            }
        }
        let leaves: Vec<(Vec3, f64)> = self
            .nodes
            .iter()
            .zip(&kids)
            .filter(|(nd, _)| nd.alive)
            .map(|(nd, k)| (nd.pos, if *k { 0.0 } else { 1.0 } + nd.rec_n))
            .filter(|x| x.1 > 0.0)
            .collect();
        self.w.light.rebuild(&leaves);
    }

    /// Per node: the demand-weighted light of the main line (qm) and the
    /// laterals (ql) for allocation, the tips with shed pipes (cnt), and the
    /// subtree's light gathered (lit) and wood upkeep (upk): its net is
    /// lit - upk. A metamer one unit long carrying one tip's pipe costs
    /// the upkeep setting; cost grows with its volume.
    #[allow(clippy::type_complexity)]
    fn basipetal(&self, q: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let n = self.nodes.len();
        let z = || vec![0.0; n];
        let (mut qm, mut ql, mut cnt, mut lit, mut upk, mut wr) = (z(), z(), z(), z(), z(), z());
        // Each record reads its long shoot's light once: its short shoots'
        // leaves gather it, their wood costs its upkeep, and it draws on the
        // laterals' side by what its short internodes build.
        for (i, nd) in self.nodes.iter().enumerate() {
            if nd.alive && nd.rec_n > 0.0 {
                let x = self.w.light.light(nd.pos);
                lit[i] += nd.rec_n * x;
                upk[i] += self.p.upkeep * nd.rec_len / self.unit;
                cnt[i] += nd.rec_n;
                wr[i] = x * nd.rec_n * self.p.short;
                ql[i] += wr[i];
            }
        }
        for (b, &x) in self.buds.iter().zip(q) {
            let w = x * self.shoot_len(b.phi);
            if b.terminal { qm[b.node as usize] += w } else { ql[b.node as usize] += w }
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
            // Leaves are on the living tips; a bud gathers nothing itself.
            if !kids[i] {
                lit[i] += self.w.light.light(self.nodes[i].pos);
            }
            cnt[p] += cnt[i];
            upk[i] += self.p.upkeep * self.nodes[i].len / self.unit * cnt[i].powf(2.0 / e);
            upk[p] += upk[i];
            lit[p] += lit[i];
        }
        cnt[0] += if kids[0] { 0.0 } else { 1.0 };
        (qm, ql, cnt, lit, upk, wr)
    }

    /// A shoot's internode as a share of a young shoot's, by age.
    fn shoot_len(&self, phi: f64) -> f64 {
        1.0 - (1.0 - self.p.short) * phi
    }

    /// Borchert-Honda: the resource enters at the base and splits at every
    /// node, lambda to the main line and 1 - lambda to the laterals, each
    /// in proportion to what it gathered. Returns each bud's vigour.
    fn allocate(&self, q: &[f64], qm: &[f64], ql: &[f64], wr: &[f64], net: f64) -> (Vec<f64>, Vec<f64>) {
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
        let vb = self
            .buds
            .iter()
            .zip(q)
            .map(|(b, &x)| {
                let i = b.node as usize;
                let d = den(i);
                let wt = if b.terminal { l } else { 1.0 - l };
                if d > 0.0 { vin[i] * wt * x * self.shoot_len(b.phi) / d } else { 0.0 }
            })
            .collect();
        let vr = (0..n).map(|i| if wr[i] > 0.0 && den(i) > 0.0 { vin[i] * (1.0 - l) * wr[i] / den(i) } else { 0.0 }).collect();
        (vb, vr)
    }

    /// Remembers every branch's relative net balance, (light - upkeep) /
    /// (light + upkeep), and sheds it with a chance that rises smoothly from
    /// 0 as the remembered balance falls past -tolerance to -(tolerance +
    /// shedWidth), times the shed rate; its pipes stay in the parent. No
    /// branch is spared: the main line's balance is the crown's.
    fn shed(&mut self, t: usize, lit: &[f64], upk: &[f64], cnt: &[f64], cover: f64) -> usize {
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
            // The reserve pays its share of the branch's upkeep first.
            let bal = (lit[i] - (1.0 - cover) * upk[i]) / (lit[i] + upk[i]).max(1e-12);
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
        let v = v * self.efficiency(base.path);
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
        let mut shorts = 0.0;
        for k in 0..n {
            let here = self.nodes[at as usize].pos;
            let photo = Vec3::new(guide.x, 0.0, guide.z) * p.photo;
            d = (d + guide * p.xi + photo + trop * p.eta + self.inward(here) * p.envelope).normalized();
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
                path: self.nodes[at as usize].path + len,
                rec_n: 0.0,
                rec_len: 0.0,
                rec_mem: 0.0,
            });
            let wk = (1.0 - p.rhythm) * prof[k] / mean + p.rhythm * if k + 1 == n { n as f64 } else { 0.0 };
            let nk = self.nodes[idx as usize].key;
            let laterals = keyed_round(dev * wk, nk, 3);
            // The buds that do not become long shoots are short shoots.
            shorts += (p.branching * wk - dev * wk).max(0.0);
            for j in 0..laterals {
                let dir = self.bud_dir(d, nk, j, laterals, k, angle);
                self.buds.push(Bud { node: idx, dormant: 0, dir, phi: phi_l, terminal: false });
            }
            at = idx;
        }
        let end = &mut self.nodes[at as usize];
        end.rec_n += shorts;
        end.rec_len += shorts * self.unit * p.short;
        let tk = self.nodes[at as usize].key;
        (hash(tk, 5) < p.persistence).then_some(Bud { node: at, dormant: 0, dir: d, phi, terminal: true })
    }

    /// Growth efficiency by path length from the root (hydraulic limit,
    /// Ryan & Yoder 1997): 1 - (path / (hydraulic x H))^2, 1 when off.
    fn efficiency(&self, path: f64) -> f64 {
        if self.p.hydraulic <= 0.0 {
            return 1.0;
        }
        (1.0 - (path / (self.p.hydraulic * self.w.height)).powi(2)).clamp(0.0, 1.0)
    }

    /// Short shoots extend by what their record drew: v units of wood.
    fn grow_records(&mut self, vr: &[f64]) {
        for (i, &v) in vr.iter().enumerate() {
            if v > 0.0 {
                let e = self.efficiency(self.nodes[i].path);
                self.nodes[i].rec_len += v * e * self.unit;
            }
        }
    }

    /// Short shoots in deficit die in proportion: the record's remembered
    /// balance sheds the same smooth share a branch's would; their pipes stay.
    fn shed_records(&mut self, t: usize, cover: f64) {
        let p = self.p;
        for i in 0..self.nodes.len() {
            let nd = &self.nodes[i];
            if !nd.alive || nd.rec_n <= 0.0 {
                continue;
            }
            let lit = nd.rec_n * self.w.light.light(nd.pos);
            let upk = p.upkeep * nd.rec_len / self.unit;
            let bal = (lit - (1.0 - cover) * upk) / (lit + upk).max(1e-12);
            let nd = &mut self.nodes[i];
            nd.rec_mem = (1.0 - p.memory) * nd.rec_mem + p.memory * bal;
            if p.shed <= 0.0 || (t as f64 - f64::from(nd.born)) < p.shed_age {
                continue;
            }
            let x = ((-nd.rec_mem - p.tolerance) / p.shed_width.max(1e-9)).clamp(0.0, 1.0);
            let share = p.shed * x * x * (3.0 - 2.0 * x);
            nd.memory += nd.rec_n * share;
            nd.rec_len *= 1.0 - share;
            nd.rec_n *= 1.0 - share;
        }
    }

    /// The records' short shoots as wood: keyed-round(count) straight shoots
    /// of their mean length, in internodes of a short shoot's length.
    fn emit_records(&mut self) {
        let angle = self.p.angle1.to_radians();
        for i in 0..self.nodes.len() {
            let nd = self.nodes[i].clone();
            let m = keyed_round(nd.rec_n, nd.key, 23);
            if m == 0 {
                continue;
            }
            let each = nd.rec_len / nd.rec_n.max(1e-9);
            let step = self.unit * self.p.short;
            let k = ((each / step).round() as usize).max(1);
            for j in 0..m {
                let d = self.bud_dir(nd.dir, nd.key, j, m, j, angle);
                let mut at = i as u32;
                for s in 0..k {
                    let pos = self.nodes[at as usize].pos + d * (each / k as f64);
                    let key = self.next_key;
                    self.next_key += 1;
                    self.nodes.push(Node {
                        parent: at,
                        pos,
                        dir: d,
                        len: each / k as f64,
                        lateral: s == 0,
                        alive: true,
                        key,
                        born: nd.born,
                        out: Vec3::ZERO,
                        memory: 0.0,
                        qmem: 0.0,
                        elev0: f32::NAN,
                        path: 0.0,
                        rec_n: 0.0,
                        rec_len: 0.0,
                        rec_mem: 0.0,
                    });
                    at = (self.nodes.len() - 1) as u32;
                }
            }
        }
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
