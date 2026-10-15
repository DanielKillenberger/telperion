//! fn-256 task 2 (DESIGN.md sections 2.3 and 7): one cycle's rough
//! layout captured for the device. The tree grows through the one cycle
//! to the captured cycle and grows it (`step`); every input `sketch` reads
//! is then taken, it runs, and every value it writes is taken by the same
//! columns. Each table is one array, column by column, a word a row.
//! Reals are the f32s the layout reads, held exactly; times are ticks. An
//! experiment entry point, wired into nothing.
use super::super::sketch::{Pencil, Sketch};
use super::super::{start, Grower, Options, Request};
use super::capture::Capture;
use crate::contract::{grid, CYCLE};
use crate::error::{refuse, Result};
use crate::geometry::{amount, cone, Layer};
use crate::species::Species;
use crate::structure::{Origin, Vec3};

const NONE: u32 = u32::MAX;
/// An axis's, a phytomer's, a unit's and a pencil's columns, and a PA's
/// form row (the device's `layout.wgsl` holds the same words).
const AXIS: usize = 31;
const PHYT: usize = 19;
const UNIT: usize = 4;
const PENCIL: usize = 63;
const FORM: usize = 20;
/// The pulls a pencil holds inline.
const LIFTS: usize = 8;

/// Grows `species` through the one cycle and captures its busiest laid
/// cycle, the tree's age less one: the last cycle lays nothing.
pub fn grow_capture_layout(species: &Species, request: Request) -> Result<Capture> {
    grow_capture_layout_at(species, request, request.age.saturating_sub(1))
}

/// The rough layout of `cycle` of a tree of `request.age`.
pub fn grow_capture_layout_at(species: &Species, request: Request, cycle: u32) -> Result<Capture> {
    if cycle == 0 || cycle >= request.age {
        return refuse("cycle", "a laid cycle is one before the tree's age");
    }
    let (mut g, _) = start(species, request, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::default());
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    g.rules.lasting = vec![None; species.states.len()].into();
    let mut advanced = Vec::new();
    for c in 1..cycle {
        g.year(c, c < request.age, &mut advanced, &mut |_| {})?;
    }
    assert!(g.couple.is_none() && g.fine.is_none(), "the whole tree, laid");
    let old: Vec<usize> = g.axes.iter().map(|a| a.phytomers.len()).collect();
    advanced.clear();
    advanced.extend(g.live.iter().map(|a| a.axis));
    g.step(cycle, &mut |_| {})?;
    let first_new = g.sketch.as_ref().expect("sketching").pencils.len();
    let mut grew: Vec<usize> = advanced.iter().copied().filter(|&i| i < first_new).collect();
    grew.sort_unstable();
    let rows = Rows::of(&g, &old, &grew, first_new);
    let mut c = Capture { arrays: Vec::new() };
    let n = g.axes.len();
    let counts = [cycle, request.age, first_new as u32, n as u32, rows.phyt.len() as u32];
    c.u32s("lay_counts", &counts);
    c.f32s("lay_form", &form(species));
    c.f32s("lay_globals", &[cone(), ground()]);
    c.u32s("lay_grew", &grew.iter().map(|&i| i as u32).collect::<Vec<_>>());
    c.u32s("lay_generations", &generations(&g, first_new));
    c.u32s("in_axes", &g.axis_columns(&rows));
    c.u32s("in_units", &g.unit_columns(&rows));
    c.u32s("in_phyts", &g.phyt_columns(&rows, &[]));
    c.u32s("in_pencils", &g.pencil_columns(n, &[]));
    let from: Vec<usize> = g.pencils().iter().map(|p| p.laid).collect();
    g.sketch(cycle, &advanced)?;
    let laid: Vec<_> = (g.pencils().iter().enumerate())
        .map(|(i, p)| from.get(i).copied().unwrap_or(0)..p.laid)
        .collect();
    c.u32s("out_axes", &g.axis_columns(&rows));
    c.u32s("out_phyts", &g.phyt_columns(&rows, &laid));
    c.u32s("out_pencils", &g.pencil_columns(n, &[first_new]));
    let leaves: Vec<f32> = (g.leaves.iter())
        .flat_map(|(p, a)| [f(p.x), f(p.y), f(p.z), f(*a)])
        .collect();
    c.f32s("out_leaves", &leaves);
    Ok(c)
}

