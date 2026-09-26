//! A side still of a preset's field, three panels on one grid projected
//! along z: its planned occupancy, the real leaflet shapes (each placed
//! leaf's own oriented box) and the field read from the placed leaves (each
//! leaf's world-aligned box). Foliage is green by depth, wood brown. Writes
//! a binary PPM. The placed and oriented panels are fields no request builds
//! for a planned family, so the still is a test of the crate's own, run by
//! hand: `FIELD_STILL="date-palm 1 0.1 out.ppm" cargo test --profile ci -p
//! telperion-core --lib field_still -- --ignored`.
mod oriented;
use std::io::Write;
use telperion_core::{
    field::Field,
    math::Vec3,
    pipeline::{self, Request},
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
        field: Some(None),
        ..Request::default()
    };
    let built = pipeline::build(&family, request).expect("the preset builds");
    let o = &built.outputs;
    let planned = o.field.as_ref().expect("a field");
    let leaves = &o.leaves.as_ref().expect("leaves").instances;
    let element = o.element.as_ref().expect("an element");
    let placed = Field::new(&built.skeleton.tree, Some((leaves, element))).expect("placed");
    let exact = oriented::leaflets(&built.skeleton.tree, leaves, element);
    let b = planned.bounds().expect("bounds");
    let n = |lo: f64, hi: f64| ((hi - lo) / cell).ceil() as usize + 1;
    let (w, h, d) = (
        n(b.min.x, b.max.x),
        n(b.min.y, b.max.y),
        n(b.min.z, b.max.z),
    );
    let gap = 8;
    let stride = 3 * w + 2 * gap;
    let mut rgb = vec![255u8; stride * h * 3];
    for (panel, field) in [planned, &exact, &placed].into_iter().enumerate() {
        for i in 0..w {
            for j in 0..h {
                let (mut leaf, mut wood) = (0usize, false);
                for k in 0..d {
                    let at = Vec3::new(i as f64 + 0.5, j as f64 + 0.5, k as f64 + 0.5);
                    let hit = field.query(b.min + at * cell, cell / 2.).expect("query");
                    leaf += usize::from(hit.foliage);
                    wood |= hit.wood;
                }
                let px = ((h - 1 - j) * stride + panel * (w + gap) + i) * 3;
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
    write!(f, "P6\n{stride} {h}\n255\n").unwrap();
    f.write_all(&rgb).unwrap();
}
