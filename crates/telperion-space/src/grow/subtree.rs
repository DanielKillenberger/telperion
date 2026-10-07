//! A fine subtree grown alone from its root's records (fn-211.4;
//! DESIGN-LIGHT.md, section 5): the engine's own grower, its buds reading
//! their root's recorded light (decision 7: it does not shade itself),
//! its root's recorded vigour split within it by Borchert–Honda on its
//! own light, each unit sized against the pass's recorded per-PA means,
//! its root's shedding and kept share the recorded ones and its inner
//! laterals' balance its own. No layout is grown: its wood is tallied,
//! and the tree it joins is laid once, whole.
use super::couple::{FineRoot, Pass, Probe, Split};
use super::sketch::{Pencil, Sketch};
use super::{bud, Apex, Grower, Rules, Stage};
use crate::couple::Records;
use crate::dormant::Woken;
use crate::error::Result;
use crate::geometry::{dominance, Layer};
use crate::light::Light;
use crate::presence::Draws;
use crate::structure::{Axis, Origin, Vec3};
use std::sync::{Arc, Mutex};

/// What a fine subtree reads of its root's records, and its growth per
/// cycle and PA.
pub(crate) struct Reads {
    pub split: Split,
    pub records: Records,
    pub means: Arc<Vec<Vec<f64>>>,
    pub growth: Vec<Vec<[f64; 2]>>,
}

/// The coupling pass measured (fn-211.4).
#[derive(Debug, Clone, Default)]
pub struct Measured {
    /// Coupling laterals made, and fine roots.
    pub coupled: usize,
    pub fine: usize,
    /// Phytomers grown in the pass and in the fine subtrees, and the
    /// aggregates' expectation of the latter.
    pub pass_grown: u64,
    pub fine_grown: u64,
    pub expected_fine: f64,
    pub record_bytes: usize,
    /// Seconds to the pass's end, and to every fine subtree grown.
    pub pass_s: f64,
    pub fine_s: f64,
    /// By the settled tree's axes, the fine root each stands in
    /// (`usize::MAX`: above the cut).
    pub tag: Vec<usize>,
    /// Per cycle and PA, every growing bud's presence and presence times
    /// size, the pass's and the fine subtrees' together.
    pub growth: Vec<Vec<[f64; 2]>>,
    /// The pass's last field: leaves `(point, area, a stand-in's)` and
    /// reads `(settled axis, point, light, a fine root's)`.
    pub leaves: Vec<(Vec3, f64, bool)>,
    pub reads: Vec<(usize, Vec3, f64, bool)>,
}

/// A fine subtree grown: its axes from index 1 (index 0 its bearer's
/// stub), their draws and lines, its living apexes and woken buds.
pub(super) struct Grown {
    axes: Vec<Axis>,
    draws: Vec<Draws>,
    units: Vec<f64>,
    successor: Vec<Vec<usize>>,
    roots: Vec<u64>,
    horizon: Vec<f64>,
    live: Vec<Apex>,
    woken: Vec<Woken>,
    grown: u64,
    growth: Vec<Vec<[f64; 2]>>,
}

