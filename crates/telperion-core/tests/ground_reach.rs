//! fn-301 R1: what each shipped preset draws below its ground, the plane
//! y = 0 through the root. For every catalogue preset at ages 6, 8, 10, 12,
//! 15, 20 and its own, at seeds 1, 7 and 42, it reads the lowest drawn point
//! of the wood, of the leaves and of the flowers, and how many leaf and
//! flower instances have a vertex below the ground.
//!
//! The wood is read from its drawn vertices, the sweep's own (fn-301 .3):
//! run by run in the curve's order, each ring's vertices and the run's two
//! caps. A trunk's run below the root is its foot, not counted: its buried
//! ring (`flareDepth`, sunk below the root by design) and its rings at the
//! root, which lean with the trunk (the palm's dip to 5 cm, on the base as
//! after). Every other run's vertices are kept off the ground by the
//! surface clearance. Leaves and flowers are read at every element vertex
//! under each stored transform. A leaf's stalk is its seat on the wood,
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

/// The lowest vertex of a foliage, its stalks' largest dip and how many
/// instances reach below what they may: a leaf its stalk's dip,
/// `depth·scale − h`, a flower nothing.
fn foliage_low(f: &Foliage, stalk: bool) -> (f64, f64, usize) {
    let depth = if stalk {
        Wedge::of(&f.element).depth
    } else {
        0.0
    };
    let (mut low, mut dip, mut under) = (f64::INFINITY, 0f64, 0);
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
        under += usize::from(least < BELOW - may);
    }
    (low, dip, under)
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

#[test]
fn nothing_drawn_passes_below_the_ground() {
    println!(
        "preset\tage\tseed\twood_low\tleaf_low\tstalk_dip\tleaves_under\tflower_low\tflowers_under\trachis_low"
    );
    let mut failures = Vec::new();
    for &(_, id, _, _) in CATALOGUE {
        let default = presets::by_identity(id).expect("a shipped preset").age;
        for age in AGES.iter().copied().chain([default]) {
            for seed in SEEDS {
                let mut f = presets::by_identity(id).expect("a shipped preset");
                (f.age, f.skeleton.seed) = (age, seed);
                let m = mesh::build(&f).unwrap_or_else(|e| panic!("{id} {age} {seed}: {e}"));
                let wood = wood_low(&m, &f.surface);
                let (leaf, dip, leaves) = foliage_low(&m.foliage, true);
                let organ = m.organ.as_ref();
                let (flower, _, flowers) =
                    organ.map_or((f64::INFINITY, 0.0, 0), |o| foliage_low(o, false));
                let rachis = organ.map_or(f64::INFINITY, |o| stalk_low(o, &f.bloom));
                let pedicels = organ.map_or(f64::INFINITY, |o| pedicel_low(o, &f.bloom));
                let flower = flower.min(pedicels);
                println!(
                    "{id}\t{age}\t{seed}\t{}\t{}\t{dip:.4}\t{leaves}\t{}\t{flowers}\t{}",
                    show(wood),
                    show(leaf),
                    show(flower),
                    show(rachis)
                );
                if wood < BELOW || leaves > 0 || flowers > 0 || flower < BELOW || rachis < BELOW {
                    failures.push(format!("{id} at {age}, seed {seed}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "below the ground: {failures:?}");
}
