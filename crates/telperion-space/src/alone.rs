//! fn-209 probe P2: a tree whose chains are grown and shed alone, each
//! from its committed boundary, never reading a realized descendant or
//! the realized whole tree; then resolved by option 4's chain geometry
//! (`bounded.rs`).
//!
//! A chain's shedding reads its own axes and, for every bud it bears, that
//! bud's committed extinction curve (the closed form's chance that the
//! bud's subtree has no living apex past a cycle, by its PA and birth). A
//! lateral or relay of the chain is shed when its subtree has held no
//! living apex for more than its delay (`shed.rs`): where its own axes
//! lived past that, it is kept; otherwise it is kept with the chance that
//! some bud it bears still lived, a draw under its own lineage. A bud on
//! a shed axis is never grown. Fades by partial presence are not taken:
//! a kept axis is whole.
use crate::bounded::{Bounded, Expected};
use crate::closed_form::boundary::Rows;
use crate::error::{Error, Result};
use crate::geometry::scale;
use crate::girth::{thicken, Girth};
use crate::grow::alone::{self, Bud, Chain};
use crate::grow::{Record, Request, Rules};
use crate::lineage::{Key, SHED};
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure};
use std::collections::VecDeque;
use std::time::Instant;

/// One chain's growth and shedding: microseconds, phytomers grown, and
/// whether its first axis was kept.
#[derive(Debug, Clone, Copy)]
pub struct ChainCost {
    pub micros: f64,
    pub grown: u64,
    pub kept: bool,
    pub order: usize,
}

/// A tree grown chain by chain alone, every chain resolved.
pub struct Alone<'s> {
    pub bounded: Bounded<'s>,
    /// Every chain grown, in the order grown (breadth first).
    pub costs: Vec<ChainCost>,
}

struct Item {
    bud: Option<Bud>,
    bearer: usize,
    record: Option<usize>,
    order: usize,
    context: Option<f64>,
}

/// Grows `species` chain by chain alone, then resolves every chain's
/// geometry against its committed boundary. Refuses a species any bud of
/// which reads light.
pub fn grow_alone<'s>(
    species: &'s Species,
    request: Request,
    expected: &Expected,
) -> Result<Alone<'s>> {
    grow_alone_with(species, request, expected, true)
}

/// `grow_alone`, its shedding conditioned on its bearer's being kept
/// (`correct`) or not.
pub fn grow_alone_with<'s>(
    species: &'s Species,
    request: Request,
    expected: &Expected,
    correct: bool,
) -> Result<Alone<'s>> {
    let rules = alone::rules(species, request.age, request.budget)?;
    let rows = expected.rows();
    let mut axes: Vec<Axis> = Vec::new();
    let mut records: Vec<Record> = Vec::new();
    let mut costs = Vec::new();
    let mut queue = VecDeque::from([Item {
        bud: None,
        bearer: usize::MAX,
        record: None,
        order: 0,
        context: None,
    }]);
    while let Some(item) = queue.pop_front() {
        let started = Instant::now();
        let chain = alone::grow(&rules, request.seed, item.bud.as_ref())?;
        let (kept, contexts) = shed(&chain, &rows, &rules, item.context.filter(|_| correct));
        costs.push(ChainCost {
            micros: started.elapsed().as_secs_f64() * 1e6,
            grown: chain.grown,
            kept: kept[chain.root],
            order: item.order,
        });
        if !kept[chain.root] {
            continue;
        }
        let mut at = vec![usize::MAX; chain.axes.len()];
        if chain.root > 0 {
            at[0] = item.bearer;
        }
        for i in chain.root..chain.axes.len() {
            if !in_chain(&chain, i) || !kept[i] {
                continue;
            }
            let mut axis = chain.axes[i].clone();
            set_parent(&mut axis.origin, &at);
            at[i] = axes.len();
            axes.push(axis);
        }
        if let Some(r) = item.record {
            records[r].axis = Some(at[chain.root]);
        }
        for (b, context) in chain.buds.into_iter().zip(contexts) {
            let bearer = at[b.bearer()];
            if bearer == usize::MAX {
                continue;
            }
            let mut origin = b.axis.origin;
            set_parent(&mut origin, &at);
            records.push(Record {
                origin,
                pa: b.axis.pa,
                birth: b.axis.birth,
                lineage: b.axis.lineage,
                vigour: b.axis.vigour,
                axis: None,
            });
            queue.push_back(Item {
                bud: Some(b),
                bearer,
                record: Some(records.len() - 1),
                order: item.order + 1,
                context,
            });
        }
    }
    let mut structure = Structure {
        age: request.age,
        pas: species.states.len(),
        axes,
    };
    if structure.phytomer_count() == 0 {
        return Err(Error::Collapsed);
    }
    scale(&mut structure, species, 1);
    thicken(&mut structure, species, &Girth::default(), 1);
    let bounded = Bounded::settled(species, structure, records, (rows, None));
    Ok(Alone { bounded, costs })
}

