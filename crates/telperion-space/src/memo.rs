//! A memo's map: keys are a few integers and float bits the recursion
//! made, never input from outside, so a multiply-rotate hash (FxHash's)
//! serves where SipHash's flood resistance buys nothing (fn-209, P1).
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

pub(crate) type Memo<K, V> = HashMap<K, V, BuildHasherDefault<Fast>>;

#[derive(Default)]
pub(crate) struct Fast(u64);

impl Hasher for Fast {
    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut word = [0u8; 8];
            word[..chunk.len()].copy_from_slice(chunk);
            self.write_u64(u64::from_le_bytes(word));
        }
    }

    fn write_u64(&mut self, n: u64) {
        self.0 = (self.0.rotate_left(5) ^ n).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }

    fn write_usize(&mut self, n: usize) {
        self.write_u64(n as u64);
    }

    fn write_u32(&mut self, n: u32) {
        self.write_u64(u64::from(n));
    }

    fn write_u8(&mut self, n: u8) {
        self.write_u64(u64::from(n));
    }

    fn finish(&self) -> u64 {
        self.0
    }
}
