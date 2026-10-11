//! The words the device computes from its counts (`vm.wgsl`): an
//! expression over the census, compiled to a short program that stores
//! one word at its point of the cycle. The host bounds the same
//! expressions over the bounds it holds for the census's words, to size
//! each dispatch directly (DESIGN.md section 7a).
use super::super::io;
use crate::Gpu;

const VM: &str = include_str!("vm.wgsl");

/// An integer expression over the census.
#[derive(Debug, Clone)]
pub(super) enum E {
    C(u32),
    At(usize),
    Add(Box<E>, Box<E>),
    /// Saturating at none.
    Sub(Box<E>, Box<E>),
    Mul(Box<E>, Box<E>),
    /// Rounded up; none over none.
    Div(Box<E>, Box<E>),
    Min(Box<E>, Box<E>),
    Max(Box<E>, Box<E>),
    /// 1 where the first is the greater, else 0.
    Gt(Box<E>, Box<E>),
    /// Rounded down; none over none.
    Floor(Box<E>, Box<E>),
    /// The second where the first is not none, else the third.
    Sel(Box<E>, Box<E>, Box<E>),
    /// The workgroups the host gave dispatch `k` this cycle.
    Given(u32),
}

impl E {
    pub(super) fn add(self, b: E) -> E {
        E::Add(Box::new(self), Box::new(b))
    }
    pub(super) fn sub(self, b: E) -> E {
        E::Sub(Box::new(self), Box::new(b))
    }
    pub(super) fn mul(self, b: E) -> E {
        E::Mul(Box::new(self), Box::new(b))
    }
    pub(super) fn div(self, b: E) -> E {
        E::Div(Box::new(self), Box::new(b))
    }
    pub(super) fn min(self, b: E) -> E {
        E::Min(Box::new(self), Box::new(b))
    }
    pub(super) fn max(self, b: E) -> E {
        E::Max(Box::new(self), Box::new(b))
    }
    pub(super) fn gt(self, b: E) -> E {
        E::Gt(Box::new(self), Box::new(b))
    }
    pub(super) fn floor(self, b: E) -> E {
        E::Floor(Box::new(self), Box::new(b))
    }
    pub(super) fn sel(self, then: E, or: E) -> E {
        E::Sel(Box::new(self), Box::new(then), Box::new(or))
    }

    /// The value on the host, from `census`: what the device computes.
    pub(super) fn eval(&self, census: &[u32]) -> u32 {
        let bin = |a: &E, b: &E| (a.eval(census), b.eval(census));
        match self {
            E::C(k) => *k,
            E::At(w) => census[*w],
            E::Add(a, b) => { let (a, b) = bin(a, b); a.wrapping_add(b) }
            E::Sub(a, b) => { let (a, b) = bin(a, b); a.saturating_sub(b) }
            E::Mul(a, b) => { let (a, b) = bin(a, b); a.wrapping_mul(b) }
            E::Div(a, b) => { let (a, b) = bin(a, b); a.checked_div(b).map_or(0, |_| a.div_ceil(b)) }
            E::Min(a, b) => { let (a, b) = bin(a, b); a.min(b) }
            E::Max(a, b) => { let (a, b) = bin(a, b); a.max(b) }
            E::Gt(a, b) => { let (a, b) = bin(a, b); u32::from(a > b) }
            E::Floor(a, b) => { let (a, b) = bin(a, b); a.checked_div(b).unwrap_or(0) }
            E::Sel(c, a, b) => if c.eval(census) != 0 { a.eval(census) } else { b.eval(census) },
            E::Given(_) => u32::MAX,
        }
    }

    /// The census words the value reads.
    pub(super) fn words(&self, out: &mut Vec<usize>) {
        match self {
            E::C(_) | E::Given(_) => {}
            E::At(w) => out.push(*w),
            E::Add(a, b) | E::Sub(a, b) | E::Mul(a, b) | E::Div(a, b) | E::Min(a, b) | E::Max(a, b) | E::Gt(a, b) | E::Floor(a, b) => {
                a.words(out);
                b.words(out);
            }
            E::Sel(c, a, b) => [c, a, b].iter().for_each(|e| e.words(out)),
        }
    }