/// Each PA's form as the layout reads it, every angle an amount.
pub(super) fn form(species: &Species) -> Vec<f32> {
    let mut out = Vec::new();
    for s in &species.states {
        let fm = &s.form;
        out.extend([
            s.internode as f32,
            fm.tropism as f32,
            amount(fm.elevation),
            fm.wander as f32,
            fm.bend_length as f32,
            fm.internode_spread as f32,
            fm.tropism_spread as f32,
            amount(s.insertion),
            amount(fm.insertion_spread),
            amount(fm.roll),
            amount(fm.plane),
            amount(s.divergence),
            s.epitony as f32,
            s.straightening as f32,
            s.erection as f32,
            s.relay_at as f32,
            s.relay_at.clamp(0.0, 1.0) as f32,
            fm.dominance as f32,
        ]);
        out.push(s.leaf_area as f32);
        out.push(s.takeover as f32);
    }
    assert_eq!(out.len(), FORM * species.states.len(), "a form row");
    out
}

/// The largest f32 below the ground's tolerance, `-1e-9` (`Layer::step`).
pub(super) fn ground() -> f32 {
    let t = -1e-9f64;
    let c = t as f32;
    if f64::from(c) < t {
        c
    } else {
        f32::from_bits(c.to_bits() + 1)
    }
}

/// Each generation's size, as `sketch_round` finds them.
fn generations(g: &Grower, first_new: usize) -> Vec<u32> {
    let mut of = vec![0usize; g.axes.len() - first_new];
    let mut sizes: Vec<u32> = Vec::new();
    for i in first_new..g.axes.len() {
        let k = match g.axes[i].origin.parent() {
            Some(p) if p >= first_new => of[p - first_new] + 1,
            _ => 0,
        };
        of[i - first_new] = k;
        if sizes.len() <= k {
            sizes.push(0);
        }
        sizes[k] += 1;
    }
    sizes
}

/// A real the layout holds as an f32.
fn f(x: f64) -> f32 {
    assert_eq!(f64::from(x as f32), x, "a layout value that is no f32");
    x as f32
}

fn bits(x: f64) -> u32 {
    f(x).to_bits()
}

fn tk(t: f64) -> u32 {
    assert!(t >= 0.0 && grid(t) == t, "a time off the tick grid: {t}");
    (t * CYCLE) as u32
}

fn v(p: Vec3) -> [u32; 3] {
    [bits(p.x), bits(p.y), bits(p.z)]
}

/// Where each axis's phytomers and pending units stand in the arenas:
/// the phytomers before the cycle of the axes a frame reads them on, then
/// every axis's grown in it; the units not yet laid.
struct Rows {
    old: Vec<usize>,
    old_first: Vec<u32>,
    new_first: Vec<u32>,
    unit_first: Vec<u32>,
    units_from: Vec<u32>,
    phyt: Vec<(usize, usize)>,
    units: Vec<(usize, usize)>,
}

