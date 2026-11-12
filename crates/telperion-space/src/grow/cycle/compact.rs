//! A retired subtree's records let go (fn-333). Retirement lets go of a
//! shed subtree's phytomers and draws (`retire.rs`) and its root stands
//! for it; nothing reads the axes inside it again, for every pass, the
//! settle's included, stops at the root: the balance, which charges dead
//! wood nothing, the evaluation of an ancestor at its fate, the fades at
//! its living. Those axes are then dropped from every store
//! the grower keeps by axis, and every axis held moves down to its place
//! among those held. The root goes with them (fn-357): its own fade is
//! none for good, and all that is read of it is its carry into its
//! bearer's living, so it keeps a `Stand` of 24 bytes, its bearer, node,
//! birth, vigour and living, until its bearer is retired in turn. The
//! order of the held axes is kept, so every pass visits them, and sums
//! them, in the order it did; the tree grows the same to the bit. It runs
//! once the axes let go but held reach a quarter of those held, so its
//! cost is spread over the cycles.
use super::super::Grower;
use super::retire::Fate;
use super::Working;
use crate::contract::presence::{q, real, times};
use crate::shed::Stand;
use crate::structure::Origin;

/// Each axis's new index, or none where it is let go; past `map`'s end,
/// moved down by every axis let go.
struct Remap {
    map: Vec<u32>,
    gone: usize,
}

const GONE: u32 = u32::MAX;

impl Remap {
    fn at(&self, i: usize) -> Option<usize> {
        match self.map.get(i) {
            Some(&GONE) => None,
            Some(&k) => Some(k as usize),
            None => Some(i - self.gone),
        }
    }

    fn held(&self, i: usize) -> bool {
        self.map.get(i).is_none_or(|&k| k != GONE)
    }

    /// The held axis `i`'s new index.
    fn to(&self, i: usize) -> usize {
        self.at(i).expect("a held axis")
    }

    /// `v`, by axis, without those let go. Its capacity is kept, for the
    /// tree grows into it again, unless it is over twice what is held:
    /// handing pages back each time cost the oak at 200 five seconds of
    /// faults (RESULT.md).
    fn keep<T>(&self, v: &mut Vec<T>) {
        let mut i = 0;
        v.retain(|_| (self.held(i), i += 1).0);
        if v.capacity() > 2 * v.len() {
            v.shrink_to_fit();
        }
    }

    /// `v`, a list of axes, without those let go, the rest renamed.
    fn rename(&self, v: &mut Vec<usize>) {
        v.retain_mut(|i| self.at(*i).map(|k| *i = k).is_some());
    }
}

impl Grower {
    /// The axes of a retired subtree let go where enough have gathered,
    /// the tree grown to `age`.
    pub(in crate::grow) fn compact(&mut self, age: u32) {
        let Some(w) = self.working.as_deref() else {
            return;
        };
        let least = match w.eager {
            true => 1,
            false => (self.axes.len() / 4).max(1 << 12),
        };
        if w.keep_all || w.keep_records || w.retire.loose < least {
            return;
        }
        let r = gone(w);
        if r.gone == 0 {
            return;
        }
        let stands = self.stands(w, &r, age);
        self.rename_axes(&r);
        let w = self.working.as_deref_mut().expect("a working set");
        rename_working(w, &r);
        w.retire.stands = stands;
        if let Some(c) = self.census.as_mut() {
            c.compact(&|i| r.held(i));
        }
    }

    /// The grower's stores by axis, and every index into them, renamed.
    fn rename_axes(&mut self, r: &Remap) {
        r.keep(&mut self.axes);
        for a in &mut self.axes {
            if let crate::structure::Origin::Lateral { parent, .. }
            | crate::structure::Origin::Continuation { parent }
            | crate::structure::Origin::Relay { parent, .. } = &mut a.origin
            {
                *parent = r.to(*parent);
            }
        }
        r.keep(&mut self.draws);
        r.keep(&mut self.horizon);
        r.keep(&mut self.units);
        r.keep(&mut self.roots);
        r.keep(&mut self.successor);
        self.successor.iter_mut().for_each(|s| r.rename(s));
        if let Some(sketch) = self.sketch.as_mut() {
            r.keep(&mut sketch.pencils);
        }
        self.live.iter_mut().for_each(|a| a.axis = r.to(a.axis));
        for asleep in &mut self.asleep {
            asleep.iter_mut().for_each(|s| s.parent = r.to(s.parent));
        }
        let w = self.working.as_deref_mut().expect("a working set");
        // A bud that woke inside is let go with it; the rest renamed, and
        // each axis's woken bud by its new place among them.
        let mut place = Vec::with_capacity(self.woken.len());
        let (mut k, seen) = (0, w.retire.seen);
        self.woken.retain_mut(|b| {
            let held = r.held(b.axis);
            if held {
                (b.axis, b.bearer) = (r.to(b.axis), r.to(b.bearer));
            }
            place.push(if held { k } else { GONE });
            k += u32::from(held);
            held
        });
        w.retire.seen = place[..seen].iter().filter(|&&p| p != GONE).count();
        r.keep(&mut w.retire.woken);
        for k in w.retire.woken.iter_mut().filter(|k| **k != GONE) {
            *k = place[*k as usize];
        }
    }
}

