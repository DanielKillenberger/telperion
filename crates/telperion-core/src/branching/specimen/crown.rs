//! Crown geometry is indexed once per envelope, sampled lazily per live shoot.
use super::*;
use slotmap::SecondaryMap;

#[derive(Clone, Copy, Default)]
struct Bounds {
    lo: [f64; 2],
    hi: [f64; 2],
}
impl Bounds {
    fn gap(self, p: [f64; 2]) -> f64 {
        (self.lo[0] - p[0])
            .max(p[0] - self.hi[0])
            .max(self.lo[1] - p[1])
            .max(p[1] - self.hi[1])
            .max(0.0)
    }
}
#[derive(Clone, Default)]
pub(super) struct Crown {
    envelope: Option<Envelope>,
    generation: u64,
    profile: Vec<[f64; 2]>,
    bounds: Vec<Bounds>,
    samples: SecondaryMap<NodeKey, (u64, Vec3, f64)>,
    #[cfg(test)]
    pub evaluated: usize,
    #[cfg(test)]
    pub segments: usize,
}
impl Crown {
    pub fn prepare(&mut self, envelope: Envelope) {
        #[cfg(test)]
        {
            self.evaluated = 0;
            self.segments = 0;
        }
        if self.envelope == Some(envelope) {
            return;
        }
        self.envelope = Some(envelope);
        self.generation += 1;
        self.profile = envelope.profile();
        let count = self.profile.len() - 1;
        self.bounds.resize(count * 2, Bounds::default());
        for i in 0..count {
            let (a, b) = (self.profile[i], self.profile[i + 1]);
            self.bounds[count + i] = Bounds {
                lo: [a[0].min(b[0]), a[1].min(b[1])],
                hi: [a[0].max(b[0]), a[1].max(b[1])],
            };
        }
        for i in (1..count).rev() {
            let (a, b) = (self.bounds[i * 2], self.bounds[i * 2 + 1]);
            self.bounds[i] = Bounds {
                lo: [a.lo[0].min(b.lo[0]), a.lo[1].min(b.lo[1])],
                hi: [a.hi[0].max(b.hi[0]), a.hi[1].max(b.hi[1])],
            };
        }
    }
    fn nearest(&mut self, i: usize, p: [f64; 2], best: &mut f64) {
        // Conservative box rejection, with room for subtraction rounding.
        if self.bounds[i].gap(p) > *best * (1.0 + 1e-12) + 1e-12 {
            return;
        }
        let count = self.profile.len() - 1;
        if i >= count {
            #[cfg(test)]
            {
                self.segments += 1;
            }
            *best = best.min(distance_to_profile(
                &self.profile[i - count..=i - count + 1],
                p[0],
                p[1],
            ));
        } else {
            let (a, b) = (i * 2, i * 2 + 1);
            let (first, second) = if self.bounds[a].gap(p) <= self.bounds[b].gap(p) {
                (a, b)
            } else {
                (b, a)
            };
            self.nearest(first, p, best);
            self.nearest(second, p, best);
        }
    }
    pub fn exposure(&mut self, node: &Node) -> f64 {
        if let Some(&(generation, position, value)) = self.samples.get(node.identity.key) {
            if generation == self.generation && position == node.position {
                return value;
            }
        }
        #[cfg(test)]
        {
            self.evaluated += 1;
        }
        let envelope = self.envelope.unwrap();
        let radial = node.position.x.hypot_fixed(node.position.z);
        let mut depth = (envelope.radius_at(node.position.y) - radial).max(0.0);
        if depth > 0.0 {
            self.nearest(1, [radial, node.position.y], &mut depth);
        }
        let exposure = (1.0 - depth / envelope.max_radius().max(1e-9)).clamp(0.0, 1.0);
        self.samples.insert(
            node.identity.key,
            (self.generation, node.position, exposure),
        );
        exposure
    }
    pub fn retire(&mut self, key: NodeKey) {
        self.samples.remove(key);
    }
}