/// Whether axis `i` of `chain` is one of its own: its first axis, or a
/// continuation or relay of one.
fn in_chain(chain: &Chain, i: usize) -> bool {
    i == chain.root
        || (i > chain.root && !matches!(chain.axes[i].origin, Origin::Lateral { .. }))
}

fn set_parent(origin: &mut Origin, at: &[usize]) {
    match origin {
        Origin::Seed => {}
        Origin::Lateral { parent, .. }
        | Origin::Continuation { parent }
        | Origin::Relay { parent, .. } => *parent = at[*parent],
    }
}

/// Which of the chain's axes are kept at the tree's age, from its own
/// axes and its borne buds' extinction curves alone.
///
/// Where the bud that began the chain was borne on an axis whose own wood
/// had died by its threshold, that axis was kept on its borne buds'
/// chances alone, so this chain's being kept is conditioned on it:
/// `context` is the chance the bearer's other buds had all died, recorded
/// by the bearer, and the chain's own chance π becomes π / (1 - (1 - π)
/// context), the chance it lived given that its bearer was kept. The
/// second value is each borne bud's context, by its place in `chain.buds`.
fn shed(
    chain: &Chain,
    rows: &Rows,
    rules: &Rules,
    context: Option<f64>,
) -> (Vec<bool>, Vec<Option<f64>>) {
    let (species, age) = (rules.species, i64::from(rules.age));
    let n = chain.axes.len();
    let parent = |i: usize| {
        chain.axes[i]
            .origin
            .parent()
            .filter(|&p| p >= chain.root && in_chain(chain, i))
    };
    // Each own axis's delay's last dead cycle, where it is shed at all.
    let threshold: Vec<Option<i64>> = (0..n)
        .map(|i| {
            if !in_chain(chain, i) {
                return None;
            }
            let a = &chain.axes[i];
            let d = species.states[a.pa].shedding;
            let sheds = matches!(a.origin, Origin::Lateral { .. } | Origin::Relay { .. });
            (sheds && d.is_finite()).then(|| (age as f64 - d - 1.0).floor() as i64)
        })
        .collect();
    // The last cycle its own axes lived, through each own axis's subtree.
    let mut until = vec![i64::MIN; n];
    for i in (chain.root..n).rev().filter(|&i| in_chain(chain, i)) {
        let own = chain.axes[i].apex_end.map_or(i64::MAX, i64::from);
        until[i] = until[i].max(own);
        if let Some(p) = parent(i) {
            until[p] = until[p].max(until[i]);
        }
    }
    // The chance every bud borne in each own axis's subtree had died by
    // that axis's threshold.
    let mut dead = vec![1.0f64; n];
    for b in &chain.buds {
        let mut j = Some(b.bearer()).filter(|&j| j >= chain.root);
        while let Some(at) = j {
            if let Some(t) = threshold[at] {
                dead[at] *= rows.dead_by(b.axis.pa, b.axis.birth, t);
            }
            j = parent(at);
        }
    }
    let mut kept = vec![false; n];
    for i in (chain.root..n).filter(|&i| in_chain(chain, i)) {
        let above = parent(i).map_or(true, |p| kept[p]);
        let pass = match threshold[i] {
            None => true,
            Some(t) if until[i] > t => true,
            Some(_) => {
                let u = Key(chain.axes[i].lineage).child(SHED).child(u64::MAX).unit();
                let lives = 1.0 - dead[i];
                let lives = match context.filter(|_| i == chain.root) {
                    Some(r) => (lives / (1.0 - (1.0 - lives) * r)).min(1.0),
                    None => lives,
                };
                u >= 1.0 - lives
            }
        };
        kept[i] = above && pass;
    }
    let contexts = chain
        .buds
        .iter()
        .enumerate()
        .map(|(k, b)| {
            let mut j = Some(b.bearer()).filter(|&j| j >= chain.root);
            while let Some(at) = j {
                if let Some(t) = threshold[at] {
                    if until[at] > t {
                        return None;
                    }
                    let others: f64 = chain
                        .buds
                        .iter()
                        .enumerate()
                        .filter(|&(o, c)| o != k && bears(chain, c.bearer(), at))
                        .map(|(_, c)| rows.dead_by(c.axis.pa, c.axis.birth, t))
                        .product();
                    return Some(others);
                }
                j = parent(at);
            }
            None
        })
        .collect();
    (kept, contexts)
}

