//! fn-301 R1: what each shipped preset draws below its ground, the plane
//! y = 0 through the root. For every catalogue preset at ages 6, 8, 10, 12,
//! 15, 20 and its own, at seeds 1, 7 and 42, it reads the lowest drawn point
//! of the wood, of the leaves and of the flowers, how many leaf and flower
//! instances have a vertex below the ground, and how many of those reach
//! below what they may.
//!
//! The wood is read from its drawn vertices, the sweep's own (fn-301 .3):
//! run by run in the curve's order, each ring's vertices and the run's two
//! caps. A trunk's run below the root is its foot, not counted: its buried
//! ring (`flareDepth`, sunk below the root by design) and its rings at the
//! root, which lean with the trunk (the palm's dip to 5 cm, on the base as
//! after). Every other run's vertices are kept off the ground by the
//! surface clearance. Beside them each row reports the bark of the wood
//! other than a trunk's, its centreline less its radius, which the lay
//! walk's landing ease reads. Leaves and flowers are read at every
//! element vertex under each stored transform. A leaf's stalk is its seat on the wood,
//! so a leaf may let it down below the ground by its base depth times its
//! scale less its attachment's height (host, 2026-10-21); that dip is
//! reported per row. A flower's rachis is read from the CPU's mirror of the
//! one the renderer rebuilds (`foliage::rachis_low`), kept off the ground
//! as `stalk.wgsl` draws it, and each floret's pedicel as `stalk.wgsl`'s
//! `pedicel_vertex` rebuilds it.
//!
//! It holds ground contact (fn-301 R4): on every row nothing lies below the
//! ground but a leaf stalk's dip within its bound. Red on the base for the
//! date palm, the birch, the spruce, the ordinary tree and the oak (fn-301
//! R1); `--nocapture` prints the table.
//!
//! The gate reads the rows R1 found below the ground on the base and every
//! preset at its own age, seed 1; the ignored `every_row_...` reads all
//! 168 (host, fn-304 ruling 2). Each row's tree is dropped before the next
//! is built.
use telperion_core::{
    foliage::{rachis_low, transform_point, withered, BloomParams, Wedge},
    math::Vec3,
    mesh::{self, Foliage, TreeMesh},
    presets::{self, CATALOGUE},
    surface::SurfaceParams,
};

const AGES: [f64; 6] = [6., 8., 10., 12., 15., 20.];
const SEEDS: [u32; 3] = [1, 7, 42];
/// What "below" admits: a leaf transform is quantised against the crown's
/// box, so a vertex laid on the ground may read a hair under it.
const BELOW: f64 = -1e-4;

/// The lowest drawn vertex of the wood, its trunk's foot left out: the
/// mesh's vertices, `segments` a ring and two caps a run, in the curve's
/// run order, a trunk's below the root not counted.
fn wood_low(m: &TreeMesh, params: &SurfaceParams) -> f64 {
    let segments = params.radial_segments.max(params.lobes * 4) as usize;
    let drawn = m.wood.flattened().positions;
    let (mut low, mut at) = (f64::INFINITY, 0);
    for run in &m.curve.runs {
        let vertices = run.count as usize * segments + 2;
        for v in at..at + vertices {
            let y = f64::from(drawn[3 * v + 1]);
            if !(run.trunk && y < 0.) {
                low = low.min(y);
            }
        }
        at += vertices;
    }
    assert_eq!(3 * at, drawn.len(), "the drawn wood is the curve's runs");
    low
}

/// The lowest bark of wood other than a trunk's, its curve's centreline
/// less its radius: how near the ground the wood the lay walk lands comes
/// (host, 2026-10-21), reported beside the drawn vertices.
fn bark_low(m: &TreeMesh) -> f64 {
    let c = &m.curve;
    let runs = c.runs.iter().filter(|r| !r.trunk);
    runs.flat_map(|r| &c.points[r.first as usize..(r.first + r.count) as usize])
        .map(|p| p.centre.y - p.radius)
        .fold(f64::INFINITY, f64::min)
}

