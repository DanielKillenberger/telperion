//! Every artifact the one build makes for a preset, written as its debug
//! text to stdout, so two builds compare byte for byte by a digest of it.
//! The skeleton is written as what a consumer reads of it, never as the
//! `Tree` it is held in, so a change to a field no consumer reads keeps
//! the digest.
//!
//!   cargo run --profile ci -p telperion-core --example artifact_digest -- <preset> [seed] | sha256sum
use std::fmt::Debug;
use telperion_core::{
    pipeline::{build, FieldRequest, Request},
    presets::Preset,
    tree::Tree,
};

/// The skeleton as its consumers read it: the nodes and every row drawn
/// beside them.
fn skeleton(t: &Tree) -> impl Debug + '_ {
    (
        (&t.nodes, t.crossover, t.diagnostics, &t.sections, &t.frames),
        (&t.keys, &t.grown, &t.blooms, &t.clusters, &t.clustered),
        (&t.leafing, &t.unfolding, t.year),
    )
}

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let id = args
        .next()
        .ok_or("usage: artifact_digest <preset> [seed]")?;
    let mut family = Preset::from_id(&id)
        .ok_or(format!("no preset {id}"))?
        .parameters();
    if let Some(seed) = args.next() {
        family.skeleton.seed = seed.parse().map_err(|_| format!("no seed {seed}"))?;
    }
    let request = Request {
        wood: true,
        leaves: true,
        field: Some(FieldRequest {
            n: 36,
            limb_order: None,
        }),
        structure: true,
        ..Request::default()
    };
    let built = build(&family, request).map_err(|e| e.to_string())?;
    let o = &built.outputs;
    println!("skeleton {:?}", skeleton(&built.skeleton.tree));
    println!("element {:?}", o.element);
    println!("wood {:?}", o.wood);
    let leaves = o
        .leaves
        .as_ref()
        .map(|l| (&l.instances, l.placed, l.retained));
    println!("leaves {leaves:?}");
    println!("structure {:?}", o.structure);
    println!("field {:?}", o.field);
    Ok(())
}