/// Whether own axis `j` of `chain` is `at` or stands on it.
fn bears(chain: &Chain, mut j: usize, at: usize) -> bool {
    loop {
        if j == at {
            return true;
        }
        if j < chain.root || !in_chain(chain, j) {
            return false;
        }
        match chain.axes[j].origin.parent() {
            Some(p) if p >= chain.root => j = p,
            _ => return false,
        }
    }
}

/// The committed light field measured (fn-209 probe P2): its resolution,
/// its build's milliseconds, its nodes, the leaves deposited and the buds
/// spread; at the reference's living apexes, its light against the
/// realized field's (p50 and p95 of the absolute difference, and both
/// means); and the buds' expected leaf area against their subtrees'
/// realized last-cycle leaf area.
#[derive(Debug, Clone, Copy)]
pub struct FieldProbe {
    pub cell: f64,
    pub millis: f64,
    pub nodes: usize,
    pub leaves: usize,
    pub buds: usize,
    pub diff: (f64, f64),
    pub means: (f64, f64),
    pub area: (f64, f64),
}

/// Builds the committed light field of the grown `reference`: the own
/// last-cycle leaves of its axes of order up to `levels` (standing for the
/// committed chains of the trunk's first levels), and every lateral bud
/// they bear spread as its closed-form expected leaf area evenly over its
/// extent bound; at each of `cells`.
pub fn field_probe(
    species: &Species,
    reference: &Structure,
    light: crate::light::Light,
    expected: &Expected,
    levels: usize,
    cells: &[f64],
    with_light: bool,
) -> Vec<FieldProbe> {
    use crate::geometry::dominance;
    use crate::light::{bud_light, Field};
    use crate::structure::Vec3;
    let age = reference.age;
    let rows = expected.rows();
    let leaves_of = crate::closed_form::last_leaves(species, species, age);
    let tables: Option<Vec<Vec<Vec<f64>>>> = with_light.then(|| {
        LEVELS
            .iter()
            .map(|&l| crate::closed_form::last_leaves(&at_light(species, l), species, age))
            .collect()
    });
    let levels_tables = tables.as_ref();
    let axes = &reference.axes;
    let mut order = vec![0usize; axes.len()];
    for (i, a) in axes.iter().enumerate() {
        order[i] = match a.origin {
            Origin::Seed => 0,
            Origin::Lateral { parent, .. } => order[parent] + 1,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => order[parent],
        };
    }
    let own: Vec<(Vec3, f64)> = axes
        .iter()
        .enumerate()
        .filter(|(i, _)| order[*i] <= levels)
        .flat_map(|(_, a)| {
            let area = species.states[a.pa].leaf_area;
            a.phytomers
                .iter()
                .filter(|p| p.cycle == age)
                .map(move |p| (p.tip, p.scale * area))
        })
        .collect();
    // Each bud: its base, its expected area and its reach.
    let mut buds: Vec<(Vec3, f64, f64)> = Vec::new();
    let mut lit: Vec<(usize, usize, f64)> = Vec::new();
    let mut root_of = vec![usize::MAX; axes.len()];
    for (i, a) in axes.iter().enumerate() {
        root_of[i] = match a.origin {
            Origin::Lateral { .. } if order[i] == levels + 1 => i,
            Origin::Seed => usize::MAX,
            _ => a.origin.parent().map_or(usize::MAX, |p| root_of[p]),
        };
        let Origin::Lateral { parent, node, .. } = a.origin else {
            continue;
        };
        if order[i] != levels + 1 {
            continue;
        }
        let at = &axes[parent].phytomers[node];
        let inherited = if at.size > 0.0 { at.scale / at.size } else { 0.0 };
        let share = dominance(species.states[a.pa].form.dominance, a.lineage);
        let s = inherited * a.vigour * share;
        let m = (age - a.birth) as usize;
        let reach = rows.lateral(a.pa, a.birth).reach * s;
        buds.push((a.base, leaves_of[a.pa][m] * s, reach));
        lit.push((a.pa, m, s));
    }
    let realized_area: f64 = axes
        .iter()
        .enumerate()
        .filter(|(i, _)| root_of[*i] != usize::MAX)
        .flat_map(|(_, a)| {
            let area = species.states[a.pa].leaf_area;
            a.phytomers
                .iter()
                .filter(|p| p.cycle == age)
                .map(move |p| p.scale * area)
        })
        .sum();
    let expected_area: f64 = buds.iter().map(|b| b.1).sum();
    let realized = bud_light(reference, species, &light);
    let tips: Vec<Vec3> = realized
        .iter()
        .map(|&(i, _)| axes[i].phytomers.last().map_or(axes[i].base, |p| p.tip))
        .collect();
    cells
        .iter()
        .map(|&cell| {
            let started = Instant::now();
            let mut leaves = own.clone();
            let mut expected_lit = expected_area;
            for &(base, area, reach) in &buds {
                spread_ball(&mut leaves, base, area, reach, cell);
            }
            let mut field = Field::sized(&light, &leaves, &[], cell);
            if let Some(levels) = levels_tables {
                // The light axis: each bud's expected area at the light
                // the neutral field gives its base, interpolated in log
                // light between the levels' tables; then the field again.
                let mut spread = own.clone();
                for (b, &(pa, m, s)) in buds.iter().zip(&lit) {
                    let l = field.at(b.0).clamp(LEVELS[LEVELS.len() - 1], 1.0);
                    spread_ball(&mut spread, b.0, interpolate(levels, pa, m, l) * s, b.2, cell);
                }
                field = Field::sized(&light, &spread, &[], cell);
                expected_lit = spread[own.len()..].iter().map(|l| l.1).sum();
            }
            let millis = started.elapsed().as_secs_f64() * 1e3;
            let mut diffs: Vec<f64> = Vec::with_capacity(tips.len());
            let (mut a, mut b) = (0.0, 0.0);
            for (&tip, &(_, l)) in tips.iter().zip(&realized) {
                let c = field.at(tip);
                diffs.push((c - l).abs());
                a += c;
                b += l;
            }
            diffs.sort_by(f64::total_cmp);
            let q = |f: f64| diffs[((diffs.len() - 1) as f64 * f).round() as usize];
            let n = tips.len().max(1) as f64;
            FieldProbe {
                cell,
                millis,
                nodes: field.nodes(),
                leaves: own.len(),
                buds: buds.len(),
                diff: (q(0.5), q(0.95)),
                means: (a / n, b / n),
                area: (expected_lit, realized_area),
            }
        })
        .collect()
}

