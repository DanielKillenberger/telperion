//! The capture file `telperion_space::grow_capture` writes, read back.
use std::collections::HashMap;

pub struct Cap {
    arrays: HashMap<String, Vec<u8>>,
}

impl Cap {
    pub fn read(path: &str) -> Self {
        let bytes = std::fs::read(path).expect("a capture");
        assert_eq!(&bytes[..8], b"FN256CAP", "a capture file");
        let mut arrays = HashMap::new();
        let mut at = 8;
        let word = |at: usize, n: usize| -> u64 {
            let mut b = [0u8; 8];
            b[..n].copy_from_slice(&bytes[at..at + n]);
            u64::from_le_bytes(b)
        };
        while at < bytes.len() {
            let len = word(at, 4) as usize;
            let name = String::from_utf8(bytes[at + 4..at + 4 + len].to_vec()).expect("a name");
            at += 4 + len + 1;
            let size = word(at, 8) as usize;
            at += 8;
            arrays.insert(name, bytes[at..at + size].to_vec());
            at += size;
        }
        Self { arrays }
    }

    fn get(&self, name: &str) -> &[u8] {
        self.arrays
            .get(name)
            .unwrap_or_else(|| panic!("{name} captured"))
    }

    pub fn u32s(&self, name: &str) -> Vec<u32> {
        bytemuck::pod_collect_to_vec(self.get(name))
    }
    pub fn u64s(&self, name: &str) -> Vec<u64> {
        bytemuck::pod_collect_to_vec(self.get(name))
    }
    pub fn f32s(&self, name: &str) -> Vec<f32> {
        bytemuck::pod_collect_to_vec(self.get(name))
    }
    pub fn i32s(&self, name: &str) -> Vec<i32> {
        bytemuck::pod_collect_to_vec(self.get(name))
    }
}
