//! Stage 1's module rule ([M98], Troll's model): the relay and fork, and
//! the death records that let a relayed head die without its base.
use super::{elev, hash, keyed_round, lerp, Bud, Grower, NONE};
use telperion_core::math::Vec3;

impl Grower {
    /// Whether a record is dead in year `t`: its own year has come, or the
    /// record of the node it was born on is dead (memoised per year:
    /// 0 unknown, 1 alive, 2 dead).
    pub(super) fn dead(&self, rec: u32, t: f32, memo: &mut [u8], chain: &mut Vec<u32>) -> bool {
        chain.clear();
        let mut r = rec;
        let mut verdict = loop {
            match memo.get(r as usize).copied().unwrap_or(0) {
                1 => break false,
                2 => break true,
                _ => {}
            }
            chain.push(r);
            if self.die[r as usize] <= t {
                break true;
            }
            let up = self.up[r as usize];
            if up == NONE {
                break false;
            }
            r = self.axis[up as usize];
        };
        for &r in chain.iter().rev() {
            verdict |= self.die[r as usize] <= t;
            if let Some(m) = memo.get_mut(r as usize) {
                *m = 1 + u8::from(verdict);
            }
        }
        verdict
    }

    pub(super) fn record(&mut self, dies: f64, up: u32) -> u32 {
        self.die.push(dies as f32);
        self.up.push(up);
        (self.die.len() - 1) as u32
    }

    /// The relay ([M98]): buds in the curvature zone, on the upper side,
    /// take the module's dominance; the first continues the line, and near
    /// maturity they fork, spread evenly and alike. The head becomes a
    /// branch: its phi jumps, and it lives a while, longer as the tree
    /// establishes.
    pub(super) fn relay(&mut self, b: &mut Bud, key: u64, t: u16) {
        let p = self.p;
        let age = f64::from(t);
        let mature = self.maturity(t);
        let c = keyed_round(1.0 + p.fork * mature, key, 43).max(1);
        let e_bend = elev(self.dir[b.bend as usize]);
        let az0 = b.out.z.atan2(b.out.x) + p.relay_turn * (hash(key, 47) - 0.5) * std::f64::consts::TAU;
        let phi_r = 1.0 - (1.0 - b.phi) * (-p.reiter_step * (c - 1) as f64).exp();
        let renew = age + 1.0 + self.life(phi_r);
        for j in 0..c {
            let (e, az) = if c == 1 {
                ((e_bend + p.relay_up).min(90.0), az0)
            } else {
                (90.0 - p.fork_angle, az0 + j as f64 * std::f64::consts::TAU / c as f64)
            };
            let out = Vec3::new(az.cos(), 0.0, az.sin());
            let e = e.to_radians();
            let dir = (Vec3::Y * e.sin() + out * e.cos()).normalized();
            let axis = self.record(renew, b.bend);
            self.buds.push(Bud { node: b.bend, dir, out, phi: phi_r, years: 0, fresh: true, cont: j == 0, axis, bend: NONE, bend_next: NONE, relayed: false });
        }
        self.relays += 1;
        // A reiterate renews the life of every axis carrying it: none is
        // shed before the young wood it bears.
        let mut r = self.axis[b.bend as usize];
        while (self.die[r as usize] as f64) < renew {
            self.die[r as usize] = renew as f32;
            let up = self.up[r as usize];
            if up == NONE {
                break;
            }
            r = self.axis[up as usize];
        }
        self.forks += usize::from(c > 1);
        // The head leaves the line: it is now a lateral of the curvature zone.
        let nx = b.bend_next as usize;
        self.lateral[nx] = true;
        self.elev0[nx] = elev(self.dir[nx]) as f32;
        let dies = age + 1.0 + p.head_life * self.establishment(t);
        let head = self.record(dies, b.bend);
        let mut i = b.node;
        while i != b.bend {
            self.axis[i as usize] = head;
            i = self.parent[i as usize];
        }
        b.axis = head;
        b.phi = 1.0 - (1.0 - b.phi) * (-p.head_jump).exp();
        b.relayed = true;
    }
    /// An axis's lifespan in years, falling with its phi at birth.
    pub(super) fn life(&self, phi: f64) -> f64 {
        let p = self.p;
        lerp(p.life0, p.life1, 1.0 - (1.0 - phi).powf(p.life_shape))
    }

    /// The tree's maturity: a logistic of its age about forkAge.
    pub(super) fn maturity(&self, t: u16) -> f64 {
        let p = self.p;
        1.0 / (1.0 + (-(f64::from(t) - p.fork_age) / p.fork_width.max(1e-9)).exp())
    }
}
