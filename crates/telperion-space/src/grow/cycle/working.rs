//! The one cycle's working set (fn-255, CYCLE-COST.md levers 1 to 3). The
//! balance and the allotment visit only the open axes: those on the way
//! from the root to a living apex or a sleeping bud. A closed subtree lays
//! no more wood, so its upkeep is frozen once, as the sum the whole tree's
//! pass takes; its light and lit presence are none.
//!
//! **The one reduction order** (COUNCIL.md, "Decided by the spike"; the
//! device takes the same): a subtree's sum is its own value, then each
//! child's subtotal in the order they stand on it (by node, then index, a
//! continuation or relay at the tip last); the normaliser's per-PA sums
//! run over the growing buds in that subtree order (preorder), cut into
//! blocks of `BLOCK` buds, each block summed in order and the blocks then
//! added in order (`order`, `passes.rs`). Every sum is therefore the same whether
//! a closed subtree is frozen or summed again, and whatever retires. The
//! links and each axis's children are kept across cycles, and the lists
//! each pass fills are kept for the next.
use crate::allocation::{drawn, sized, Link, TIP};

/// The growing buds a block of the normaliser's sums holds.
pub(super) const BLOCK: usize = 1024;
use crate::structure::{Axis, Origin};

/// What the passes keep across cycles.
#[derive(Debug, Default)]
pub(in crate::grow) struct Working {
    pub(super) links: Vec<Link>,
    pub(super) pa: Vec<usize>,
    /// Each axis's children by node then index (the tip's last): the
    /// order every subtree sum and the split take them in.
    by_node: Vec<Vec<u32>>,
    /// A closed subtree's upkeep, once taken (NaN until then).
    frozen: Vec<f32>,
    /// Whether each axis is open this cycle, the open ones in order, and
    /// each one's place among them.
    pub(super) open: Vec<bool>,
    pub(super) list: Vec<usize>,
    pub(super) closed: Vec<usize>,
    pub(super) at: Vec<u32>,
    /// The passes' lists, by axis.
    pub(super) light: Vec<f32>,
    pub(super) cost: Vec<f32>,
    pub(super) living: Vec<bool>,
    pub(super) sub: Vec<f32>,
    pub(super) incoming: Vec<f32>,
    pub(super) bud: Vec<f32>,
    rest: Vec<f32>,
    kids: Vec<u32>,
    /// Which subtrees have been retired (`retire.rs`).
    pub(super) retire: super::retire::Retirement,
    /// The tests' references: every laid axis open, none frozen; none
    /// retired.
    pub(super) all_open: bool,
    pub(super) keep_all: bool,
    /// fn-270's retirement without the final age (spec R3): a subtree is
    /// retired once its fade at the age grown so far is none, and a
    /// fading one waits (`retire.rs`). A test hook until task 2 makes it
    /// the rule.
    pub(super) continuable: bool,
}

impl Working {
    /// The tests' reference: every laid axis open and none retired.
    pub(super) fn reference() -> Self {
        Self { all_open: true, keep_all: true, ..Self::default() }
    }

    /// fn-270's retirement without the final age.
    pub(super) fn continuable() -> Self {
        Self { continuable: true, ..Self::default() }
    }

    /// What the settle reads of the working set: its retirement.
    pub(in crate::grow) fn settled_copy(&self) -> Self {
        Self { retire: self.retire.clone(), keep_all: self.keep_all, ..Self::default() }
    }

    /// Axis `i`'s children, in the order they stand on it.
    pub(super) fn children(&self, i: usize) -> &[u32] {
        &self.by_node[i]
    }

    /// Which subtrees have been retired.
    pub(in crate::grow) fn retirement(&self) -> &super::retire::Retirement {
        &self.retire
    }

