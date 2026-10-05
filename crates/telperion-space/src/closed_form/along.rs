//! Where a growth unit's wood and buds stand along its axis, in
//! expectation (fn-209, the stand-in's envelope). A zone's nodes follow the
//! zones before it in the order `ALONG` gives, each an internode long; its
//! nodes stand in order of their first place's draw (`grow/unit.rs`), the
//! largest draw at the base, so the PA that draw gives a place-0 bud
//! decides how far up the zone it stands. Every other place and every
//! sleeping bud stands at a node of no preferred rank.
use crate::species::{NodeLaw, MAX_NODES_PER_ZONE};

/// A length along an axis: its mean and its mean square.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Along {
    pub mean: f64,
    pub square: f64,
}

impl Along {
    /// This length and an independent one, end to end.
    pub fn then(self, more: Along) -> Along {
        Along {
            mean: self.mean + more.mean,
            square: self.square + 2.0 * self.mean * more.mean + more.square,
        }
    }

    /// `share` of the way along `length` from this point.
    pub fn into(self, length: Along, share: f64) -> Along {
        self.then(Along {
            mean: share * length.mean,
            square: share * share * length.square,
        })
    }
}

/// The moments E[N^p], p = 0 to 3, of a zone's node count.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Count([f64; 4]);

impl Count {
    /// The node count's moments under `law`, as `lineage::nodes` draws it.
    pub fn of(law: NodeLaw) -> Self {
        match law {
            NodeLaw::Poisson { mean: m } => Self([1.0, m, m + m * m, m + 3.0 * m * m + m * m * m]),
            NodeLaw::Uniform { min, max } => {
                let span = max - min + 1.0;
                let mut moments = [1.0, 0.0, 0.0, 0.0];
                for j in 0..(max.ceil() as u32).min(MAX_NODES_PER_ZONE) {
                    let stands = ((max - f64::from(j)) / span).clamp(0.0, 1.0);
                    let j = f64::from(j);
                    moments[1] += stands;
                    moments[2] += (2.0 * j + 1.0) * stands;
                    moments[3] += (3.0 * j * j + 3.0 * j + 1.0) * stands;
                }
                Self(moments)
            }
        }
    }

    /// The zone's length in internodes.
    pub fn length(&self, internode: f64) -> Along {
        Along {
            mean: self.0[1] * internode,
            square: self.0[2] * internode * internode,
        }
    }

    /// The zone's own wood from `start`, its internodes `internode` long,
    /// weighted by length: [E Σ l, E Σ l·x, E Σ l·x²] over its
    /// internodes' middles x.
    pub fn wood(&self, start: Along, internode: f64) -> [f64; 3] {
        let [_, n1, n2, n3] = self.0;
        let l = internode;
        // Σ (n - 1/2) = N²/2 and Σ (n - 1/2)² = N(4N² - 1)/12.
        let half = n2 / 2.0;
        let squares = (4.0 * n3 - n1) / 12.0;
        [
            l * n1,
            l * n1 * start.mean + l * l * half,
            l * n1 * start.square + 2.0 * l * l * start.mean * half + l * l * l * squares,
        ]
    }

    /// The buds of a place-0 draw in [low, high) on every node of the
    /// zone from `start`: their expected number per node count's node, and
    /// where each stands (at its node's tip).
    pub fn ranked(&self, start: Along, internode: f64, (low, high): (f64, f64)) -> (f64, Along) {
        let [_, n1, n2, n3] = self.0;
        let (a, b) = (low, high);
        // Integrals over the draw's band of 1, (1 - u), (1 - u)² and u(1 - u).
        let i0 = b - a;
        let i1 = (b - a) - (b * b - a * a) / 2.0;
        let i2 = ((1.0 - a).powi(3) - (1.0 - b).powi(3)) / 3.0;
        let iu = (b * b - a * a) / 2.0 - (b.powi(3) - a.powi(3)) / 3.0;
        // A node whose draw is u stands at rank 1 + Binomial(N - 1, 1 - u).
        let pairs = n2 - n1;
        let triples = n3 - 2.0 * n2 + n1;
        let count = n1 * i0;
        let rank = n1 * i0 + pairs * i1;
        let rank2 = n1 * i0 + pairs * (iu + 2.0 * i1) + triples * i2;
        (count, placed(count, rank, rank2, start, internode))
    }

    /// The buds of a place whose draw is apart from the node's rank, one a
    /// node: their expected number and where each stands, at its node's
    /// tip, or half an internode below it (`below`, a sleeping bud).
    pub fn spread(&self, start: Along, internode: f64, below: bool) -> (f64, Along) {
        let [_, n1, n2, n3] = self.0;
        let (rank, rank2) = if below {
            (n2 / 2.0, (4.0 * n3 - n1) / 12.0)
        } else {
            ((n2 + n1) / 2.0, (2.0 * n3 + 3.0 * n2 + n1) / 6.0)
        };
        (n1, placed(n1, rank, rank2, start, internode))
    }
}

/// Where buds stand from their summed ranks: `count` of them, Σ rank and
/// Σ rank², after `start`.
fn placed(count: f64, rank: f64, rank2: f64, start: Along, l: f64) -> Along {
    if count <= 0.0 {
        return start;
    }
    let (r1, r2) = (rank / count, rank2 / count);
    Along {
        mean: start.mean + l * r1,
        square: start.square + 2.0 * l * start.mean * r1 + l * l * r2,
    }
}

/// E[d^p], p = 1 to 3, of a lateral's dominance share d = 1 - D (1 - u⁴),
/// u uniform (`geometry::dominance`).
pub(crate) fn dominance(d: f64) -> [f64; 3] {
    if d <= 0.0 {
        return [1.0; 3];
    }
    let c = 1.0 - d;
    // E[u^4i] = 1 / (4i + 1).
    let e = |i: i32| 1.0 / f64::from(4 * i + 1);
    [
        c + d * e(1),
        c * c + 2.0 * c * d * e(1) + d * d * e(2),
        c.powi(3) + 3.0 * c * c * d * e(1) + 3.0 * c * d * d * e(2) + d.powi(3) * e(3),
    ]
}