/// A pencil with no frame, at `base_scale`.
pub(super) fn bare(base_scale: f64) -> Pencil {
    let frame = (
        Vec3::default(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    Pencil {
        layer: Layer::bent(frame, Vec3::default()),
        base_scale,
        running: 1.0,
        rank: 0.0,
        units: 0,
        laid: 0,
        trunk: false,
        light: 1.0,
        size: 1.0,
        wood: 0.0,
        memory: 1.0,
        kept: 1.0,
    }
}

/// Adds a growing bud's presence and presence times size to its cycle
/// and PA.
pub(super) fn tally(growth: &mut Vec<Vec<[f64; 2]>>, cycle: u32, pa: usize, p: f64, size: f64) {
    let c = cycle as usize;
    if growth.len() <= c {
        growth.resize(c + 1, Vec::new());
    }
    if growth[c].len() <= pa {
        growth[c].resize(pa + 1, [0.0; 2]);
    }
    growth[c][pa][0] += p;
    growth[c][pa][1] += p * size;
}

/// Grows one fine subtree alone from its records.
fn grow_one(
    rules: &Rules,
    root: &FineRoot,
    (means, split): (&Arc<Vec<Vec<f64>>>, Split),
) -> Result<Grown> {
    let (age, seed) = (rules.age, &root.seed);
    let mut sketch = Sketch::new(Light::NEUTRAL);
    sketch.pencils.push(bare(0.0));
    let mut first = bare(root.scale);
    first.light = root.records.light(seed.from);
    sketch.pencils.push(first);
    let mut g = Grower::bare(rules.clone(), 1);
    g.sketch = Some(sketch);
    g.fine = Some(Box::new(Reads {
        split,
        records: root.records.clone(),
        means: means.clone(),
        growth: Vec::new(),
    }));
    g.rules.grown = 0;
    g.asleep = vec![Vec::new(); age as usize + 2];
    let stub = bud(crate::lineage::Key(0), root.pa, 0, Origin::Seed);
    g.push_axis(stub, Draws::default(), (0.0, 0, f64::NEG_INFINITY));
    let mut axis = seed.axis.clone();
    if let Origin::Lateral { parent, .. } = &mut axis.origin {
        *parent = 0;
    }
    g.push_axis(
        axis,
        seed.draws.clone(),
        (seed.units, seed.root, seed.horizon),
    );
    for &(wakes, mut sleeper) in &seed.asleep {
        sleeper.parent = 1;
        g.asleep[wakes].push(sleeper);
    }
    g.live.push(Apex {
        axis: 1,
        spent: seed.spent,
    });
    // The partial unit a woken root grew in the pass.
    g.sketch(seed.from.saturating_sub(1), &[1])?;
    let mut advanced = Vec::new();
    let quiet = &mut |_: Stage| {};
    for cycle in seed.from..=age {
        if g.live.is_empty() && g.asleep[cycle as usize..].iter().all(Vec::is_empty) {
            break;
        }
        advanced.clear();
        advanced.extend(g.live.iter().map(|a| a.axis));
        g.step(cycle, quiet)?;
        if cycle < age {
            g.sketch(cycle, &advanced)?;
            g.shade(cycle, quiet);
        }
    }
    Ok(Grown {
        growth: g.fine.take().map_or_else(Vec::new, |r| r.growth),
        grown: g.rules.grown,
        axes: g.axes,
        draws: g.draws,
        units: g.units,
        successor: g.successor,
        roots: g.roots,
        horizon: g.horizon,
        live: g.live,
        woken: g.woken,
    })
}

impl<'a> Grower<'a> {
    /// A grower with no tree yet, on `threads` cores.
    pub(super) fn bare(rules: Rules<'a>, threads: usize) -> Self {
        let species = rules.species;
        Self {
            rules,
            species,
            threads,
            share: super::parallel::SHARE,
            horizon: Vec::new(),
            marked: Vec::new(),
            axes: Vec::new(),
            draws: Vec::new(),
            live: Vec::new(),
            units: Vec::new(),
            successor: Vec::new(),
            roots: Vec::new(),
            asleep: Vec::new(),
            woken: Vec::new(),
            sketch: None,
            leaves: Vec::new(),
            wakes: Vec::new(),
            handed: Vec::new(),
            spare: Default::default(),
            oracle: None,
            couple: None,
            fine: None,
        }
    }

    /// Adds an axis with its draws, units, root lineage and horizon.
    pub(super) fn push_axis(
        &mut self,
        axis: Axis,
        draws: Draws,
        (units, root, horizon): (f64, u64, f64),
    ) {
        self.axes.push(axis);
        self.draws.push(draws);
        self.units.push(units);
        self.successor.push(Vec::new());
        self.roots.push(root);
        self.horizon.push(horizon);
    }

    /// A fine subtree's axis `i`'s pencil: no frame, its base scale as
    /// the rough layout gives it, its root's light.
    pub(super) fn fine_pencil(&self, sketch: &Sketch, i: usize) -> Pencil {
        let axis = &self.axes[i];
        let made = self.draws[i].birth[0] * self.draws[i].birth[1];
        let (inherited, vigour) = match axis.origin {
            Origin::Seed => (1.0, made),
            Origin::Lateral { parent, node, .. } => (self.axes[parent].phytomers[node].scale, made),
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                let p = &sketch.pencils[parent];
                (p.base_scale, p.running * made)
            }
        };
        let share = match axis.origin {
            Origin::Lateral { .. } => {
                dominance(self.species.states[axis.pa].form.dominance, axis.lineage)
            }
            _ => 1.0,
        };
        let mut pencil = bare(inherited * vigour * share);
        pencil.light = sketch.pencils[1].light;
        pencil
    }

    /// The rough layout's cycle in a fine subtree: no layout, each unit's
    /// phytomers sized and ranked as `draw` sizes them, and its wood.
    pub(super) fn tally_units(&mut self, sketch: &mut Sketch, advanced: &[usize]) {
        let first_new = sketch.pencils.len();
        for &i in advanced.iter().filter(|&&i| i < first_new) {
            self.tally_axis(sketch, i);
        }
        for i in first_new..self.axes.len() {
            let pencil = self.fine_pencil(sketch, i);
            sketch.pencils.push(pencil);
            self.tally_axis(sketch, i);
        }
    }

    fn tally_axis(&mut self, sketch: &mut Sketch, i: usize) {
        let pencil = &mut sketch.pencils[i];
        let draws = &self.draws[i];
        let axis = &mut self.axes[i];
        let state = &self.species.states[axis.pa];
        while pencil.units < draws.units.len() {
            let k = pencil.units;
            let drawn = draws.units[k];
            let present = pencil.running * drawn.survive;
            let [size, _] = draws.shade(k);
            let birth = axis.birth;
            let end = pencil.laid
                + axis.phytomers[pencil.laid..]
                    .iter()
                    .take_while(|p| (p.cycle - birth - 1) as usize == k)
                    .count();
            for j in pencil.laid..end {
                let p = &mut axis.phytomers[j];
                p.scale = pencil.base_scale * present * draws.nodes[j] * drawn.share;
                p.size = size;
                p.rank = pencil.rank;
                pencil.rank += draws.nodes[j] * drawn.share;
                pencil.wood += state.internode * p.scale * size;
            }
            pencil.laid = end;
            pencil.running = present * drawn.persist;
            pencil.units += 1;
        }
        axis.rank = pencil.rank;
    }

    /// Grows every fine subtree from its records on every core and joins
    /// each to its root's place among the pass's axes.
    pub(super) fn grow_fine_all(&mut self, started: f64) -> Result<Measured> {
        let pass: Box<Pass> = self.couple.take().expect("a coupling pass");
        let pass_s = crate::clock::seconds() - started;
        let pass_grown = self.rules.grown;
        let (means, split) = (Arc::new(pass.means), pass.split);
        let fine = pass.fine;
        let slots: Vec<Mutex<Option<Result<Grown>>>> =
            (0..fine.len()).map(|_| Mutex::new(None)).collect();
        let rules = &self.rules;
        crate::cores::spread(fine.len(), self.threads, &|k| {
            *slots[k].lock().unwrap() = Some(grow_one(rules, &fine[k], (&means, split)));
        });
        let mut tag = vec![usize::MAX; self.axes.len()];
        let mut growth = pass.growth;
        let mut fine_grown = 0;
        for (k, (f, slot)) in fine.iter().zip(slots).enumerate() {
            let mut g = slot.into_inner().unwrap().expect("grown")?;
            fine_grown += g.grown;
            for (c, row) in std::mem::take(&mut g.growth).into_iter().enumerate() {
                for (pa, [p, ps]) in row.into_iter().enumerate().filter(|r| r.1[0] > 0.0) {
                    tally(&mut growth, c as u32, pa, p, ps / p);
                }
            }
            self.join(f.axis, g, k, &mut tag);
        }
        self.rules.grown += fine_grown;
        let Probe { leaves, reads } = pass.probe;
        Ok(Measured {
            coupled: pass.coupled,
            fine: fine.len(),
            pass_grown,
            fine_grown,
            expected_fine: fine.iter().map(|f| f.agg.grown).sum(),
            record_bytes: fine.iter().map(|f| f.records.bytes()).sum(),
            pass_s,
            fine_s: crate::clock::seconds() - started - pass_s,
            tag,
            growth,
            leaves,
            reads,
        })
    }

    /// Puts fine subtree `g` in place of its root `at`, its other axes
    /// after the tree's, each tagged `k`.
    fn join(&mut self, at: usize, g: Grown, k: usize, tag: &mut Vec<usize>) {
        let base = self.axes.len();
        let map = |i: usize| match i {
            0 => usize::MAX,
            1 => at,
            i => base + i - 2,
        };
        let origin = self.axes[at].origin;
        let parts = g
            .axes
            .into_iter()
            .zip(g.draws)
            .zip(g.units)
            .zip(g.successor);
        for (i, (((mut axis, draws), units), successor)) in parts.enumerate().skip(1) {
            match &mut axis.origin {
                Origin::Seed => {}
                Origin::Lateral { parent, .. }
                | Origin::Continuation { parent }
                | Origin::Relay { parent, .. } => *parent = map(*parent),
            }
            let successor: Vec<usize> = successor.iter().map(|&s| map(s)).collect();
            if i == 1 {
                axis.origin = origin;
                self.axes[at] = axis;
                (self.draws[at], self.units[at], self.successor[at]) = (draws, units, successor);
                tag[at] = k;
                continue;
            }
            self.push_axis(axis, draws, (units, g.roots[i], g.horizon[i]));
            self.successor[base + i - 2] = successor;
            tag.push(k);
        }
        let live = g.live.into_iter().map(|a| Apex {
            axis: map(a.axis),
            ..a
        });
        self.live.extend(live);
        self.woken.extend(g.woken.into_iter().map(|mut w| {
            w.axis = map(w.axis);
            w.bearer = map(w.bearer);
            w
        }));
    }
}