    /// The laid axes `axes[..n]` taken in: the new ones' links and PAs,
    /// each set among its bearer's children.
    pub(super) fn extend(&mut self, axes: &[Axis], n: usize) {
        for i in self.links.len()..n {
            let link = match axes[i].origin {
                Origin::Seed => Link { parent: None, node: 0 },
                Origin::Lateral { parent, node, .. } => Link { parent: Some(parent), node },
                Origin::Continuation { parent } | Origin::Relay { parent, .. } => Link {
                    parent: Some(parent),
                    node: TIP,
                },
            };
            if let Some(p) = link.parent {
                let links = &self.links;
                let kids = &mut self.by_node[p];
                let at = kids.partition_point(|&c| links[c as usize].node <= link.node);
                kids.insert(at, i as u32);
            }
            self.links.push(link);
            self.pa.push(axes[i].pa);
            self.by_node.push(Vec::new());
            self.frozen.push(f32::NAN);
            self.open.push(false);
            self.at.push(u32::MAX);
            self.light.push(0.0);
            self.cost.push(0.0);
            self.living.push(false);
            self.sub.push(0.0);
            self.incoming.push(0.0);
            self.bud.push(0.0);
        }
    }

    /// This cycle's open axes: every one on the way to `from`; and in
    /// `closed`, those that closed since the last cycle or were laid closed.
    pub(super) fn open_from(&mut self, from: impl Iterator<Item = usize>, laid: usize) {
        let was = std::mem::take(&mut self.list);
        for &i in &was {
            self.open[i] = false;
        }
        if self.all_open {
            self.list.extend(0..self.links.len());
            self.list.iter().for_each(|&i| self.open[i] = true);
        }
        for mut i in from.filter(|&i| i < self.links.len()) {
            while !self.open[i] {
                self.open[i] = true;
                self.list.push(i);
                match self.links[i].parent {
                    Some(p) => i = p,
                    None => break,
                }
            }
        }
        // In index order: a scan of the marks, cheaper than a sort.
        let low = self.list.iter().copied().min().unwrap_or(0);
        self.list.clear();
        self.list.extend((low..self.open.len()).filter(|&i| self.open[i]));
        for (k, &i) in self.list.iter().enumerate() {
            self.at[i] = k as u32;
        }
        self.closed.clear();
        let fresh = laid..self.open.len();
        let open = &self.open;
        self.closed.extend(was.into_iter().chain(fresh).filter(|&i| !open[i]));
    }

    /// Each open axis's subtree's light, upkeep and living, its own given
    /// by `own`, then its children's in order: a closed child's frozen
    /// upkeep, no light and nothing living.
    pub(super) fn totals(&mut self, own: &dyn Fn(usize) -> (f32, f32, bool), upkeep: &dyn Fn(usize) -> f32) {
        for k in (0..self.list.len()).rev() {
            let i = self.list[k];
            let (mut light, mut cost, mut living) = own(i);
            for j in 0..self.by_node[i].len() {
                let c = self.by_node[i][j] as usize;
                if self.open[c] {
                    light += self.light[c];
                    cost += self.cost[c];
                    living |= self.living[c];
                } else {
                    cost += self.closed(c, upkeep);
                }
            }
            (self.light[i], self.cost[i], self.living[i]) = (light, cost, living);
        }
    }

    /// A closed subtree's upkeep, taken once as the whole tree's pass
    /// takes it.
    fn closed(&mut self, c: usize, upkeep: &dyn Fn(usize) -> f32) -> f32 {
        if !self.frozen[c].is_nan() {
            return self.frozen[c];
        }
        let mut cost = upkeep(c);
        for j in 0..self.by_node[c].len() {
            let g = self.by_node[c][j] as usize;
            cost += self.closed(g, upkeep);
        }
        self.frozen[c] = cost;
        cost
    }

    /// `allocation::split` over the open axes: each subtree's lit
    /// presence from `own` (none where closed), then the base's vigour
    /// split at every branching point, parents first; each axis's own
    /// bud's share in `bud`.
    pub(super) fn split(&mut self, own: &[f32], lambda: &[f32]) {
        for k in (0..self.list.len()).rev() {
            let i = self.list[k];
            let mut sub = own[k];
            for j in 0..self.by_node[i].len() {
                let c = self.by_node[i][j] as usize;
                if self.open[c] {
                    sub += self.sub[c];
                }
            }
            self.sub[i] = sub;
            self.incoming[i] = if self.links[i].parent.is_none() { sub } else { 0.0 };
            self.bud[i] = 0.0;
        }
        for k in 0..self.list.len() {
            let i = self.list[k];
            let v = self.incoming[i];
            if v <= 0.0 || self.sub[i] <= 0.0 {
                continue;
            }
            self.share(i, own[k], lambda[k]);
        }
    }

