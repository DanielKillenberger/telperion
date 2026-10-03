//! The probe's tree as a pipeline tree, and the scorecard's own wood classes:
//! wood at 0.05 of the root's radius or more is structural, on every tree
//! measured, so today's tree and the law's are classed by one rule.
use crate::law::Grown;
use telperion_core::math::Vec3;
use telperion_core::tree::{BudFate, Node, NodeKind, ShootState, Tree};

pub const STRUCTURAL: f64 = 0.05;

/// Reclasses a tree's wood by the scorecard's rule; finest wood on an
/// unbranched tip is twig, the rest branch.
pub fn reclass(t: &mut Tree) {
    let root = t.nodes[0].radius;
    let n = t.nodes.len();
    let mut kids = vec![0u32; n];
    for x in &t.nodes[1..] {
        kids[x.parent.unwrap() as usize] += 1;
    }
    for i in 0..n {
        let r = t.nodes[i].radius;
        t.nodes[i].kind = if i == 0 || r >= STRUCTURAL * root {
            NodeKind::Structural
        } else if kids[i] == 0 {
            NodeKind::Twig
        } else {
            NodeKind::Branch
        };
    }
}

pub fn to_tree(g: &Grown) -> Tree {
    let n = g.pos.len();
    let mut branch = vec![0u32; n];
    let mut nodes = Vec::with_capacity(n);
    for i in 0..n {
        let p = g.parent[i];
        branch[i] = match p {
            Some(p) if !g.lateral[i] => branch[p],
            _ => i as u32,
        };
        let r = g.radius[i];
        let start = match p {
            Some(p) if !g.lateral[i] => g.radius[p].max(r),
            _ => r,
        };
        let mut node = Node::root();
        node.shoot = ShootState { bud_fate: if g.lateral[i] { BudFate::Lateral } else { BudFate::Terminal } };
        node.position = g.pos[i];
        node.parent = p.map(|p| p as u32);
        node.radius = r;
        node.start_radius = start;
        node.base_radius = g.radius[branch[i] as usize];
        node.branch = branch[i];
        nodes.push(node);
    }
    // Stem: the root's terminal line.
    let mut stem = vec![false; n];
    for i in 1..n {
        let p = g.parent[i].unwrap();
        stem[i] = !g.lateral[i] && (p == 0 || stem[p]);
    }
    for i in 1..n {
        nodes[i].stem = stem[i];
    }
    let mut t = Tree { nodes, crossover: n, diagnostics: Default::default(), sections: vec![], pipe: vec![] };
    reclass(&mut t);
    t
}

fn pct(v: &[f64], q: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let mut v = v.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    v[((v.len() - 1) as f64 * q).round() as usize]
}

/// Fine wood (R3-PROBE8's diagnosis on the scorecard's classes): laterals
/// per metre, generations from structural wood, terminal runs; and the
/// spruce's first-order branch elevation.
pub fn fine(t: &Tree) -> serde_json::Value {
    let n = &t.nodes;
    let len = n.len();
    let mut kids = vec![0u32; len];
    for x in &n[1..] {
        kids[x.parent.unwrap() as usize] += 1;
    }
    let seg = |i: usize| n[i].parent.map_or(0.0, |p| (n[i].position - n[p as usize].position).length());
    let lateral = |i: usize| n[i].shoot.bud_fate == BudFate::Lateral || n[i].codominant.is_some();
    let local = |i: usize| n[i].kind != NodeKind::Structural;
    let mut cont = vec![usize::MAX; len];
    for i in 1..len {
        let p = n[i].parent.unwrap() as usize;
        if !lateral(i) && cont[p] == usize::MAX {
            cont[p] = i;
        }
    }
    let (mut gen, mut local_m, mut lats) = (vec![0u32; len], 0.0, 0usize);
    for i in 1..len {
        let p = n[i].parent.unwrap() as usize;
        gen[i] = if local(i) { gen[p] + u32::from(lateral(i) || !local(p)) } else { 0 };
        if local(i) {
            local_m += seg(i);
            lats += (kids[i] as usize).saturating_sub(usize::from(cont[i] != usize::MAX));
        }
    }
    let gens: Vec<f64> = (1..len).filter(|&i| local(i) && kids[i] == 0).map(|i| f64::from(gen[i])).collect();
    // Elevation of first-order branches: laterals off the root's terminal
    // line whose axis reaches 0.05 H; the chord from the parent node.
    let h = n.iter().map(|x| x.position.y).fold(0.0, f64::max);
    let mut on_stem = vec![false; len];
    on_stem[0] = true;
    let mut j = 0;
    while cont[j] != usize::MAX {
        j = cont[j];
        on_stem[j] = true;
    }
    let mut elev = vec![];
    let mut lowest = h;
    for i in 1..len {
        let p = n[i].parent.unwrap() as usize;
        if !(lateral(i) && on_stem[p]) {
            continue;
        }
        let (mut e, mut l) = (i, seg(i));
        while cont[e] != usize::MAX {
            e = cont[e];
            l += seg(e);
        }
        if l < 0.05 * h {
            continue;
        }
        lowest = lowest.min(n[p].position.y);
        let d: Vec3 = n[e].position - n[p].position;
        elev.push(d.y.atan2(d.x.hypot(d.z)).to_degrees());
    }
    let steep = elev.iter().filter(|&&a| a > 45.0).count() as f64 / elev.len().max(1) as f64;
    let wood: f64 = (1..len).map(seg).sum();
    let root = n[0].radius;
    let under = |k: f64| (1..len).filter(|&i| n[i].radius < k * root).map(seg).sum::<f64>();
    let r3 = |x: f64| (x * 1000.0).round() / 1000.0;
    serde_json::json!({
        "nodes": len, "wood_m": r3(wood), "fine_m": r3(local_m), "fine_m_004": r3(under(0.04)), "fine_m_006": r3(under(0.06)),
        "laterals_per_m": r3(lats as f64 / local_m.max(1e-9)),
        "generations_p50_p90_max": [pct(&gens, 0.5), pct(&gens, 0.9), pct(&gens, 1.0)],
        "lowest_lateral_axis": r3(lowest / h.max(1e-9)), "elevation_median": r3(pct(&elev, 0.5)), "elevation_steep_share": r3(steep), "elevation_n": elev.len(),
    })
}
