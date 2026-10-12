//! fn-256 task 2: the settled tree for the device's settle (SETTLE-PLAN.md).
//! The grower is set up as `grow_capture_grown` sets it up (every axis
//! open and none retired), grown to its age, then settled as far as
//! `SettleTo` names, and the tree taken in its lineage order: each axis's
//! scalars and its units' and phytomers' values, the leaf term's mean,
//! the disused pipes kept, and each grown axis's place in the order. An
//! experiment entry point, wired into nothing.
use super::super::sketch::Sketch;
use super::super::{start, Options, Request};
use super::capture::Capture;
use super::Working;
use crate::contract::grid;
use crate::error::{Error, Result};
use crate::fine::lineage_permutation;
use crate::species::Species;
use crate::structure::{Axis, Origin, Vec3};
use crate::transcendental::Transcendental;

const NONE: u32 = u32::MAX;

/// How far the captured tree is settled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettleTo {
    /// Shed and its presences assigned (`Grower::settle_shed`).
    Shed,
    /// Scaled and thickened (`Options::settle`).
    Thick,
    /// Laid once, unbent, each phytomer's lever gathered on it
    /// (`s_levers`): what the sag's lay takes.
    Placed,
    /// Laid and sagged: the tree `grow_cycle` returns.
    Laid,
}

/// The tree `grow_cycle` returns from its reference grower, every
/// intermediate the device's settle compares beside it.
pub fn grow_capture_settled(species: &Species, request: Request) -> Result<Capture> {
    grow_capture_settled_at(species, request, SettleTo::Laid)
}

/// The tree settled as far as `to`. A tree that collapses, or cannot be
/// laid, is captured as its error (`s_error`: 1 collapsed, 2 below the
/// ground, 3 beyond the contract's reach; then its grown axis).
pub fn grow_capture_settled_at(species: &Species, request: Request, to: SettleTo) -> Result<Capture> {
    let (mut g, keeps) = start(species, request, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::new(Working::reference()));
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let mut advanced = Vec::new();
    for c in 1..=request.age {
        g.year(c, c < request.age, &mut advanced, &mut |_| {})?;
    }
    let mut c = Capture { arrays: Vec::new() };
    let settling = match g.settle_shed((request, Options::default()), keeps, &mut |_| {}) {
        Err(Error::Collapsed) => return Ok(failed(c, [1, NONE])),
        other => other?,
    };
    let index = settling.index.clone();
    let girth = settling.girth.clone();
    let mut levers = None;
    let structure = match to {
        SettleTo::Shed => settling.structure,
        SettleTo::Placed => {
            let options = Options { settle: true, ..Options::default() };
            let mut s = settling.settle_lay(options, true, &mut |_| {})?.structure;
            if let Err(e) = crate::geometry::place(&mut s, species, None, 1) {
                return Ok(failed(c, error_of(&e, &index)));
            }
            levers = Some(crate::sag::levers(&s, species, 1));
            s
        }
        SettleTo::Thick => {
            let options = Options { settle: true, ..Options::default() };
            settling.settle_lay(options, true, &mut |_| {})?.structure
        }
        SettleTo::Laid => match settling.settle_lay(Options::default(), true, &mut |_| {}) {
            Ok(grown) => grown.structure,
            Err(e) => return Ok(failed(c, error_of(&e, &index))),
        },
    };
    let order = lineage_permutation(&structure.axes);
    let mut at = vec![NONE; order.len()];
    for (k, &i) in order.iter().enumerate() {
        at[i] = k as u32;
    }
    let grown_at: Vec<u32> = index.iter().map(|&i| if i == usize::MAX { NONE } else { at[i] }).collect();
    c.u32s("s_error", &[0, NONE]);
    c.u32s("s_index", &grown_at);
    let mean = girth.leaf_mean.or_else(|| crate::girth::leaf_mean(&structure, species));
    c.u64s("s_leaf_mean", &[u64::from(mean.is_some()), mean.unwrap_or(0.0).to_bits()]);
    let disused: Vec<u64> = (girth.disused.iter())
        .flat_map(|d| {
            let node = d.node.map_or(u64::from(NONE), |n| n as u64);
            [u64::from(at[d.axis]), node, d.radius.to_bits(), d.share.to_bits()]
        })
        .collect();
    c.u64s("s_disused", &disused);
    let ordered: Vec<&Axis> = order.iter().map(|&i| &structure.axes[i]).collect();
    axes(&mut c, &ordered, &at);
    phytomers(&mut c, &ordered);
    if let Some(l) = levers {
        let words: Vec<u32> = (order.iter())
            .flat_map(|&i| l[i].iter().flat_map(|v| v.moment.into_iter().chain(v.chord)).map(f32::to_bits))
            .collect();
        c.u32s("s_levers", &words);
    }
    Ok(c)
}

/// A capture of only the error a settle met.
fn failed(mut c: Capture, error: [u32; 2]) -> Capture {
    c.u32s("s_error", &error);
    c
}

