//! Phase (iv)'s light in the stepped executor: every grower's leaves of
//! the cycle into one lattice, swept once, each grower reading its own
//! buds and each standing root its centroid (`stands.rs`).
use super::Executor;
use crate::light::Field;
use crate::structure::Vec3;

impl Executor<'_> {
    /// Every grower's leaves, the coarse grower's first and then the items'
    /// in the order they are visited, into one lattice, swept once; each
    /// grower reads its own buds and each standing root its centroid.
    pub(super) fn shade(&mut self, cycle: u32) {
        let order = self.visit();
        let threads = self.coarse.threads;
        let light = self.coarse.sketch.as_ref().expect("sketching").light;
        let growers: Vec<usize> = std::iter::once(usize::MAX).chain(order).collect();
        let (mut leaves, mut tips): (Vec<(Vec3, f64)>, Vec<Vec3>) = (Vec::new(), Vec::new());
        for &k in &growers {
            let g = self.grower(k);
            leaves.extend_from_slice(&g.leaves);
            tips.extend(g.live.iter().map(|a| g.tip(a.axis)));
        }
        let (stood, at) = self.stand_leaves(cycle);
        leaves.extend(stood);
        tips.extend(at);
        let field = Field::on(&light, &leaves, &tips, threads);
        let mut read = vec![0.0; tips.len()];
        crate::cores::fill(&mut read, 1 << 10, threads, &|k| field.at(tips[k]));
        let mut from = 0;
        for &k in &growers {
            let g = self.grower_mut(k);
            let n = g.live.len();
            let sketch = g.sketch.as_mut().expect("sketching");
            for (apex, &light) in g.live.iter().zip(&read[from..from + n]) {
                sketch.pencils[apex.axis].light = light;
            }
            from += n;
        }
        self.stand_read(&read[from..]);
    }
}
