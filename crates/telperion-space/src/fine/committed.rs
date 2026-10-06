//! The committed-boundary probe (fn-209 F1 tasks 1 to 5; option 4): a
//! tree whose chains are grown and shed alone, each from its committed
//! boundary, and resolved by option 4 against its bearer's laid frame.
//! Host decision 3 (fn-211's DESIGN-INTERLEAVE.md, "After Astra's
//! review") took it out of production for realized mechanics
//! (`realized.rs`). It stays only as the reference the GPU executor's
//! tests (`telperion-render`'s `generation::fine`) hold their option-4
//! growth and geometry against, until that executor mirrors realized
//! mechanics.
//!
//! The CPU grows the structure above a cut and records each bud below it
//! as a fine root (`record.rs`); the expansion grows every fine root from
//! its record alone (`executor.rs`). A chain is grown from its bud alone
//! and shed by its own axes and its borne buds' committed extinction, on
//! either side of the cut, so a tree is the same, to the bit, at every
//! cut, in every order of its items and in any chunking of them, and it
//! is the tree P2's chain grower grows whole (`tests.rs`).
use super::executor::{expand, Context, Expanded, BEARER};
use super::record::{Attachment, Committed, FineRoot, Frame};
use super::{each, Plan, Work};
use crate::alone::{chains, seed_item, Chains, Item};
use crate::bounded::{Bounded, Expected};
use crate::error::{Error, Result};
use crate::geometry::scale;
use crate::grow::alone::rules;
use crate::grow::Request;
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure};

/// A tree grown above the cut and expanded below it on the committed
/// boundaries: the tree (the structure above the cut first, then each
/// fine root's axes in the order the roots were recorded), its axes above
/// the cut, each root's record, kept phytomers and work, and where each
/// root's axes start.
pub struct CommittedFine {
    pub structure: Structure,
    pub above: usize,
    pub roots: Vec<(FineRoot, usize, Work)>,
    pub starts: Vec<usize>,
    /// The seconds the expansion took, every item on the plan's cores.
    pub expand_seconds: f64,
}

/// Grows `species` above `plan`'s cut on the CPU, then expands every fine
/// root from its record alone on the committed boundaries. Refuses a
/// species any bud of which reads light (F2b).
pub fn grow_committed(
    species: &Species,
    request: Request,
    expected: &Expected,
    plan: Plan,
) -> Result<CommittedFine> {
    let rules = rules(species, request.age, request.budget)?;
    let rows = expected.rows();
    let threads = match plan.threads {
        0 => crate::cores::available(),
        n => n,
    };
    let fine =
        |bud: &crate::grow::alone::Bud| rows.lateral(bud.axis.pa, bud.axis.birth).count <= plan.cut;
    let start = (Vec::new(), Vec::new(), seed_item());
    let Chains {
        axes,
        records,
        handed,
        ..
    } = chains(
        &rules,
        &rows,
        request.seed,
        start,
        (&fine, &mut |_| {}),
        true,
    )?;
    let mut above = Structure {
        age: request.age,
        pas: species.states.len(),
        axes,
    };
    if above.phytomer_count() == 0 && handed.is_empty() {
        return Err(Error::Collapsed);
    }
    scale(&mut above, species, 1);
    let mut bounded = Bounded::settled(species, above, records, (rows.clone(), None));
    bounded.resolve_all(threads)?;
    let roots: Vec<FineRoot> = handed
        .into_iter()
        .map(|item| record(&bounded, species, item))
        .collect();
    let cx = Context {
        species,
        rules: &rules,
        rows: &rows,
        seed: request.seed,
        age: request.age,
    };
    let started = std::time::Instant::now();
    let expanded = execute(&cx, &roots, plan, threads)?;
    let expand_seconds = started.elapsed().as_secs_f64();
    let mut structure = bounded.into_structure();
    let above = structure.axes.len();
    let mut out = Vec::with_capacity(roots.len());
    let mut starts = Vec::with_capacity(roots.len() + 1);
    for (root, done) in roots.into_iter().zip(expanded) {
        let offset = structure.axes.len();
        starts.push(offset);
        let kept = done.axes.iter().map(|a| a.phytomers.len()).sum();
        let bearer = root.attachment.bearer;
        structure
            .axes
            .extend(done.axes.into_iter().map(|a| placed(a, offset, bearer)));
        out.push((root, kept, done.work));
    }
    if structure.phytomer_count() == 0 {
        return Err(Error::Collapsed);
    }
    starts.push(structure.axes.len());
    Ok(CommittedFine {
        structure,
        above,
        roots: out,
        starts,
        expand_seconds,
    })
}