    /// One axis's vigour split between what carries it on and its lit
    /// children, in the arithmetic `allocation::split` takes.
    fn share(&mut self, i: usize, own: f32, l: f32) {
        let mut kids = std::mem::take(&mut self.kids);
        kids.clear();
        let lit = |c: &&u32| self.open[**c as usize] && self.sub[**c as usize] > 0.0;
        kids.extend(self.by_node[i].iter().filter(lit));
        self.rest.clear();
        self.rest.resize(kids.len() + 1, own);
        for j in (0..kids.len()).rev() {
            self.rest[j] = self.rest[j + 1] + self.sub[kids[j] as usize];
        }
        let mut v = self.incoming[i];
        let mut k = 0;
        let node = |c: u32| self.links[c as usize].node;
        while k < kids.len() && node(kids[k]) != TIP {
            let at = node(kids[k]);
            let end = k + kids[k..].iter().take_while(|&&c| node(c) == at).count();
            let lateral: f32 = kids[k..end].iter().map(|&c| self.sub[c as usize]).sum();
            let main = self.rest[end];
            let split = l * main + (1.0 - l) * lateral;
            if split > 0.0 {
                for &c in &kids[k..end] {
                    self.incoming[c as usize] = v * (1.0 - l) * self.sub[c as usize] / split;
                }
                v = v * l * main / split;
            }
            k = end;
        }
        let above = self.rest[k];
        if above > 0.0 {
            self.bud[i] = v * own / above;
            for &c in &kids[k..] {
                self.incoming[c as usize] = v * self.sub[c as usize] / above;
            }
        }
        self.kids = kids;
    }
}

impl Working {
    /// The open axes' places in the subtree order: preorder from the
    /// root, each axis's open children in the order they stand on it.
    pub(super) fn order(&self) -> Vec<u32> {
        let mut order = Vec::with_capacity(self.list.len());
        let mut stack = match self.list.first() {
            Some(&root) => vec![root as u32],
            None => Vec::new(),
        };
        while let Some(i) = stack.pop() {
            order.push(self.at[i as usize]);
            let kids = self.by_node[i as usize].iter().rev();
            stack.extend(kids.filter(|&&c| self.open[c as usize]));
        }
        order
    }
}

/// Each open axis's size from its bud's vigour, presence and ψ against its
/// PA's sums, `[Σp, Σp(v/p)^ψ]`, taken in the one reduction order: the
/// growing buds (presence above none) in `order`, a block of `BLOCK` at a
/// time, so the blocks are the same whatever else is open.
pub(super) fn sizes(vigour: &[f64], presence: &[f64], psi: &[f64], pa: &[usize], order: &[u32]) -> Vec<f64> {
    let groups = pa.iter().copied().max().map_or(0, |g| g + 1);
    let order: Vec<u32> = order.iter().copied().filter(|&k| presence[k as usize] > 0.0).collect();
    let mut sums = vec![[0.0f32; 2]; groups];
    let mut part = vec![[0.0f32; 2]; groups];
    for block in order.chunks(BLOCK) {
        part.iter_mut().for_each(|s| *s = [0.0; 2]);
        for &k in block {
            let k = k as usize;
            let p = presence[k] as f32;
            let [total, weighted] = &mut part[pa[k]];
            *total += p;
            *weighted += drawn(vigour[k] as f32, p, psi[k] as f32) * p;
        }
        for (s, b) in sums.iter_mut().zip(&part) {
            (s[0], s[1]) = (s[0] + b[0], s[1] + b[1]);
        }
    }
    sized(vigour, presence, psi, pa, &sums)
}
