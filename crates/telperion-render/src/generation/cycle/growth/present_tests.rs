//! fn-270 R5 and R5a on the device: every young preset drawn at
//! fractions of its last year is the CPU's presentation, node by node by
//! lineage within the lineage tolerance, each node's leaf share with it,
//! and its whole year is the tree grown.
use super::super::super::resident::agree_tests::{drawn_nodes, Node};
use super::super::grown_tests::YOUNG;
use super::tests::block;
use super::*;
use std::collections::HashMap;
use telperion_core::pipeline::{executor, growth};

/// The most apart a position (m), a radius (m) and a leaf share stood,
/// and the nodes drawn, matched and their leaves still expanding.
#[derive(Debug, Default)]
struct Apart {
    device: usize,
    cpu: usize,
    matched: usize,
    position: f64,
    radius: f64,
    leaf: f64,
    unfolding: usize,
    /// Nodes whose retentions, or whose season, differ from the CPU's.
    retention: usize,
    season: usize,
}

fn apart(dev: &[Node], tree: &telperion_core::tree::Tree) -> Apart {
    let (mut by_key, mut by_identity) = (HashMap::new(), HashMap::new());
    for i in 1..tree.nodes.len() {
        by_key.insert(tree.key(i), i);
        by_identity.insert(tree.nodes[i].identity.birth_order(), i);
    }
    let mut out = Apart { device: dev.len(), cpu: tree.nodes.len() - 1, ..Apart::default() };
    for d in dev {
        let found = match d.identity {
            Some(k) => by_identity.get(&k),
            None => by_key.get(&d.key),
        };
        let Some(&i) = found else { continue };
        let n = &tree.nodes[i];
        let p = [n.position.x, n.position.y, n.position.z];
        let r = [n.radius, n.start_radius, n.base_radius];
        let most = |a: [f32; 3], b: [f64; 3]| (0..3).map(|c| (f64::from(a[c]) - b[c]).abs()).fold(0.0, f64::max);
        out.matched += 1;
        out.position = out.position.max(most(d.position, p));
        out.radius = out.radius.max(most(d.radii, r));
        let u = tree.unfolding(i);
        out.leaf = out.leaf.max((f64::from(d.leaf) - f64::from(u.grow)).abs());
        out.unfolding += usize::from(d.leaf < 1.0);
        let l = tree.leafing(i);
        out.retention += usize::from(!tree.leafing.is_empty() && d.retention != (l.keep, l.was));
        let (fall, colour, regrow, life) = d.season;
        let shares = [(fall, u.fall), (colour, u.colour), (regrow, u.regrow)];
        let apart = shares.iter().any(|(a, b)| (a - b).abs() > 1e-4) || life.abs_diff(u32::from(u.life)) > 1;
        out.season += usize::from(!tree.unfolding.is_empty() && apart);
    }
    out
}

#[test]
fn a_tree_drawn_within_its_year_on_the_device_is_the_cpus() {
    let _held = super::super::super::whole_tree();
    let Some(gpu) = super::super::super::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    for (preset, name, h) in YOUNG {
        let mut family = preset.parameters();
        family.canopy.leaf_bases = 0;
        let g = executor::growth(&family).expect("a growth");
        let seed = g.request.seed;
        let params = growth::Params::of(&family).expect("params");
        let mut cpu = growth::begin(&params, seed as u32, f64::from(h)).expect("begun");
        growth::grow_to(&mut cpu, f64::from(h)).expect("grown");
        let mut dev = GrowthState::sized_now(&gpu, &k, &g, (seed, h));
        block(dev.grow_to(&gpu, &k, f64::from(h))).expect("grown");
        for f in [0.02, 0.35, 0.6, 0.88, 0.95] {
            let at = f64::from(h) - 1.0 + f;
            let (dressed, _) = block(dev.settle(&gpu, &k, at)).expect("drawn");
            let a = apart(&drawn_nodes(&gpu, &dressed), &growth::settle(&cpu, at).expect("drawn"));
            println!("{name} {h} at {f}: {a:?}");
            assert_eq!(a.device, a.cpu, "{name} at {f}: nodes");
            assert!(a.matched * 1000 >= a.device * 999, "{name} at {f}: {a:?}");
            assert!(a.position < 1e-3 && a.radius < 1e-5 && a.leaf < 1e-3, "{name} at {f}: {a:?}");
            assert_eq!((a.retention, a.season), (0, 0), "{name} at {f}: retentions and season");
        }
    }
}