/// The code and grown axis of an error the lay met (`s_error`).
fn error_of(e: &Error, index: &[usize]) -> [u32; 2] {
    let grown = |shed: usize| index.iter().position(|&i| i == shed).map_or(NONE, |g| g as u32);
    match e {
        Error::Collapsed => [1, NONE],
        Error::BelowGround { axis, .. } => [2, grown(*axis)],
        Error::Refused { input, .. } => {
            let axis = input.trim_start_matches("axes[").trim_end_matches(']').parse().ok();
            [3, axis.map_or(NONE, grown)]
        }
        _ => [4, NONE],
    }
}

/// An f64 that holds an f32, as that f32's bits.
fn f32_of(x: f64) -> u32 {
    assert_eq!(f64::from(x as f32), x, "a settled value that is no f32");
    (x as f32).to_bits()
}

fn v(p: Vec3) -> [u32; 3] {
    [f32_of(p.x), f32_of(p.y), f32_of(p.z)]
}

/// Each axis's scalars in the order, its parent renumbered by `at`.
fn axes(c: &mut Capture, ordered: &[&Axis], at: &[u32]) {
    let col = |g: &dyn Fn(&Axis) -> u32| -> Vec<u32> { ordered.iter().map(|a| g(a)).collect() };
    let wide = |g: &dyn Fn(&Axis) -> u64| -> Vec<u64> { ordered.iter().map(|a| g(a)).collect() };
    c.u32s("s_kind", &col(&|a| kind(a.origin)));
    c.u32s("s_parent", &col(&|a| a.origin.parent().map_or(NONE, |p| at[p])));
    c.u32s("s_node", &col(&|a| match a.origin {
        Origin::Lateral { node, .. } | Origin::Relay { node, .. } => node as u32,
        _ => 0,
    }));
    c.u64s("s_whorl", &wide(&|a| match a.origin {
        Origin::Lateral { whorl, .. } => whorl.to_bits(),
        _ => 0,
    }));
    c.u64s("s_lineage", &wide(&|a| a.lineage));
    c.u32s("s_pa", &col(&|a| a.pa as u32));
    c.u32s("s_birth", &col(&|a| a.birth));
    c.u32s("s_apex_end", &col(&|a| a.apex_end.map_or(0, |e| e + 1)));
    for (name, g) in [
        ("s_vigour", &(|a: &Axis| a.vigour) as &dyn Fn(&Axis) -> f64),
        ("s_blend", &|a: &Axis| a.blend),
        ("s_alive", &|a: &Axis| a.alive),
        ("s_rank", &|a: &Axis| a.rank),
    ] {
        c.u64s(name, &wide(&|a| g(a).to_bits()));
    }
    c.u32s("s_sleep", &col(&|a| ticks(a.sleep)));
    c.u32s("s_phase", &col(&|a| a.phase.to_bits()));
    c.u32s("s_units_n", &col(&|a| a.units.len() as u32));
    c.u32s("s_phyt_n", &col(&|a| a.phytomers.len() as u32));
    let frame: Vec<u32> = ordered.iter().flat_map(|a| [v(a.base), v(a.heading), v(a.side)].concat()).collect();
    c.u32s("s_frame", &frame);
    let joined: Vec<u64> = (ordered.iter())
        .flat_map(|a| match a.joined() {
            Some(j) => [1, j.node as u64, u64::from(j.t.to_bits()), j.below.to_bits(), j.above.to_bits()],
            None => [0; 5],
        })
        .collect();
    c.u64s("s_joined", &joined);
    let units: Vec<u64> = ordered.iter().flat_map(|a| a.units.iter().map(|u| u.to_bits())).collect();
    c.u64s("s_unit", &units);
}

/// Each phytomer's values, axis by axis in the order.
fn phytomers(c: &mut Capture, ordered: &[&Axis]) {
    let all = || ordered.iter().flat_map(|a| a.phytomers.iter());
    c.u32s("s_ph_cycle", &all().map(|p| p.cycle).collect::<Vec<_>>());
    c.u64s("s_ph_key", &all().map(|p| p.key).collect::<Vec<_>>());
    let reals = [
        ("s_ph_rank", &(|p: &crate::structure::Phytomer| p.rank) as &dyn Fn(&crate::structure::Phytomer) -> f64),
        ("s_ph_size", &|p| p.size),
        ("s_ph_light", &|p| p.light),
        ("s_ph_grown", &|p| p.grown),
        ("s_ph_scale", &|p| p.scale),
        ("s_ph_radius", &|p| p.radius),
    ];
    for (name, g) in reals {
        c.u64s(name, &all().map(|p| g(p).to_bits()).collect::<Vec<_>>());
    }
    let places: Vec<u32> = all().flat_map(|p| [v(p.tip), v(p.heading), v(p.side)].concat()).collect();
    c.u32s("s_ph_frame", &places);
}

