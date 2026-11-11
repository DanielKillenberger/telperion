//! Named arrays a cycle's capture fills (`grow_capture_light_at`).

/// The captured arrays, by name, each a kind and its little-endian bytes.
pub struct Capture {
    pub arrays: Vec<(String, u8, Vec<u8>)>,
}

macro_rules! put {
    ($c:expr, $name:expr, $kind:expr, $v:expr) => {
        $c.arrays.push((
            $name.to_string(),
            $kind,
            $v.iter().flat_map(|x| x.to_le_bytes()).collect(),
        ))
    };
}

impl Capture {
    pub(super) fn u32s(&mut self, name: &str, v: &[u32]) {
        put!(self, name, 1, v);
    }
    pub(super) fn u64s(&mut self, name: &str, v: &[u64]) {
        put!(self, name, 2, v);
    }
    pub(super) fn f32s(&mut self, name: &str, v: &[f32]) {
        put!(self, name, 3, v);
    }

    /// The array named `name`: its kind and its bytes.
    pub fn array(&self, name: &str) -> Option<(u8, &[u8])> {
        (self.arrays.iter())
            .find(|a| a.0 == name)
            .map(|a| (a.1, &a.2[..]))
    }

    /// The capture as one file's bytes: per array its name's length and
    /// name, its kind, its byte count and its bytes.
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = b"FN256CAP".to_vec();
        for (name, kind, data) in &self.arrays {
            out.extend((name.len() as u32).to_le_bytes());
            out.extend(name.as_bytes());
            out.push(*kind);
            out.extend((data.len() as u64).to_le_bytes());
            out.extend(data);
        }
        out
    }
}
