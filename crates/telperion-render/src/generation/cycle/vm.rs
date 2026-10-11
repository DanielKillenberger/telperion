//! The words the device computes from its counts (`vm.wgsl`): an
//! expression over the census, compiled to a short program that stores
//! one word, each program one invocation at its point of the cycle.
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
            E::Div(a, b) => { let (a, b) = bin(a, b); if b == 0 { 0 } else { (a + b - 1) / b } }
            E::Min(a, b) => { let (a, b) = bin(a, b); a.min(b) }
            E::Max(a, b) => { let (a, b) = bin(a, b); a.max(b) }
            E::Gt(a, b) => { let (a, b) = bin(a, b); u32::from(a > b) }
            E::Floor(a, b) => { let (a, b) = bin(a, b); a.checked_div(b).unwrap_or(0) }
            E::Sel(c, a, b) => if c.eval(census) != 0 { a.eval(census) } else { b.eval(census) },
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
        }
    }
}

/// Where a program stores its word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Dst {
    Census(usize),
    Cfg(u32),
    Args(u32),
}

/// `e` compiled to store into `dst`.
pub(super) fn program(dst: Dst, e: &E) -> Vec<u32> {
    let mut out = Vec::new();
    e.emit(&mut out);
    let store = match dst {
        Dst::Census(w) => 20 | (w as u32) << 8,
        Dst::Cfg(w) => 21 | w << 8,
        Dst::Args(w) => 22 | w << 8,
    };
    assert!(match dst { Dst::Cfg(w) | Dst::Args(w) => w < 1 << 24, Dst::Census(w) => w < 1 << 24 }, "a word the program can name");
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
