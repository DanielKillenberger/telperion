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
use super::capture_layout::{form, ground, pencil_row};
use super::capture_units::Before;
use super::Working;
use crate::error::Result;
use crate::geometry::cone;
use crate::species::Species;

/// The grown state: every axis's scalars and draws, each axis's units
/// and phytomers in order (keys, cycles, node presences, and the sketch's
/// tip, heading, side, scale, grown, size and rank), the pencils, the
/// sleeping buds left, the woken buds, the living apexes and the
/// phytomers grown (`capture_state` under `g_`), and the layout's
/// columns under `sk_`.
pub fn grow_capture_grown(species: &Species, request: Request) -> Result<Capture> {
    grow_capture_grown_at(species, request, request.age)
}

/// `grow_capture_grown` after `cycle` of a tree of `request.age`: where a
/// build first parts from the CPU's.
pub fn grow_capture_grown_at(species: &Species, request: Request, cycle: u32) -> Result<Capture> {
    if cycle == 0 || cycle > request.age {
        return crate::error::refuse("cycle", "a captured cycle is one the tree grows");
    }
    let (mut g, _) = start(species, request, true, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::new(Working::reference()));
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let mut advanced = Vec::new();
    for c in 1..=cycle {
        g.year(c, c < request.age, &mut advanced, &mut |_| {})?;
    }
    let mut c = Capture { arrays: Vec::new() };
    c.u32s("g_counts", &[cycle, g.axes.len() as u32]);
    g.capture_state(&mut c, "g", &Before::nothing());
    g.capture_sketch(&mut c);
    Ok(c)
}

/// The species' tables and the request as the device's build reads them,
/// from a grower set up as `run` sets it up, before its first cycle: the
/// units' (`capture_tables`), the layout's form and globals, each PA's
/// balance and allotment constants (upkeep, balance hazard, tolerance,
/// apical control, ψ), and the light's directions and extinction.
pub fn grow_capture_tables(species: &Species, request: Request) -> Result<Capture> {
    let (mut g, _) = start(species, request, true, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    let mut c = Capture { arrays: Vec::new() };
    c.u32s("cycle", &[0, request.age, request.budget]);
    c.u64s("seed", &[request.seed]);
    g.capture_tables(&mut c);
    c.f32s("lay_form", &form(species));
    c.f32s("lay_globals", &[cone(), ground()]);
    let pa: Vec<f32> = (species.states.iter())
        .flat_map(|s| [s.upkeep, s.balance_hazard, s.tolerance, s.apical_control, s.shade_size])
        .map(|x| x as f32)
        .collect();
    c.f32s("hier_pa", &pa);
    g.capture_light_inputs(&mut c);
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