/// The lowest vertex of a foliage, its stalks' largest dip, how many
/// instances have a vertex below the ground and how many reach below what
/// they may: a leaf its stalk's dip, `depth·scale − h`, a flower nothing.
fn foliage_low(f: &Foliage, stalk: bool) -> (f64, f64, usize, usize) {
    let depth = if stalk {
        Wedge::of(&f.element).depth
    } else {
        0.0
    };
    let (mut low, mut dip, mut below, mut under) = (f64::INFINITY, 0f64, 0, 0);
    for m in f.instances.matrices() {
        let scale = (0..3)
            .map(|c| Vec3::new(m[c * 4].into(), m[c * 4 + 1].into(), m[c * 4 + 2].into()).length())
            .sum::<f64>()
            / 3.0;
        let may = (depth * scale - f64::from(m[13])).max(0.0);
        let mut least = f64::INFINITY;
        for &p in &f.element.positions {
            least = least.min(transform_point(&m, p).y);
        }
        low = low.min(least);
        dip = dip.max(-least.min(0.0));
        below += usize::from(least < BELOW);
        under += usize::from(least < BELOW - may);
    }
    (low, dip, below, under)
}

/// The lowest vertex of an organ's pedicels, each rebuilt from its
/// floret's transform as the renderer rebuilds it (`pedicel_vertex`): from
/// its seat a pedicel back along the floret's axis to its base, a tube the
/// stamen's half width round.
fn pedicel_low(f: &Foliage, p: &BloomParams) -> f64 {
    let mut low = f64::INFINITY;
    for (i, m) in f.instances.matrices().enumerate() {
        if withered(f.instances.leaves[i]) {
            continue;
        }
        let column = |c: usize| {
            Vec3::new(m[c * 4].into(), m[c * 4 + 1].into(), m[c * 4 + 2].into()).normalized()
        };
        let base = Vec3::new(m[12].into(), m[13].into(), m[14].into());
        for along in [0.0, 1.0] {
            for side in 0..3 {
                let turn = std::f64::consts::TAU * f64::from(side) / 3.0;
                let round = column(0) * turn.cos() + column(2) * turn.sin();
                let at = base - column(1) * (p.pedicel * (1.0 - along));
                low = low.min((at + round * (0.5 * p.stamen_width)).y);
            }
        }
    }
    low
}

/// The lowest drawn vertex of an organ's rachises: each rebuilt from its
/// instance's seat, heading and radius, kept off the ground as drawn.
fn stalk_low(f: &Foliage, p: &BloomParams) -> f64 {
    let rachises = f
        .instances
        .leaves
        .iter()
        .enumerate()
        .filter(|(_, l)| withered(**l));
    rachises
        .map(|(i, _)| {
            let m = f.instances.matrix(i);
            let heading = Vec3::new(m[4].into(), m[5].into(), m[6].into()).normalized();
            let seat = f.instances.position(i);
            rachis_low(
                p,
                seat,
                heading,
                f.instances.reference.scale(f.instances.leaves[i]),
            )
            .max(0.0)
        })
        .fold(f64::INFINITY, f64::min)
}

fn show(v: f64) -> String {
    if v.is_finite() {
        format!("{v:.3}")
    } else {
        "-".into()
    }
}

/// The rows fn-301 R1 found reaching below the ground on the base
/// (`.flow/evidence/fn-301-a-tree-rests-on-its-ground/r1-base.tsv`).
const TOUCHING: &[(&str, f64, u32)] = &[
    ("ordinary", 40., 42),
    ("oregon-white-oak", 40., 42),
    ("norway-spruce", 40., 1),
    ("norway-spruce", 40., 7),
    ("norway-spruce", 40., 42),
    ("silver-birch", 6., 1),
    ("silver-birch", 6., 7),
    ("silver-birch", 8., 1),
    ("silver-birch", 8., 7),
    ("silver-birch", 8., 42),
    ("silver-birch", 10., 1),
    ("silver-birch", 10., 7),
    ("silver-birch", 10., 42),
    ("silver-birch", 12., 1),
    ("silver-birch", 12., 7),
    ("silver-birch", 12., 42),
    ("silver-birch", 15., 1),
    ("silver-birch", 15., 7),
    ("silver-birch", 15., 42),
    ("silver-birch", 20., 1),
    ("silver-birch", 20., 7),
    ("silver-birch", 20., 42),
    ("silver-birch", 40., 1),
    ("silver-birch", 40., 7),
    ("date-palm", 6., 1),
    ("date-palm", 6., 7),
    ("date-palm", 6., 42),
    ("date-palm", 8., 1),
    ("date-palm", 8., 7),
    ("date-palm", 8., 42),
    ("date-palm", 10., 1),
    ("date-palm", 10., 7),
    ("date-palm", 10., 42),
    ("date-palm", 12., 1),
    ("date-palm", 12., 7),
    ("date-palm", 12., 42),
    ("date-palm", 15., 1),
    ("date-palm", 15., 7),
    ("date-palm", 15., 42),
    ("date-palm", 20., 1),
    ("date-palm", 20., 7),
    ("date-palm", 20., 42),
    ("date-palm", 40., 1),
    ("date-palm", 40., 7),
    ("date-palm", 40., 42),
];

