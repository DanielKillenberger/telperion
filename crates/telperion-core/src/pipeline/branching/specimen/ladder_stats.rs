//! fn-183 R1 exploration, not for merge: whole-run excursion of every
//! twig-layer axis beyond the lobed outline (and a shortened system's mapped
//! shell), node density per crown volume, and a dropping curtain's band.
use super::*;

/// How far `p` stands outside `shell`, in metres: radially past the lobed
/// radius at its height, or above the top.
fn outside(shell: &Envelope, seed: u32, p: Vec3) -> f64 {
    let radial = p.x.hypot_fixed(p.z) - shell.radius_toward(p, seed);
    radial.max(p.y - shell.height).max(0.0)
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    sorted[((sorted.len() - 1) as f64 * q).round() as usize]
}

impl Specimen {
    pub(super) fn ladder_stats(&self) -> String {
        let tree = &self.tree;
        let shell = self.params.envelope;
        let seed = self.params.seed;
        let t = self.params.twigs.resolved().unwrap();
        let drops = t.hang > 0.0 && t.curtain_drop > 0.0;
        let limbs = self.scaffold.limbs();
        let widest = shell.max_radius();
        let first = tree.crossover.min(tree.nodes.len());
        let mut axis = std::collections::HashMap::<u32, (f64, bool, u8)>::new();
        let mut station = vec![usize::MAX; tree.nodes.len()];
        let (mut band, mut lowest) = (0usize, f64::INFINITY);
        for i in first..tree.nodes.len() {
            let n = &tree.nodes[i];
            let parent = n.parent.unwrap() as usize;
            let id = if n.kind == NodeKind::Twig
                && n.shoot.bud_fate == crate::tree::BudFate::Terminal
                && parent >= first
            {
                tree.nodes[parent].branch
            } else {
                n.branch
            };
            station[i] = if parent < first {
                parent
            } else {
                station[parent]
            };
            let stem = tree.nodes[station[i]].stem;
            let p = n.position;
            let e = if drops && super::local::in_band(&shell, &t, seed, p, 1e-9) {
                band += 1;
                lowest = lowest.min(p.y / shell.height);
                0.0
            } else {
                let bound = limbs.of(tree, i);
                let mapped = if bound.short() {
                    outside(&shell, seed, bound.map(p)) * bound.scale()
                } else {
                    0.0
                };
                outside(&shell, seed, p).max(mapped) / widest
            };
            let slot = axis.entry(id).or_insert((0.0, stem, 0));
            if e > slot.0 {
                let base = shell.height * shell.crown_base;
                *slot = (
                    e,
                    stem,
                    if p.y > shell.height {
                        1
                    } else if p.y < base {
                        2
                    } else {
                        3
                    },
                );
            }
        }
        let on_stem = |s: bool| {
            let v: Vec<f64> = axis.values().filter(|a| a.1 == s).map(|a| a.0).collect();
            let out = v.iter().filter(|e| **e > 1e-9).count();
            let max = v.iter().copied().fold(0.0, f64::max);
            format!("[{},{},{:.4}]", v.len(), out, max)
        };
        let at = |w: u8| axis.values().filter(|a| a.0 > 1e-9 && a.2 == w).count();
        let mut out: Vec<f64> = axis.values().map(|a| a.0).filter(|e| *e > 1e-9).collect();
        out.sort_by(f64::total_cmp);
        let base = shell.height * shell.crown_base;
        let span = shell.height - base;
        let volume: f64 = (0..256)
            .map(|i| {
                shell
                    .radius_at(base + (i as f64 + 0.5) / 256.0 * span)
                    .powi(2)
            })
            .sum::<f64>()
            * std::f64::consts::PI
            * span
            / 256.0;
        format!(
            "{{\"nodes\":{},\"twig_layer_nodes\":{},\"axes\":{},\"axes_outside\":{},\
             \"share_outside\":{:.4},\"median\":{:.4},\"p95\":{:.4},\"max\":{:.4},\
             \"worst_above_below_beside\":[{},{},{}],\"stem_axes_out_max\":{},\"limb_axes_out_max\":{},\"band_nodes\":{},\"band_lowest\":{:.3},\"crown_m3\":{:.2},\"detail\":{:?}}}",
            tree.nodes.len(),
            tree.nodes.len() - first,
            axis.len(),
            out.len(),
            out.len() as f64 / axis.len().max(1) as f64,
            quantile(&out, 0.5),
            quantile(&out, 0.95),
            out.last().copied().unwrap_or(0.0),
            at(1),
            at(2),
            at(3),
            on_stem(true),
            on_stem(false),
            band,
            if band > 0 { lowest } else { 0.0 },
            volume,
            tree.diagnostics.twig_detail,
        )
    }
}
