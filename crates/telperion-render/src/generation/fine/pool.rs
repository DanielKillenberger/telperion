//! Buffers kept from one build to the next (fn-209 F1 task 8). The
//! driver takes about 3.6 ms to create a large buffer, whatever its size:
//! eleven of them, 40 ms, in a spruce at 80. A build takes its packed
//! axes and phytomers, its geometry scratch and its laid waves from the
//! pool by size class and gives them back when it is done, so a tree
//! built again, as a retune builds it, creates none. A buffer taken
//! again is cleared, as a new one is, so a build reads what it would
//! have read in a new buffer (the geometry's status words are read
//! before they are written).
use super::steps::buffer;
use crate::buffer::Owned;
use crate::Gpu;
use std::cell::RefCell;

#[derive(Default)]
pub(super) struct Pool(RefCell<Vec<Owned>>);

/// `bytes` rounded up to its size class: a power of two's eighths, in
/// whole words, so a class wastes at most an eighth (a word's bytes when
/// small) and a tree a little larger or smaller takes the same buffers.
fn class(bytes: u64) -> u64 {
    let bytes = bytes.max(16);
    let step = 1u64 << (63 - bytes.leading_zeros()).saturating_sub(3).max(2);
    bytes.next_multiple_of(step)
}

impl Pool {
    /// A storage buffer of at least `words`, cleared: its class's from
    /// the pool where one is free.
    pub(super) fn take(&self, gpu: &Gpu, label: &'static str, words: u64) -> Owned {
        let bytes = class(words.max(4) * 4);
        let mut free = self.0.borrow_mut();
        let Some(k) = free.iter().position(|b| b.size() == bytes) else {
            return buffer(gpu, label, bytes / 4);
        };
        let taken = free.swap_remove(k);
        let mut encoder = gpu.encoder(&Default::default());
        encoder.clear_buffer(&taken, 0, None);
        gpu.submit([encoder.finish()]);
        taken
    }

    /// Buffers a build is done with, for the next; those the last build
    /// left and this one did not take are let go, so the pool holds one
    /// build's buffers at most.
    /// The bytes the pool keeps between builds.
    #[cfg(test)]
    pub(super) fn bytes(&self) -> u64 {
        self.0.borrow().iter().map(|b| b.size()).sum()
    }

    pub(super) fn give(&self, buffers: impl IntoIterator<Item = Owned>) {
        *self.0.borrow_mut() = buffers.into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::class;

    #[test]
    fn a_class_holds_its_bytes_in_words_and_wastes_at_most_an_eighth() {
        for bytes in [1, 16, 17, 100, 4096, 40 << 20, (40 << 20) + 1, 251 << 20] {
            let c = class(bytes);
            let b = bytes.max(16);
            assert!(c >= b && c - b <= (b / 8).max(3), "{bytes}: {c}");
            assert_eq!(c % 4, 0);
        }
    }
}
