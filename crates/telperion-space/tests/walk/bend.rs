//! The scale `form.sag` is walked on.
use telperion_space::{Form, Origin, Structure, Vec3};

/// The radians a unit of sag bends the most loaded axis of `pa`, before
/// it eases: the sum over its phytomers of the moment they carry over
/// their radius to the fourth, times their length. The load is the wood
/// and foliage beyond each phytomer, as `sag.rs` weighs it, at the walk
/// tree's default pipe and no ripening.
pub fn sag_bend(tree: &Structure, pa: usize) -> f64 {
    let pipe = Form::default().pipe;
    let n = tree.axes.len();
    // Each axis's phytomers' masses and centres, base to tip.
    let parts: Vec<Vec<(f64, Vec3)>> = tree
        .axes
        .iter()
        .map(|a| {
            let mut from = a.base;
            a.phytomers
                .iter()
                .map(|p| {
                    let length = (p.tip - from).length();
                    let leaf = pipe * p.scale;
                    let part = (
                        length * (p.radius * p.radius + leaf * leaf),
                        (from + p.tip) * 0.5,
                    );
                    from = p.tip;
                    part
                })
                .collect()
        })
        .collect();
    // Which axes each axis bears, and at which node (its tip for a
    // continuation or relay).
    let mut borne: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (i, a) in tree.axes.iter().enumerate() {
        match a.origin {
            Origin::Lateral { parent, node, .. } => borne[parent].push((i, node)),
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                let tip = tree.axes[parent].phytomers.len();
                borne[parent].push((i, tip))
            }
            Origin::Seed => {}
        }
    }
    // Every mass an axis carries from its node `k` on.
    fn carried(
        i: usize,
        k: usize,
        parts: &[Vec<(f64, Vec3)>],
        borne: &[Vec<(usize, usize)>],
        out: &mut Vec<(f64, Vec3)>,
    ) {
        out.extend_from_slice(&parts[i][k..]);
        for &(c, node) in &borne[i] {
            if node >= k {
                carried(c, 0, parts, borne, out);
            }
        }
    }
    let mut most = 0.0f64;
    for (i, a) in tree.axes.iter().enumerate().filter(|(_, a)| a.pa == pa) {
        let mut from = a.base;
        let mut sum = 0.0;
        for (k, p) in a.phytomers.iter().enumerate() {
            let mut masses = Vec::new();
            carried(i, k, &parts, &borne, &mut masses);
            let (mx, my) = masses.iter().fold((0.0, 0.0), |(x, y), (m, c)| {
                (x + m * (c.x - from.x), y + m * (c.y - from.y))
            });
            if p.radius > 0.0 {
                sum += mx.hypot(my) / p.radius.powi(4) * (p.tip - from).length();
            }
            from = p.tip;
        }
        most = most.max(sum);
    }
    most
}
