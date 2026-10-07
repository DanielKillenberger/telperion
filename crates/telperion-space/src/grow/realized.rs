//! Growth under realized mechanics (fn-209 F1 task 6; fn-211's
//! DESIGN-INTERLEAVE.md, "After Astra's review", host decision 3): the
//! structure above a cut is grown by the engine's own grower, each bud
//! below the cut made where it stands and never grown there (its stand-in
//! keeps its place among its bearer's children); each fine root is then
//! grown whole by the same grower from its bud alone. Where no bud reads
//! light, an apex's growth reads only its own axis, so the two together
//! are the whole tree grown in one, draw for draw, and the order of the
//! axes each holds is the whole tree's restricted to it.
use super::alone::{start, woken_bud, Bud};
use super::{Grower, Rules};
use crate::dormant::Woken;
use crate::error::Result;
use crate::presence::{assign, Draws};
use crate::structure::{Axis, Origin};
use std::sync::Arc;

/// Which laterals the structure above a cut hands on, by their first
/// axis's PA and birth: decided before growth, from the bud alone.
#[derive(Debug)]
pub(crate) struct Hands {
    age: u32,
    fine: Vec<bool>,
}

impl Hands {
    pub fn new(pas: usize, age: u32, fine: impl Fn(usize, u32) -> bool) -> Self {
        let span = age as usize + 1;
        let fine = (0..pas * span)
            .map(|k| fine(k / span, (k % span) as u32))
            .collect();
        Self { age, fine }
    }

    pub fn fine(&self, pa: usize, birth: u32) -> bool {
        birth <= self.age && self.fine[pa * (self.age as usize + 1) + birth as usize]
    }
}

/// A tree's axes grown, marked and assigned, and the phytomers grown for
/// it, the shed ones included.
pub(crate) struct Assigned {
    pub axes: Vec<Axis>,
    pub grown: u64,
}

/// A bud handed on below the cut: its stand-in's index above it, and the
/// bud as its bearer made it.
pub(crate) struct Handed {
    pub at: usize,
    pub bud: Bud,
}

/// Grows the structure above the cut `hands` draws, from the seed, on
/// `threads` cores: its axes, a stand-in at each bud handed on, and the
/// buds handed on, by their stand-ins' order.
pub(crate) fn above(
    rules: &Rules,
    hands: Hands,
    seed: u64,
    threads: usize,
) -> Result<(Assigned, Vec<Handed>)> {
    let mut laterals = Vec::new();
    let (grown, woken) = streamed(rules, hands, seed, threads, &mut |made| {
        laterals.extend(made)
    })?;
    Ok((grown, merged(laterals, woken, |h| h.at)))
}

/// `above`, each cycle's laterals handed to `sow` as the cycle makes them
/// (task 9: the executor grows them while the structure grows on): its
/// axes, and the sleeping buds that woke below the cut, by their
/// stand-ins' order. A lateral below the cut never grows here, so what it
/// is handed on with is fixed when it is made; the woken buds wait for
/// what their bearers keep of them, which the whole growth decides.
pub(crate) fn streamed(
    rules: &Rules,
    hands: Hands,
    seed: u64,
    threads: usize,
    sow: &mut dyn FnMut(Vec<Handed>),
) -> Result<(Assigned, Vec<Handed>)> {
    let rules = Rules {
        alone: false,
        hands: Some(Arc::new(hands)),
        ..rules.clone()
    };
    let share = super::parallel::SHARE;
    let mut g = Grower::seeded(rules, (threads, share), None, seed)?;
    #[cfg(test)]
    let mut sown = Vec::new();
    let mut from = 0;
    for cycle in 1..=g.rules.age {
        g.step(cycle, &mut |_| {})?;
        let made = g.laterals(from);
        from = g.axes.len();
        #[cfg(test)]
        sown.extend(made.iter().map(|h| format!("{} {:?}", h.at, h.bud)));
        sow(made);
    }
    g.mark();
    // Every lateral as handed on when made is the one the grown tree
    // holds (each crate test that grows through the partition checks).
    #[cfg(test)]
    {
        let last: Vec<String> = g
            .laterals(0)
            .iter()
            .map(|h| format!("{} {:?}", h.at, h.bud))
            .collect();
        assert_eq!(sown, last, "the laterals as made are those grown");
    }
    let woken = g.woken();
    Ok((g.assigned(), woken))
}

/// Two lists, each ordered by `at`, as one.
pub(crate) fn merged<T>(a: Vec<T>, b: Vec<T>, at: impl Fn(&T) -> usize) -> Vec<T> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut a, mut b) = (a.into_iter().peekable(), b.into_iter().peekable());
    while let (Some(x), Some(y)) = (a.peek(), b.peek()) {
        let next = if at(x) < at(y) { a.next() } else { b.next() };
        out.extend(next);
    }
    out.extend(a);
    out.extend(b);
    out
}

/// Grows `bud`'s whole subtree from the bud alone: its bearer's stub
/// (axis 0, whose place the root's origin names), then its axes.
pub(crate) fn root(rules: &Rules, bud: &Bud) -> Result<Assigned> {
    let rules = Rules {
        alone: false,
        hands: None,
        ..rules.clone()
    };
    let (mut g, from, _) = start(rules, 0, Some(bud))?;
    for cycle in from..=g.rules.age {
        if g.idle(cycle) {
            break;
        }
        g.step(cycle, &mut |_| {})?;
    }
    g.mark();
    Ok(g.assigned())
}

impl Grower<'_> {
    /// The laterals handed on from axis `from` on, by their stand-ins'
    /// index: made and not grown.
    fn laterals(&self, from: usize) -> Vec<Handed> {
        let lateral = |a: &Axis| {
            matches!(a.origin, Origin::Lateral { woken: false, .. })
                && self.rules.hands(a.pa, a.birth)
        };
        (from..self.axes.len())
            .filter(|&i| lateral(&self.axes[i]))
            .map(|i| Handed {
                at: i,
                bud: Bud {
                    axis: self.axes[i].clone(),
                    draws: Draws {
                        birth: self.draws[i].birth,
                        ..Draws::default()
                    },
                    root: self.roots[i],
                    horizon: self.horizon[i],
                    wake: None,
                },
            })
            .collect()
    }

    /// The sleeping buds handed on as they woke, by their stand-ins'
    /// index, each keeping what its bearer kept of it.
    fn woken(&self) -> Vec<Handed> {
        let age = self.rules.age;
        let mut out: Vec<Handed> = self
            .handed
            .iter()
            .map(|&(at, p)| {
                let woken = Woken {
                    axis: at,
                    bearer: p.bearer,
                    next: p.next,
                    cycle: p.cycle,
                    share: 1.0 - p.sleeper.sleep,
                };
                let kept = woken.kept(&self.axes, &self.draws, &self.successor, age);
                let horizon = self.horizon[p.sleeper.parent];
                Handed {
                    at,
                    bud: woken_bud(&self.rules, horizon, p, kept),
                }
            })
            .collect();
        out.sort_by_key(|h| h.at);
        out
    }

    /// Its axes with their presences assigned.
    fn assigned(mut self) -> Assigned {
        assign(&mut self.axes, &self.draws, self.species, self.threads);
        Assigned {
            axes: self.axes,
            grown: self.rules.grown,
        }
    }
}
