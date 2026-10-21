//! fn-301 .5 scratch: every R1 row (preset x ages 6..20 and default x seeds
//! 1, 7, 42), each artifact the one build makes, as artifact_digest writes
//! it, folded by DefaultHasher per artifact.
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Write};
use std::hash::Hasher;
use telperion_core::{
    pipeline::{build, FieldRequest, Request},
    presets::{self, CATALOGUE},
    tree::Tree,
};

struct H(DefaultHasher);
impl Write for H {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}
fn h(x: impl Debug) -> String {
    let mut w = H(DefaultHasher::new());
    write!(w, "{x:?}").unwrap();
    format!("{:016x}", w.0.finish())
}
fn skeleton(t: &Tree) -> impl Debug + '_ {
    (
        (&t.nodes, t.crossover, t.diagnostics, &t.sections, &t.frames),
        (&t.keys, &t.grown, &t.blooms, &t.clusters, &t.clustered),
        (&t.leafing, &t.unfolding, t.year),
    )
}
fn main() {
    let only: Option<String> = std::env::args().nth(1);
    for &(_, id, _, _) in CATALOGUE {
        if only.as_ref().is_some_and(|o| !o.split(',').any(|x| x == id)) {
            continue;
        }
        let default = presets::by_identity(id).unwrap().age;
        for age in [6., 8., 10., 12., 15., 20., default] {
            for seed in [1u32, 7, 42] {
                let mut f = presets::by_identity(id).unwrap();
                (f.age, f.skeleton.seed) = (age, seed);
                let request = Request {
                    wood: true,
                    leaves: true,
                    field: Some(FieldRequest { n: 36, limb_order: None }),
                    structure: true,
                    ..Request::default()
                };
                let b = build(&f, request).unwrap_or_else(|e| panic!("{id} {age} {seed}: {e}"));
                let o = &b.outputs;
                let leaves = o.leaves.as_ref().map(|l| (&l.instances, l.placed, l.retained));
                println!(
                    "{id}\t{age}\t{seed}\tskeleton {}\telement {}\twood {}\tleaves {}\tstructure {}\tfield {}",
                    h(skeleton(&b.skeleton.tree)),
                    h(&o.element),
                    h(&o.wood),
                    h(leaves),
                    h(&o.structure),
                    h(&o.field)
                );
            }
        }
    }
}