/// An origin's kind, slot and waking (`capture_state`'s `origin`).
fn kind(o: Origin) -> u32 {
    match o {
        Origin::Seed => 0,
        Origin::Lateral { slot, woken, .. } => 1 | u32::from(slot) << 8 | u32::from(woken) << 16,
        Origin::Continuation { .. } => 2,
        Origin::Relay { .. } => 3,
    }
}

fn ticks(t: f64) -> u32 {
    assert!(t >= 0.0 && grid(t) == t, "a time off the tick grid: {t}");
    (t * crate::contract::CYCLE) as u32
}

/// Each PA's settle constants as the device reads them, eight words a
/// PA (`settle_cols.wgsl`'s `K_`): its life in ticks, its successor's go
/// and its relay's on an ended age (presences), its shedding delay plus a
/// cycle (none: an infinite delay), its retained share, its leaf girth's
/// χ, its relay's place among its last unit's nodes, and its flags (sheds
/// 1, retains 2, leaf girth 4, sags 8, dominance 16); then each PA's
/// pipe, ripening, exponent, its reciprocal, secondary share and
/// dominance, its sag's log2, eight words a PA.
pub fn grow_capture_settle_tables(species: &Species) -> Capture {
    let mut c = Capture { arrays: Vec::new() };
    c.u32s("settle_pa", &settle_rows(species));
    c
}

/// `grow_capture_settle_tables`'s rows.
pub(super) fn settle_rows(species: &Species) -> Vec<u32> {
    let mut rows = Vec::new();
    for (pa, s) in species.states.iter().enumerate() {
        let life = (grid(s.life()) * crate::contract::CYCLE).min(f64::from(u32::MAX)) as u32;
        let q = crate::contract::presence::q;
        let go = q(species.successor(pa).map_or(0.0, |(_, go)| go));
        let flags = u32::from(s.shedding.is_finite())
            | u32::from(s.retained > 0.0) << 1
            | u32::from(s.leaf_girth > 0.0) << 2
            | u32::from(s.form.sag > 0.0) << 3
            | u32::from(s.form.dominance > 0.0) << 4;
        let relay = s.relay_at.clamp(0.0, 1.0) as f32;
        let delay = if s.shedding.is_finite() { q(s.shedding + 1.0) } else { 0.0 };
        let words = [life, go.to_bits(), q(s.relay_ended).to_bits(), delay.to_bits()];
        rows.extend(words);
        rows.extend([s.retained as f32, s.leaf_girth as f32, relay].map(f32::to_bits));
        rows.push(flags);
        let f = &s.form;
        let form = [f.pipe as f32, f.ripening as f32, f.exponent as f32, (1.0 / f.exponent) as f32];
        rows.extend(form.map(f32::to_bits));
        let sag = if f.sag > 0.0 { f.sag.log2_fixed() as f32 } else { 0.0 };
        rows.extend([f.secondary as f32, f.dominance as f32, sag, 0.0].map(f32::to_bits));
    }
    rows
}

/// Sag's turn (`sag::turn`) of a phytomer `radius` thick and `length`
/// long under `lever`, its moment capped at `cap`: the device's sag held
/// to the CPU's input by input.
pub fn settle_sag_turn(lever: [f32; 3], cap: f32, sag: f64, (radius, length): (f64, f64)) -> f32 {
    crate::sag::turn(lever, cap, sag, radius, length)
}

/// Whether `grow_cycle`, its working set retiring subtrees as it grows,
/// returns the tree the reference grower (every axis open, none retired)
/// settles: the tree the device's settle is held to (SETTLE-PLAN.md risk
/// 5); and the subtrees retired.
pub fn grow_cycle_is_the_reference(species: &Species, request: Request) -> Result<(bool, usize)> {
    let cycled = super::run(species, request, true, Working::default())?;
    let retired = cycled.census.map_or(0, |c| c.retired);
    let reference = super::run(species, request, false, Working::reference())?;
    Ok((cycled.structure == reference.structure, retired))
}

/// Every retirement verdict `grow_cycle` takes, in its order
/// (`r_log`: the subtree's root, the cycle, waited 0, kept 1 or retired
/// 2, its living's lived and alive bits), and the phytomers each cycle
/// ends holding (`r_held`): what the device's retirement is held to.
pub fn grow_capture_retired(species: &Species, request: Request) -> Result<Capture> {
    let (mut g, _) = start(species, request, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::default());
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let (mut advanced, mut held) = (Vec::new(), Vec::new());
    for c in 1..=request.age {
        g.year(c, c < request.age, &mut advanced, &mut |_| {})?;
        held.push(g.axes.iter().map(|a| a.phytomers.len() as u32).sum::<u32>());
    }
    let mut c = Capture { arrays: Vec::new() };
    let log = &g.working.as_ref().expect("a working set").retirement().log;
    let words: Vec<u32> = (log.iter())
        .flat_map(|&(root, cycle, kind, l)| [root as u32, cycle, u32::from(kind), l.lived.to_bits(), l.alive.to_bits()])
        .collect();
    c.u32s("r_log", &words);
    c.u32s("r_held", &held);
    Ok(c)
}