impl Grower {
    /// The stands after `r`: each held root's, beside those let go before
    /// whose bearer is held, each bearer renamed, in the bearers' order.
    fn stands(&self, w: &Working, r: &Remap, age: u32) -> Vec<Stand> {
        let renamed = |s: &Stand| Stand {
            bearer: r.to(s.bearer as usize) as u32,
            ..*s
        };
        let before = w.retire.stands.iter().filter(|s| r.held(s.bearer as usize));
        let mut out: Vec<Stand> = before.map(renamed).collect();
        for (i, f) in w.retire.fate.iter().enumerate() {
            let Fate::Root(living) = *f else {
                continue;
            };
            let axis = &self.axes[i];
            let Origin::Lateral { parent, node, .. } = axis.origin else {
                unreachable!("a retired root is a lateral");
            };
            // Its vigour as the settle assigns it (`Grower::mark`,
            // `presence::assign`), a woken bud's share kept, final by now
            // (`retire.rs`).
            let mut birth = self.draws[i].birth;
            if let Some(&k) = w.retire.woken.get(i).filter(|&&k| k != GONE) {
                let kept =
                    self.woken[k as usize].kept(&self.axes, &self.draws, &self.successor, age);
                birth[1] = real(times(q(birth[1]), q(kept)));
            }
            out.push(Stand {
                bearer: r.to(parent) as u32,
                node: node as u32,
                birth: axis.birth,
                vigour: times(q(birth[0]), q(birth[1])),
                living,
            });
        }
        out.sort_by_key(|s| s.bearer);
        out
    }
}

/// The axes let go: retired roots and those inside them.
fn gone(w: &Working) -> Remap {
    let n = w.links.len();
    let mut out = vec![false; n];
    let mut todo = Vec::new();
    for (i, f) in w.retire.fate.iter().enumerate() {
        if matches!(f, Fate::Root(_)) {
            todo.push(i);
        }
    }
    while let Some(i) = todo.pop() {
        out[i] = true;
        todo.extend(w.by_node[i].iter().map(|&c| c as usize));
    }
    let (mut map, mut k) = (Vec::with_capacity(n), 0u32);
    for &g in &out {
        map.push(if g { GONE } else { k });
        k += u32::from(!g);
    }
    Remap {
        map,
        gone: n - k as usize,
    }
}

/// The working set's stores by axis, and every index into them, renamed.
fn rename_working(w: &mut Working, r: &Remap) {
    r.keep(&mut w.links);
    for l in &mut w.links {
        l.parent = l.parent.map(|p| r.to(p));
    }
    r.keep(&mut w.pa);
    r.keep(&mut w.by_node);
    for kids in &mut w.by_node {
        kids.retain_mut(|c| r.at(*c as usize).map(|k| *c = k as u32).is_some());
    }
    r.keep(&mut w.open);
    r.rename(&mut w.list);
    r.rename(&mut w.closed);
    r.keep(&mut w.at);
    for v in [
        &mut w.light,
        &mut w.cost,
        &mut w.sub,
        &mut w.incoming,
        &mut w.bud,
    ] {
        r.keep(v);
    }
    r.keep(&mut w.living);
    let t = &mut w.retire;
    r.keep(&mut t.fate);
    r.rename(&mut t.pending);
    r.rename(&mut t.fading);
    let loose = |f: &&Fate| matches!(f, Fate::Root(_) | Fate::Inside);
    t.loose = t.fate.iter().filter(loose).count();
}