/// The record of the bud `item` hands on, from the laid tree above the cut.
fn record(bounded: &Bounded, species: &Species, item: Item) -> FineRoot {
    let r = item.record.expect("a handed bud is recorded");
    let record = bounded.records()[r];
    let Origin::Lateral { parent, node, .. } = record.origin else {
        unreachable!("a fine root is a lateral bud")
    };
    let (unbent, bent) = (
        &bounded.unbent().axes[parent],
        &bounded.structure().axes[parent],
    );
    let attachment = Attachment {
        bearer: parent,
        pa: bent.pa,
        lineage: bent.lineage,
        birth: bent.birth,
        node,
        unbent: Frame::of(unbent, node),
        bent: Frame::of(bent, node),
    };
    debug_assert_eq!(
        attachment.bent.at.size, 1.0,
        "no unit is sized where no bud reads light"
    );
    let rows = bounded.rows_of();
    let row = rows.lateral(record.pa, record.birth);
    let s = bounded.base_scale(&record);
    let committed = Committed {
        section: bounded.handed(&record, species.states[bent.pa].form.exponent),
        mass: row.mass * s.powi(3),
        moment: row.moment * s.powi(4),
        radius: bounded.base_radius(row, s, record.pa),
    };
    let grounded = grounded(species, &attachment, (row.reach * s, s, committed.radius));
    FineRoot {
        bud: item.bud.expect("a handed bud"),
        context: item.context,
        record,
        attachment,
        committed,
        predicted: row.count,
        grounded,
    }
}

/// Host decision 3: whether a root standing at `attachment` can reach the
/// ground's landing zone. Its wood lies within `reach` (the closed form's
/// extent bound times its base scale `s`) of its node, or of the point
/// below it where a woken bud stands; no phytomer of it is longer than the
/// longest internode at scale `s`, and its radii are taken at twice its
/// committed base radius.
fn grounded(species: &Species, a: &Attachment, (reach, s, radius): (f64, f64, f64)) -> bool {
    let internode = species
        .states
        .iter()
        .map(|p| p.internode)
        .fold(0.0, f64::max);
    let zone = crate::geometry::ground_zone(2.0 * radius, internode * s);
    let low = [a.unbent, a.bent]
        .iter()
        .flat_map(|f| [f.below.z, f.at.tip.z])
        .fold(f64::INFINITY, f64::min);
    low - reach <= zone
}

/// Every root expanded, by its place among `roots`, taken in the plan's
/// order, `chunk` at a time on `threads` cores.
fn execute(cx: &Context, roots: &[FineRoot], plan: Plan, threads: usize) -> Result<Vec<Expanded>> {
    let mut slots: Vec<&FineRoot> = roots.iter().collect();
    each(&mut slots, plan, threads, &|_, root| expand(cx, root))
        .into_iter()
        .collect()
}

/// An expanded axis placed in the whole tree: its root on its bearer, the
/// rest after `offset`.
fn placed(mut axis: Axis, offset: usize, bearer: usize) -> Axis {
    match &mut axis.origin {
        Origin::Seed => unreachable!("a fine root is a lateral bud"),
        Origin::Lateral { parent, .. }
        | Origin::Continuation { parent }
        | Origin::Relay { parent, .. } => {
            *parent = if *parent == BEARER {
                bearer
            } else {
                *parent + offset
            };
        }
    }
    axis
}
