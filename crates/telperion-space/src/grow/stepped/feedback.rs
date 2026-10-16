//! Phase (iv)'s light in the stepped executor: the coarse grower's leaves
//! of the cycle and the standing roots' into one lattice, swept once, the
//! grower reading its own buds and each standing root its centroid
//! (`stands.rs`).
use super::Executor;
use crate::light::Field;

impl Executor<'_> {
    /// The coarse grower's leaves, then the stands', into one lattice,
    /// swept once; the grower reads its own buds and each standing root
    /// its centroid.
    pub(super) fn shade(&mut self, cycle: u32) {
        let threads = self.coarse.threads;
        let g = &self.coarse;
        let light = g.sketch.as_ref().expect("sketching").light;
        let mut leaves = g.leaves.clone();
        let mut tips: Vec<_> = g.live.iter().map(|a| g.tip(a.axis)).collect();
        let (stood, at) = self.stand_leaves(cycle);
        leaves.extend(stood);
        tips.extend(at);
        let field = Field::on(&light, &leaves, &tips, threads);
        let mut read = vec![0.0; tips.len()];
        crate::cores::fill(&mut read, 1 << 10, threads, &|k| field.at(tips[k]));
        let g = &mut self.coarse;
        let n = g.live.len();
        let sketch = g.sketch.as_mut().expect("sketching");
        for (apex, &light) in g.live.iter().zip(&read[..n]) {
            sketch.pencils[apex.axis].light = light;
        }
        self.stand_read(&read[n..]);
    }
}