/// The light levels the light axis holds tables at.
const LEVELS: [f64; 5] = [1.0, 0.5, 0.25, 0.125, 0.0625];

/// `species` grown under a constant light `light` as far as a unit's
/// survival reads it (`grow/advance.rs`): its viability to the power
/// light^-phi. Dormant where no PA's shade hazard is set.
pub(crate) fn at_light(species: &Species, light: f64) -> Species {
    let mut s = species.clone();
    for state in &mut s.states {
        if state.shade_hazard > 0.0 && light < 1.0 {
            state.viability = state.viability.powf(light.powf(-state.shade_hazard));
        }
    }
    s
}

/// A table's value at light `l`, linear in log light between the levels.
fn interpolate(tables: &[Vec<Vec<f64>>], pa: usize, m: usize, l: f64) -> f64 {
    let x = l.ln();
    for k in 0..LEVELS.len() - 1 {
        let (a, b) = (LEVELS[k].ln(), LEVELS[k + 1].ln());
        if x <= a && x >= b {
            let t = (a - x) / (a - b);
            return (1.0 - t) * tables[k][pa][m] + t * tables[k + 1][pa][m];
        }
    }
    tables[LEVELS.len() - 1][pa][m]
}

/// Spreads `area` evenly over the lattice points within `reach` of `base`
/// above the ground.
fn spread_ball(
    leaves: &mut Vec<(crate::structure::Vec3, f64)>,
    base: crate::structure::Vec3,
    area: f64,
    reach: f64,
    cell: f64,
) {
    use crate::structure::Vec3;
    let r = reach.max(cell * 0.5);
    let n = (r / cell).ceil() as i64;
    let first = leaves.len();
    for dx in -n..=n {
        for dy in -n..=n {
            for dz in -n..=n {
                let d = Vec3::new(dx as f64, dy as f64, dz as f64) * cell;
                if d.length() <= r && base.z + d.z >= 0.0 {
                    leaves.push((base + d, 1.0));
                }
            }
        }
    }
    if leaves.len() == first {
        leaves.push((base, area));
        return;
    }
    let k = (leaves.len() - first) as f64;
    for l in &mut leaves[first..] {
        l.1 = area / k;
    }
}