    /// The least and the most the value can be, where each census word
    /// `w` is at least `lo[w]` and at most `hi[w]`.
    pub(super) fn span(&self, lo: &[u32], hi: &[u32]) -> (u32, u32) {
        let bin = |a: &E, b: &E| (a.span(lo, hi), b.span(lo, hi));
        let floor = |a: u32, b: u32| a.checked_div(b).unwrap_or(0);
        let ceil = |a: u32, b: u32| a.checked_div(b).map_or(0, |_| a.div_ceil(b));
        match self {
            E::C(k) => (*k, *k),
            E::At(w) => (lo[*w], hi[*w]),
            E::Add(a, b) => { let (a, b) = bin(a, b); (a.0.saturating_add(b.0), a.1.saturating_add(b.1)) }
            E::Sub(a, b) => { let (a, b) = bin(a, b); (a.0.saturating_sub(b.1), a.1.saturating_sub(b.0)) }
            E::Mul(a, b) => { let (a, b) = bin(a, b); (a.0.saturating_mul(b.0), a.1.saturating_mul(b.1)) }
            // None over none: a divisor that can be none bounds nothing.
            E::Div(a, b) => { let (a, b) = bin(a, b); (ceil(a.0, b.1), if b.0 == 0 { a.1 } else { ceil(a.1, b.0) }) }
            E::Floor(a, b) => { let (a, b) = bin(a, b); (floor(a.0, b.1), if b.0 == 0 { a.1 } else { floor(a.1, b.0) }) }
            E::Min(a, b) => { let (a, b) = bin(a, b); (a.0.min(b.0), a.1.min(b.1)) }
            E::Max(a, b) => { let (a, b) = bin(a, b); (a.0.max(b.0), a.1.max(b.1)) }
            E::Gt(a, b) => { let (a, b) = bin(a, b); (u32::from(a.0 > b.1), u32::from(a.1 > b.0)) }
            E::Sel(c, a, b) => {
                let (c, a, b) = (c.span(lo, hi), a.span(lo, hi), b.span(lo, hi));
                match c {
                    (1.., _) => a,
                    (_, 0) => b,
                    _ => (a.0.min(b.0), a.1.max(b.1)),
                }
            }
            E::Given(_) => (0, u32::MAX),
        }
    }

    fn emit(&self, out: &mut Vec<u32>) {
        let op = |out: &mut Vec<u32>, a: &E, b: &E, code: u32| {
            a.emit(out);
            b.emit(out);
            out.push(code);
        };
        match self {
            E::C(k) if *k < 1 << 24 => out.push(1 | k << 8),
            E::C(k) => out.extend([2, *k]),
            E::At(w) => out.push(3 | (*w as u32) << 8),
            E::Add(a, b) => op(out, a, b, 4),
            E::Sub(a, b) => op(out, a, b, 5),
            E::Mul(a, b) => op(out, a, b, 6),
            E::Div(a, b) => op(out, a, b, 7),
            E::Min(a, b) => op(out, a, b, 8),
            E::Max(a, b) => op(out, a, b, 9),
            E::Gt(a, b) => op(out, a, b, 10),
            E::Floor(a, b) => op(out, a, b, 12),
            E::Sel(c, a, b) => {
                c.emit(out);
                a.emit(out);
                b.emit(out);
                out.push(11);
            }
            E::Given(k) => out.push(13 | k << 8),
        }
    }
}

/// Where a program stores its word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Dst {
    Census(usize),
    Cfg(u32),
    /// A census word set where the value is not none, else left: a flag
    /// any number of programs at a point may raise.
    Raise(usize),
}

/// `e` compiled to store into `dst`.
pub(super) fn program(dst: Dst, e: &E) -> Vec<u32> {
    let mut out = Vec::new();
    e.emit(&mut out);
    let store = match dst {
        Dst::Census(w) => 20 | (w as u32) << 8,
        Dst::Cfg(w) => 21 | w << 8,
        Dst::Raise(w) => 23 | (w as u32) << 8,
    };
    assert!(match dst { Dst::Cfg(w) => w < 1 << 24, Dst::Census(w) | Dst::Raise(w) => w < 1 << 24 }, "a word the program can name");
    out.extend([store, 0]);
    out
}

pub(super) struct Kernels {
    pub pass: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        Self {
            pass: io::Pass::new(gpu, VM.to_string(), &[true, false, false, false], &["vm_main"]),
        }
    }
}

/// The WGSL, for the shader checks.
pub(super) fn source() -> String {
    VM.to_string()
}
