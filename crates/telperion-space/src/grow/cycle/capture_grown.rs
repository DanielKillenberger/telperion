//! fn-256 task 2: the grower as the one cycle leaves it, after its last
//! cycle and before its settle, for the device's build from the seed. The
//! grower is set up as `run` sets it up, with every axis open and none
//! retired (`Working::reference`): retirement leaves the tree as it was
//! (R3B.md) but lets a retired subtree's phytomers go, and the device
//! keeps every one, so this compares the most. An experiment entry point,
//! wired into nothing.
use super::super::sketch::Sketch;
use super::super::{start, Options, Request};
use super::capture::Capture;
use super::capture_layout::pencil_row;
use super::capture_units::Before;
use super::Working;
use crate::error::Result;
use crate::species::Species;

/// The grown state: every axis's scalars and draws, each axis's units
/// and phytomers in order (keys, cycles, node presences, and the sketch's
/// tip, heading, side, scale, grown, size and rank), the pencils, the
/// sleeping buds left, the woken buds, the living apexes and the
/// phytomers grown (`capture_state` under `g_`), and the layout's
/// columns under `sk_`.
pub fn grow_capture_grown(species: &Species, request: Request) -> Result<Capture> {
    let (mut g, _) = start(species, request, true, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::new(Working::reference()));
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let mut advanced = Vec::new();
    for cycle in 1..=request.age {
        g.year(cycle, cycle < request.age, &mut advanced, &mut |_| {})?;
    }
    let mut c = Capture { arrays: Vec::new() };
    c.u32s("g_counts", &[request.age, g.axes.len() as u32]);
    g.capture_state(&mut c, "g", &Before::nothing());
    g.capture_sketch(&mut c);
    Ok(c)
}

impl super::super::Grower<'_> {
    /// The rough layout's values: each axis's rank, base, heading, side
    /// and phase; each phytomer's (in axis order, then node order) tip,
    /// heading, side, scale, grown, size and rank; each pencil's words.
    fn capture_sketch(&self, c: &mut Capture) {
        let w = |x: f64| -> u32 {
            assert_eq!(f64::from(x as f32), x, "a layout value that is no f32");
            (x as f32).to_bits()
        };
        let v = |p: crate::structure::Vec3| [w(p.x), w(p.y), w(p.z)];
        let mut axes = Vec::with_capacity(11 * self.axes.len());
        for a in &self.axes {
            axes.push(w(a.rank));
            axes.extend([a.base, a.heading, a.side].into_iter().flat_map(v));
            axes.push(a.phase.to_bits());
        }
        c.u32s("sk_axes", &axes);
        let mut phyts = Vec::new();
        for a in &self.axes {
            for p in &a.phytomers {
                phyts.extend([p.tip, p.heading, p.side].into_iter().flat_map(v));
                phyts.extend([p.scale, p.grown, p.size, p.rank].map(w));
            }
        }
        c.u32s("sk_phyts", &phyts);
        let pencils = &self.sketch.as_ref().expect("sketching").pencils;
        let rows: Vec<u32> = pencils.iter().flat_map(|p| pencil_row(p, true)).collect();
        c.u32s("sk_pencils", &rows);
    }
}
