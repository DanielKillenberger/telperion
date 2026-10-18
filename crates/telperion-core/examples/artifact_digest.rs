//! Every artifact the one build makes for a preset, written as its debug
//! text to stdout, so two builds compare byte for byte by a digest of it.
//!
//!   cargo run --profile ci -p telperion-core --example artifact_digest -- <preset> | sha256sum
use telperion_core::{
    pipeline::{build, FieldRequest, Request},
    presets::Preset,
};

fn main() -> Result<(), String> {
    let id = std::env::args()
        .nth(1)
        .ok_or("usage: artifact_digest <preset>")?;
    let family = Preset::from_id(&id)
        .ok_or(format!("no preset {id}"))?
        .parameters();
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
    println!("skeleton {:?}", built.skeleton.tree);
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
