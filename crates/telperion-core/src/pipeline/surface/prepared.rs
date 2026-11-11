//! A run of rings as the sweep lays it, and its triangles in the CPU
//! builder's order.
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Run {
    pub base: u32,
    pub first_index: u32,
    pub ring_start: u32,
    pub rings: u32,
    pub index_count: u32,
}
impl Run {
    #[inline]
    pub(super) fn visit_triangles(
        &self,
        segments: u32,
        mut visit: impl FnMut([u32; 3]) -> Result<()>,
    ) -> Result<()> {
        for ring in 0..self.rings - 1 {
            let lower = self.base + ring * segments;
            let upper = lower + segments;
            for k in 0..segments {
                let next = (k + 1) % segments;
                visit([lower + k, lower + next, upper + k])?;
                visit([lower + next, upper + next, upper + k])?;
            }
        }
        let bottom = self.base + self.rings * segments;
        let top = bottom - segments;
        for k in 0..segments {
            let next = (k + 1) % segments;
            visit([bottom, self.base + next, self.base + k])?;
            visit([bottom + 1, top + k, top + next])?;
        }
        Ok(())
    }

    /// Triangle in the CPU builder's strip-then-interleaved-cap order.
    pub fn triangle(&self, face: u32, segments: u32) -> [u32; 3] {
        let strip = (self.rings - 1) * segments * 2;
        if face < strip {
            let pair = face / 2;
            let lower = self.base + pair / segments * segments;
            let k = pair % segments;
            let next = (k + 1) % segments;
            if face % 2 == 0 {
                [lower + k, lower + next, lower + segments + k]
            } else {
                [lower + next, lower + segments + next, lower + segments + k]
            }
        } else {
            let cap = face - strip;
            let k = cap / 2;
            let next = (k + 1) % segments;
            let bottom = self.base + self.rings * segments;
            if cap % 2 == 0 {
                [bottom, self.base + next, self.base + k]
            } else {
                let top = bottom - segments;
                [bottom + 1, top + k, top + next]
            }
        }
    }
}

#[cfg(test)]
mod traversal_tests {
    use super::*;

    #[test]
    fn nested_faces_match_procedural_order() {
        for (base, rings, segments) in [(0, 2, 3), (17, 4, 8), (101, 1025, 17)] {
            let run = Run {
                base,
                rings,
                first_index: 33,
                ring_start: 7,
                index_count: rings * segments * 6,
            };
            let mut face = 0;
            run.visit_triangles(segments, |triangle| {
                assert_eq!(triangle, run.triangle(face, segments));
                face += 1;
                Ok(())
            })
            .unwrap();
            assert_eq!(face, run.index_count / 3);
        }
    }
}