/// A shipped preset's own age.
fn own_age(id: &str) -> f64 {
    presets::by_identity(id).expect("a shipped preset").age
}

/// Every row of R1: each preset at each age and its own, at each seed.
fn every_row() -> Vec<(&'static str, f64, u32)> {
    let mut rows = Vec::new();
    for &(_, id, _, _) in CATALOGUE {
        for age in AGES.iter().copied().chain([own_age(id)]) {
            rows.extend(SEEDS.map(|seed| (id, age, seed)));
        }
    }
    rows
}

/// The gate's rows: the touching ones, then each preset at its own age,
/// seed 1, where not already among them.
fn gate_rows() -> Vec<(&'static str, f64, u32)> {
    let mut rows = TOUCHING.to_vec();
    for &(_, id, _, _) in CATALOGUE {
        let row = (id, own_age(id), 1);
        if !rows.contains(&row) {
            rows.push(row);
        }
    }
    rows
}

/// The row printed, and named if anything lies below the ground. Its tree
/// is dropped on return.
fn below(id: &str, age: f64, seed: u32) -> Option<String> {
    let mut f = presets::by_identity(id).expect("a shipped preset");
    (f.age, f.skeleton.seed) = (age, seed);
    let m = mesh::build(&f).unwrap_or_else(|e| panic!("{id} {age} {seed}: {e}"));
    let wood = wood_low(&m, &f.surface);
    let bark = bark_low(&m);
    let (leaf, dip, leaves_below, leaves) = foliage_low(&m.foliage, true);
    let organ = m.organ.as_ref();
    let (flower, _, flowers_below, flowers) =
        organ.map_or((f64::INFINITY, 0.0, 0, 0), |o| foliage_low(o, false));
    let rachis = organ.map_or(f64::INFINITY, |o| stalk_low(o, &f.bloom));
    let pedicels = organ.map_or(f64::INFINITY, |o| pedicel_low(o, &f.bloom));
    let flower = flower.min(pedicels);
    println!(
        "{id}\t{age}\t{seed}\t{}\t{}\t{}\t{dip:.4}\t{leaves_below}\t{leaves}\t{}\t{flowers_below}\t{flowers}\t{}",
        show(wood),
        show(bark),
        show(leaf),
        show(flower),
        show(rachis)
    );
    let under = wood < BELOW || leaves > 0 || flowers > 0 || flower < BELOW || rachis < BELOW;
    under.then(|| format!("{id} at {age}, seed {seed}"))
}

fn nothing_below(rows: &[(&str, f64, u32)]) {
    println!(
        "preset\tage\tseed\twood_low\tbark_low\tleaf_low\tstalk_dip\tleaves_below\tleaves_under\tflower_low\tflowers_below\tflowers_under\trachis_low"
    );
    let failures: Vec<String> = rows
        .iter()
        .filter_map(|&(id, age, seed)| below(id, age, seed))
        .collect();
    assert!(failures.is_empty(), "below the ground: {failures:?}");
}

#[test]
fn nothing_drawn_passes_below_the_ground() {
    nothing_below(&gate_rows());
}

/// All 168 rows of R1:
/// `cargo test --profile ci -p telperion-core --test ground_reach -- --ignored --nocapture`
#[test]
#[ignore = "fn-304: fn-301 R1's full sweep, outside the gate; about 3 minutes"]
fn nothing_drawn_passes_below_the_ground_on_every_row() {
    nothing_below(&every_row());
}
