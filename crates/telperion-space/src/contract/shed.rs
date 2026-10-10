//! D16 (CONTRACT-A.md 2.7; Astra's review, point 6; in f32 since fn-254):
//! a lateral's remembered balance, its hazard's chance, the threshold its
//! draw is held to, and the kept share its lead past that threshold leaves
//! it, as every other presence takes one (FINE-CONTRACT.md, host decision
//! 8).
use super::presence::{of, over, times};
use super::{fails, threshold};

/// One lateral's balance inputs: its collected light `q` and cost `c`, its
/// memory, its PA's hazard and tolerance, its 53-bit draw, its window
/// (`Windows::window_q` at the cycle, stake one) and its bearer's kept
/// share.
#[derive(Debug, Clone, Copy, Default)]
pub struct Bud {
    pub q: f32,
    pub c: f32,
    pub memory: f32,
    pub hazard: f32,
    pub tolerance: f32,
    pub draw: u64,
    pub window: f32,
    pub keep: f32,
}

/// What the balance makes of it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Balance {
    pub memory: f32,
    /// The draw's threshold: shed where the draw lies below it.
    pub threshold: u64,
    pub shed: bool,
    /// The kept share: the bearer's where shed or undecided.
    pub kept: f32,
}

/// `(q - c) / (q + c)`; one where `q + c` is none or less.
fn now(q: f32, c: f32) -> f32 {
    let total = q + c;
    if total <= 0.0 {
        return 1.0;
    }
    (q - c) / total
}

/// D16: the memory halves toward the balance now; the hazard is the PA's
/// times the memory's shortfall; shed where the draw lies below the
/// threshold of `1 - e^-hazard` (none where the hazard is none or less);
/// else the kept share is the bearer's times the presence of the draw's
/// lead over its window.
pub fn balance(b: &Bud) -> Balance {
    let memory = (b.memory + now(b.q, b.c)) * 0.5;
    let hazard = b.hazard * (b.tolerance - memory).max(0.0);
    let undecided = Balance {
        memory,
        threshold: 0,
        shed: false,
        kept: b.keep,
    };
    if hazard <= 0.0 {
        return undecided;
    }
    let t = threshold(fails(hazard * std::f32::consts::LOG2_E));
    let shed = b.draw < t;
    let kept = if shed {
        b.keep
    } else {
        times(b.keep, of(over(t, b.draw), b.window))
    };
    Balance {
        threshold: t,
        shed,
        kept,
        ..undecided
    }
}

/// One axis of a grower as D16's walk reads it: its bearer's index (none
/// for a base), whether it decides (a living lateral), and its balance
/// inputs, `keep` aside (the walk hands its bearer's down).
#[derive(Debug, Clone, Copy, Default)]
pub struct Lateral {
    pub parent: Option<u32>,
    pub decides: bool,
    pub bud: Bud,
}

/// What the walk makes of an axis: whether it is shed, its kept share,
/// and its memory where it decided.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Walked {
    pub shed: bool,
    pub kept: f32,
    pub memory: Option<f32>,
}

/// D16 over a grower's axes, parents first: each inherits its bearer's
/// shedding and kept share, axis 1 takes `root` where its bearer decides it
/// elsewhere, and each living lateral under a living bearer decides by
/// `balance` with its bearer's kept share. The one walk every executor
/// takes (fn-211 task 10).
pub fn walk(axes: &[Lateral], root: Option<(bool, f32)>) -> Vec<Walked> {
    let mut out: Vec<Walked> = Vec::with_capacity(axes.len());
    for (i, a) in axes.iter().enumerate() {
        let (dead, keep) = match a.parent {
            Some(p) => (out[p as usize].shed, out[p as usize].kept),
            None => (false, 1.0),
        };
        let mut w = Walked {
            shed: dead,
            kept: keep,
            memory: None,
        };
        if let (1, Some((shed, kept))) = (i, root) {
            (w.shed, w.kept) = (shed, kept);
        } else if !dead && a.decides {
            let b = balance(&Bud { keep, ..a.bud });
            w.memory = Some(b.memory);
            if b.shed {
                w.shed = true;
            } else if b.threshold > 0 {
                w.kept = b.kept;
            }
        }
        out.push(w);
    }
    out
}
