//! fn-192 R1: every draw is keyed to the bud's lineage, its path from the
//! root, so adding a branch anywhere reshuffles nothing else.
use std::collections::HashMap;
use telperion_space::{grow, NodeLaw, Origin, PaState, Request, Species, Structure, Zone};

fn state(lifespan: u32, viability: f64, nodes: NodeLaw, lateral: &[f64]) -> PaState {
    PaState {
        lifespan,
        next: None,
        viability,
        zones: vec![Zone {
            nodes,
            buds: 2,
            lateral: lateral.to_vec(),
        }],
        shedding: None,
        internode: 0.5,
        insertion: 0.7,
        divergence: 2.4,
    }
}

/// The trunk's lateral probability `p`; its limbs are mortal and branch on.
fn species(p: f64) -> Species {
    Species {
        states: vec![
            state(8, 1.0, NodeLaw::Poisson { mean: 2.5 }, &[0.0, p, 0.0]),
            state(5, 0.85, NodeLaw::Uniform { min: 1, max: 3 }, &[0.0, 0.0, 0.5]),
            state(3, 0.9, NodeLaw::Poisson { mean: 1.5 }, &[0.0, 0.0, 0.0]),
        ],
    }
}

/// Each axis by its structural path: the parent's path, then the node and
/// slot it stands on, or the continuation.
fn by_path(tree: &Structure) -> HashMap<String, usize> {
    let mut paths: Vec<String> = Vec::with_capacity(tree.axes.len());
    for axis in &tree.axes {
        let path = match axis.origin {
            Origin::Seed => "s".to_string(),
            Origin::Lateral {
                parent, node, slot, ..
            } => format!("{}/{node}.{slot}", paths[parent]),
            Origin::Continuation { parent } => format!("{}/c", paths[parent]),
        };
        paths.push(path);
    }
    paths.into_iter().enumerate().map(|(i, p)| (p, i)).collect()
}

#[test]
fn adding_a_branch_reshuffles_nothing_else() {
    let request = |seed| Request {
        age: 8,
        seed,
        budget: 1_000_000,
    };
    let mut added = 0;
    for seed in 1..=8 {
        for step in 0..12 {
            let p = 0.2 + 0.05 * f64::from(step);
            let fewer = grow(&species(p), request(seed)).unwrap();
            let more = grow(&species(p + 0.05), request(seed)).unwrap();
            let ours = by_path(&more);
            added += more.axes.len() - fewer.axes.len();
            for (path, &i) in &by_path(&fewer) {
                let Some(&j) = ours.get(path) else {
                    panic!("seed {seed}, p {p:.2}: {path} vanished when branches were added");
                };
                let (a, b) = (&fewer.axes[i], &more.axes[j]);
                assert_eq!(a.pa, b.pa, "seed {seed}, p {p:.2}: {path} changed its PA");
                let cycles = |x: &telperion_space::Axis| {
                    x.phytomers.iter().map(|ph| ph.cycle).collect::<Vec<_>>()
                };
                assert_eq!(cycles(a), cycles(b), "seed {seed}, p {p:.2}: {path} regrew");
            }
        }
    }
    assert!(added > 100, "the walk adds branches ({added})");
}