impl Rows {
    fn of(g: &Grower, old: &[usize], grew: &[usize], first_new: usize) -> Self {
        let n = g.axes.len();
        let pencils = &g.sketch.as_ref().expect("sketching").pencils;
        let read = read_set(g, old, grew, first_new);
        let mut rows = Rows {
            old: (0..n).map(|i| old.get(i).copied().unwrap_or(0)).collect(),
            old_first: vec![NONE; n],
            new_first: vec![NONE; n],
            unit_first: vec![NONE; n],
            units_from: vec![0; n],
            phyt: Vec::new(),
            units: Vec::new(),
        };
        for i in (0..n).filter(|&i| read[i]) {
            rows.old_first[i] = rows.phyt.len() as u32;
            rows.phyt.extend((0..rows.old[i]).map(|j| (i, j)));
        }
        for i in 0..n {
            rows.new_first[i] = rows.phyt.len() as u32;
            let all = g.axes[i].phytomers.len();
            rows.phyt.extend((rows.old[i]..all).map(|j| (i, j)));
            let from = pencils.get(i).map_or(0, |p| p.units);
            rows.unit_first[i] = rows.units.len() as u32;
            rows.units_from[i] = from as u32;
            rows.units.extend((from..g.draws[i].units.len()).map(|k| (i, k)));
        }
        rows
    }
}

/// The axes whose phytomers before the cycle a frame or a walk reads: a
/// lateral's bearer where it stands at or below them, each relay's and
/// continuation's line, and a grown axis laid short of them.
fn read_set(g: &Grower, old: &[usize], grew: &[usize], first_new: usize) -> Vec<bool> {
    let mut read = vec![false; g.axes.len()];
    let pencils = &g.sketch.as_ref().expect("sketching").pencils;
    for &i in grew {
        read[i] |= pencils[i].laid < old[i];
    }
    for a in &g.axes[first_new..] {
        match a.origin {
            Origin::Lateral { parent, node, .. } => {
                read[parent] |= node <= old.get(parent).copied().unwrap_or(0);
            }
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                let mut at = Some(parent);
                while let Some(p) = at {
                    read[p] = true;
                    at = g.axes[p].carried_on().map(|c| c.0);
                }
            }
            Origin::Seed => {}
        }
    }
    read
}

