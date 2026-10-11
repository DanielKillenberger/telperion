//! The committed light field measured (fn-209 probe P2), split from
//! `alone.rs`.
use crate::bounded::Expected;
use crate::species::Species;
use crate::structure::{Origin, Structure};
use crate::transcendental::Transcendental;

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
/// extent bound; at the lattice's one resolution, 0.5 m (the lattice fixes
/// it, fn-211 task 9).
pub fn field_probe(
    species: &Species,
    reference: &Structure,
    light: crate::light::Light,
    expected: &Expected,
    levels: usize,
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
        let inherited = if at.size > 0.0 {
            at.scale / at.size
        } else {
            0.0
        };
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
    [crate::light::CELL]
        .iter()
        .map(|&cell| {
            let started = crate::clock::seconds();
            let mut leaves = own.clone();
            let mut expected_lit = expected_area;
            for &(base, area, reach) in &buds {
                spread_ball(&mut leaves, base, area, reach, cell);
            }
            let mut field = Field::new(&light, &leaves, &[]);
            if let Some(levels) = levels_tables {
                // The light axis: each bud's expected area at the light
                // the neutral field gives its base, interpolated in log
                // light between the levels' tables; then the field again.
                let mut spread = own.clone();
                for (b, &(pa, m, s)) in buds.iter().zip(&lit) {
                    let l = field.at(b.0).clamp(LEVELS[LEVELS.len() - 1], 1.0);
                    spread_ball(
                        &mut spread,
                        b.0,
                        interpolate(levels, pa, m, l) * s,
                        b.2,
                        cell,
                    );
                }
                field = Field::new(&light, &spread, &[]);
                expected_lit = spread[own.len()..].iter().map(|l| l.1).sum();
            }
            let millis = (crate::clock::seconds() - started) * 1e3;
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
fn at_light(species: &Species, light: f64) -> Species {
    let mut s = species.clone();
    for state in &mut s.states {
        if state.shade_hazard > 0.0 && light < 1.0 {
            state.viability = state
                .viability
                .powf_fixed(light.powf_fixed(-state.shade_hazard));
        }
    }
    s
}

/// A table's value at light `l`, linear in log light between the levels.
fn interpolate(tables: &[Vec<Vec<f64>>], pa: usize, m: usize, l: f64) -> f64 {
    let x = l.ln_fixed();
    for k in 0..LEVELS.len() - 1 {
        let (a, b) = (LEVELS[k].ln_fixed(), LEVELS[k + 1].ln_fixed());
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
