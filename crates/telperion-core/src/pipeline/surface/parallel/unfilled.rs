//! An output buffer the workers fill in place: each worker owns one part and
//! writes every element of it once, in order, and nothing is zeroed first.
use super::{failed, reserved, Result};
use std::{
    mem::MaybeUninit,
    sync::atomic::{AtomicUsize, Ordering},
};

pub(super) struct Unfilled<T> {
    vec: Vec<T>,
    len: usize,
    /// How far the parts are handed out; they never overlap.
    taken: usize,
    /// Elements the parts wrote, summed as each part is dropped.
    written: AtomicUsize,
}

pub(super) struct Parts<'a, T> {
    rest: &'a mut [MaybeUninit<T>],
    taken: &'a mut usize,
    written: &'a AtomicUsize,
}

/// One worker's part: written front to back, counted when dropped.
pub(super) struct Part<'a, T> {
    out: &'a mut [MaybeUninit<T>],
    at: usize,
    written: &'a AtomicUsize,
}

impl<T: Copy> Unfilled<T> {
    pub(super) fn new(len: usize) -> Result<Self> {
        Ok(Self {
            vec: reserved(len)?,
            len,
            taken: 0,
            written: AtomicUsize::new(0),
        })
    }

    /// The parts not yet handed out, from the front.
    pub(super) fn parts(&mut self) -> Parts<'_, T> {
        let rest = &mut self.vec.spare_capacity_mut()[self.taken..self.len];
        Parts {
            rest,
            taken: &mut self.taken,
            written: &self.written,
        }
    }

    /// The buffer, where every part was handed out and written in full.
    pub(super) fn filled(self) -> Result<Vec<T>> {
        if self.written.into_inner() != self.len {
            return Err(failed());
        }
        let mut vec = self.vec;
        // SAFETY: the parts tile `0..taken` without overlap, each counts
        // only the prefix it wrote, and prefixes sum to `len` only when
        // `taken == len` and every part was written in full.
        unsafe { vec.set_len(self.len) };
        Ok(vec)
    }
}

impl<'a, T> Parts<'a, T> {
    /// The next `n` elements, or an error past the buffer's end.
    pub(super) fn take(&mut self, n: usize) -> Result<Part<'a, T>> {
        let rest = std::mem::take(&mut self.rest);
        let (out, rest) = rest.split_at_mut_checked(n).ok_or_else(failed)?;
        self.rest = rest;
        *self.taken += n;
        Ok(Part {
            out,
            at: 0,
            written: self.written,
        })
    }
}

impl<T: Copy> Part<'_, T> {
    /// Writes `values` next; past the part's end it panics before writing.
    #[inline]
    pub(super) fn extend(&mut self, values: &[T]) {
        let out = &mut self.out[self.at..self.at + values.len()];
        for (slot, &v) in out.iter_mut().zip(values) {
            slot.write(v);
        }
        self.at += values.len();
    }
}

impl<T> Drop for Part<'_, T> {
    fn drop(&mut self) {
        self.written.fetch_add(self.at, Ordering::Relaxed);
    }
}