impl Grower<'_> {
    /// Every axis's columns (`AXIS`), column by column.
    fn axis_columns(&self, rows: &Rows) -> Vec<u32> {
        let n = self.axes.len();
        let mut cols = vec![vec![0u32; n]; AXIS];
        for (i, a) in self.axes.iter().enumerate() {
            let d = &self.draws[i];
            let (origin, parent, node, whorl) = match a.origin {
                Origin::Seed => (0, NONE, 0, 0.0),
                Origin::Lateral { parent, node, slot, whorl, woken } => {
                    (1 | u32::from(slot) << 8 | u32::from(woken) << 16, parent as u32, node as u32, whorl as f32)
                }
                Origin::Continuation { parent } => (2, parent as u32, 0, 0.0),
                Origin::Relay { parent, node } => (3, parent as u32, node as u32, 0.0),
            };
            let lineage = [a.lineage as u32, (a.lineage >> 32) as u32];
            let [b, h, s] = [a.base, a.heading, a.side].map(v);
            let row = [
                a.pa as u32,
                a.birth,
                origin,
                parent,
                node,
                whorl.to_bits(),
                lineage[0],
                lineage[1],
                (a.blend as f32).to_bits(),
                a.phase.to_bits(),
                bits(a.rank),
                b[0], b[1], b[2], h[0], h[1], h[2], s[0], s[1], s[2],
                (d.birth[0] as f32).to_bits(),
                (d.birth[1] as f32).to_bits(),
                tk(d.sleep),
                d.units.len() as u32,
                rows.unit_first[i],
                rows.units_from[i],
                rows.old_first[i],
                rows.old[i] as u32,
                rows.new_first[i],
                (a.phytomers.len() - rows.old[i]) as u32,
                0,
            ];
            for (col, w) in cols.iter_mut().zip(row) {
                col[i] = w;
            }
        }
        cols.concat()
    }

    /// The pending units' columns (`UNIT`): survival, persistence, size
    /// and their phytomers' size from the bud's vigour.
    fn unit_columns(&self, rows: &Rows) -> Vec<u32> {
        let mut cols: Vec<Vec<u32>> = (0..UNIT).map(|_| Vec::with_capacity(rows.units.len())).collect();
        for &(i, k) in &rows.units {
            let u = self.draws[i].units[k];
            let row = [
                (u.survive as f32).to_bits(),
                (u.persist as f32).to_bits(),
                (u.size as f32).to_bits(),
                bits(self.draws[i].shade(k)[0]),
            ];
            for (col, w) in cols.iter_mut().zip(row) {
                col.push(w);
            }
        }
        cols.concat()
    }

    /// The arena's phytomers' columns (`PHYT`); a phytomer laid this
    /// cycle carries its leaf's area and a mark.
    fn phyt_columns(&self, rows: &Rows, laid: &[std::ops::Range<usize>]) -> Vec<u32> {
        let mut cols: Vec<Vec<u32>> = (0..PHYT).map(|_| Vec::with_capacity(rows.phyt.len())).collect();
        for &(i, j) in &rows.phyt {
            let p = &self.axes[i].phytomers[j];
            assert_eq!(p.radius, 0.0, "a phytomer laid without girth");
            let [t, h, s] = [p.tip, p.heading, p.side].map(v);
            let area = self.species.states[self.axes[i].pa].leaf_area;
            let leaf = laid.get(i).is_some_and(|r| r.contains(&j));
            let area = crate::light::leaf_area(p.scale, p.size, area);
            let row = [
                p.key as u32,
                (p.key >> 32) as u32,
                p.cycle,
                bits(self.draws[i].nodes[j]),
                t[0], t[1], t[2], h[0], h[1], h[2], s[0], s[1], s[2],
                bits(p.scale),
                bits(p.grown),
                bits(p.size),
                bits(p.rank),
                if leaf { bits(area) } else { 0 },
                u32::from(leaf),
            ];
            for (col, w) in cols.iter_mut().zip(row) {
                col.push(w);
            }
        }
        cols.concat()
    }

    fn pencils(&self) -> &[Pencil] {
        &self.sketch.as_ref().expect("sketching").pencils
    }

    /// Every pencil's columns (`PENCIL`) over `n` rows; the fresh pencils'
    /// constants from `fresh[0]` on, where given.
    fn pencil_columns(&self, n: usize, fresh: &[usize]) -> Vec<u32> {
        let mut cols = vec![vec![0u32; n]; PENCIL];
        for (i, p) in self.pencils().iter().enumerate() {
            let row = pencil_row(p, fresh.first().is_some_and(|&f| i >= f));
            for (col, w) in cols.iter_mut().zip(row) {
                col[i] = w;
            }
        }
        cols.concat()
    }
}

/// A pencil's words: its walk, its pulls inline, then its scalars.
pub(super) fn pencil_row(p: &Pencil, fresh: bool) -> Vec<u32> {
    let mut row: Vec<u32> = walk(&p.layer);
    row.extend([
        bits(p.base_scale),
        bits(p.running),
        bits(p.rank),
        p.units as u32,
        p.laid as u32,
        u32::from(p.trunk),
        bits(p.wood),
    ]);
    let constants = [p.light, p.size, p.memory, p.kept];
    row.extend(constants.map(|x| if fresh { bits(x) } else { 0 }));
    row.extend(p.layer.take_words().map(f32::to_bits));
    row
}

/// A walk's words with its pulls' count and its pulls, `LIFTS` inline.
fn walk(layer: &Layer) -> Vec<u32> {
    let words = layer.words();
    let lifts = (words.len() - 15) / 4;
    assert!(lifts <= LIFTS, "a walk carries {lifts} pulls, past {LIFTS}");
    let mut row: Vec<u32> = words[..15].iter().map(|x| x.to_bits()).collect();
    row.push(lifts as u32);
    row.extend(words[15..].iter().map(|x| x.to_bits()));
    row.resize(16 + 4 * LIFTS, 0);
    row
}
