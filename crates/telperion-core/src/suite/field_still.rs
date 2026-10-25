//! A side still of a preset's field, two panels on one grid projected along
//! z: its planned occupancy and the real leaflet shapes (each placed leaf's
//! own oriented box). Foliage is green by depth, wood brown. Writes a binary
//! PPM. The oriented panel is a field no request builds, so the still is a
//! test of the crate's own, run by hand: `FIELD_STILL="date-palm 1 0.1
//! out.ppm" cargo test --profile ci -p telperion-core --lib field_still --
//! --ignored`.
mod oriented;
use std::io::Write;
use telperion_core::{
    field::{self, Frame},
    pipeline::{self, FieldRequest, Request},
    presets::Preset,
};

#[test]
#[ignore = "a still for the eye; FIELD_STILL names the preset, seed, cell and file"]
fn field_still() {
    let args = std::env::var("FIELD_STILL").expect("FIELD_STILL=<preset> <seed> <cell> <out.ppm>");
    let args: Vec<&str> = args.split_whitespace().collect();
    let [id, seed, cell, out] = args[..] else {
        panic!("FIELD_STILL=<preset> <seed> <cell metres> <out.ppm>");
    };
    let mut family = Preset::from_id(id).expect("a shipped preset").parameters();
    family.skeleton.seed = seed.parse().expect("seed");
    let cell: f64 = cell.parse().expect("cell");
    let request = Request {
        leaves: true,
        field: Some(FieldRequest {
            n: 3,
            limb_order: None,
        }),
        ..Request::default()
    };
    let built = pipeline::build(&family, request).expect("the preset builds");
    let (tree, o) = (&built.skeleton.tree, &built.outputs);
    let plan = o.plan.as_ref().expect("a plan");
    let leaves = &o.leaves.as_ref().expect("leaves").instances;
    let exact = oriented::leaflets(leaves, o.element.as_ref().expect("an element"));
    let b = field::bounds(tree, plan).expect("bounds").expect("a box");
    let size = b.max - b.min;
    let n = (size.x.max(size.y).max(size.z) / cell).ceil() as u32 + 2;
    let frame = Frame::over(b, n).expect("a grid");
    let n = n as usize;
    let gap = 8;
    let stride = 2 * n + gap;
    let mut rgb = vec![255u8; stride * n * 3];
    for (panel, plan) in [plan, &exact].into_iter().enumerate() {
        let grid = field::read(tree, plan, frame).expect("read");
        for i in 0..n {
            for j in 0..n {
                let (mut leaf, mut wood) = (0usize, false);
                for k in 0..n {
                    let hit = grid.cells[i + n * (j + n * k)];
                    leaf += usize::from(hit.foliage);
                    wood |= hit.wood;
                }
                let px = ((n - 1 - j) * stride + panel * (n + gap) + i) * 3;
                let shade = |c: u8| (f64::from(c) * (1. - 0.6 * (leaf as f64 / 40.).min(1.))) as u8;
                if leaf > 0 {
                    rgb[px..px + 3].copy_from_slice(&[shade(120), shade(200), shade(110)]);
                } else if wood {
                    rgb[px..px + 3].copy_from_slice(&[110, 80, 55]);
                }
            }
        }
    }
    let mut f = std::fs::File::create(out).expect("output file");
    write!(f, "P6\n{stride} {n}\n255\n").unwrap();
    f.write_all(&rgb).unwrap();
}
