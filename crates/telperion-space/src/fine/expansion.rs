//! The structure above the cut settled with its fine roots through the
//! four phases of realized mechanics (fn-209 F1 task 6), whichever
//! executor grows the roots: the CPU reference (`realized.rs`) or the
//! GPU's (`telperion-render`). The structure above the cut is grown and
//! settled here exactly as the whole tree settles it; each phase hands
//! the roots one record each down and takes one record each up (Astra
//! change 3: a record per root, never per node).
use super::landing::Landing;
use super::surface::input;
use super::{Clock, Plan};
use crate::bounded::Expected;
use crate::closed_form::boundary::Rows;
use crate::error::{Error, Result};
use crate::fixed::presence::{q, times};
use crate::geometry::{place, scale};
use crate::girth::{thicken_given, Girth};
use crate::grow::alone::Bud;
use crate::grow::realized::{above, Handed, Hands};
use crate::grow::{Request, Rules};
use crate::sag::{any, levers_given, Load};
use crate::shed::{shed_given, Living};
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};

/// A subtree's living as its bearer's fade reads it: the last time
/// anything in it lived (Q32.32 cycles) and its most present living
/// apex (Q32).
pub type Lived = (u64, u64);

/// A root's load at its base: its mass and its first moment, in the
/// tree's frame.
pub type Loaded = (f64, Vec3);

/// A fine root as an executor takes it: its bud, its first axis's
/// vigour as made and its node's buds a whorl, and the presences of the
/// draws that made its first axis (Q32), as its own grower takes them.
pub struct RootSeed {
    pub bud: super::ChainBud,
    pub made: (f64, f64),
    pub birth: [u64; 2],
    pub(crate) handed: Handed,
}

/// What grows and settles the fine roots, phase by phase.
pub trait Roots {
    /// Phase 1: every root grown whole from its seed, marked, assigned
    /// and shed by its own subtree; each one's living, by root, and the
    /// phytomers grown in all.
    fn grow(&mut self, seeds: &[RootSeed]) -> Result<(Vec<Lived>, u64)>;
    /// Phase 2: the live roots (`live`, by root) scaled from their
    /// bearers' base scales and their nodes' drawn scales (`scales`) and
    /// thickened; each one's base radius.
    fn girth(&mut self, live: &[usize], scales: &[(f64, f64)]) -> Vec<f64>;
    /// Phase 3: each live root laid from its bearer (`bearers`, axes of
    /// `tree` as laid before sag); each one's load where any PA sags.
    fn lay(&mut self, tree: &Structure, live: &[usize], bearers: &[usize]) -> Result<Vec<Loaded>>;
    /// Phase 4: each live root laid again from its bearer as laid under
    /// sag, from its own first lay's frames.
    fn bend(&mut self, tree: &Structure, live: &[usize], bearers: &[usize]) -> Result<()>;
}

/// The structure above the cut, settled: the tree (its fine roots' first
/// axes standing where their stand-ins stood, unexpanded), each root's
/// stand-in's index (none: shed), the live roots and their bearers, the
/// seconds each phase took, and the roots laid on the CPU.
pub struct Settled {
    pub structure: Structure,
    pub at: Vec<Option<usize>>,
    pub live: Vec<usize>,
    pub bearers: Vec<usize>,
    pub phases: Vec<f64>,
    /// The roots laid on the CPU because they can reach the ground (host
    /// decision 3), by root; and those roots, by their place among the
    /// live.
    pub grounded: Vec<usize>,
    pub(crate) landed: Vec<(usize, super::realized::Root)>,
}

/// Grows `species` above `plan`'s cut and settles it with its fine
/// roots, grown by `roots`, every root that can reach the ground laid on
/// the CPU reference executor; and the roots' seeds.
pub fn settle(
    species: &Species,
    request: Request,
    expected: &Expected,
    plan: Plan,
    roots: &mut dyn Roots,
) -> Result<(Settled, Vec<RootSeed>)> {
    let rules = crate::grow::alone::rules(species, request.age, request.budget)?;
    settle_with(
        species,
        (request, &expected.rows(), &rules),
        plan,
        roots,
        true,
    )
}

pub(crate) fn settle_with(
    species: &Species,
    (request, rows, rules): (Request, &Rows, &Rules),
    plan: Plan,
    roots: &mut dyn Roots,
    land: bool,
) -> Result<(Settled, Vec<RootSeed>)> {
    request.light.validate()?;
    let threads = match plan.threads {
        0 => crate::cores::available(),
        n => n,
    };
    let pas = species.states.len();
    let hands = Hands::new(pas, request.age, |pa, birth| {
        rows.lateral(pa, birth).count <= plan.cut
    });
    let (grown, handed) = above(rules, hands, request.seed, threads)?;
    let seeds: Vec<RootSeed> = handed
        .into_iter()
        .map(|h| RootSeed {
            bud: input(species, request.seed, Some(&h.bud)),
            made: (h.bud.axis.vigour, whorl(&h.bud)),
            birth: birth(&h.bud),
            handed: h,
        })
        .collect();
    let mut clock = Clock::default();
    let (living, grew) = clock.time(|| roots.grow(&seeds))?;
    if grown.grown + grew > u64::from(request.budget) {
        return Err(Error::Budget {
            limit: request.budget,
        });
    }
    // Shed on what each root realized.
    let mut stood = vec![None; grown.axes.len()];
    for (k, s) in seeds.iter().enumerate() {
        stood[s.handed.at] = Some(k);
    }
    let given = |i: usize| {
        stood[i].map(|k: usize| Living {
            lived: living[k].0,
            alive: living[k].1,
        })
    };
    let (shed, _) = shed_given(grown.axes, species, request.age, &given, None);
    let at: Vec<Option<usize>> = seeds
        .iter()
        .map(|s| Some(shed.index[s.handed.at]).filter(|&i| i != usize::MAX))
        .collect();
    let mut tree = Structure {
        age: request.age,
        pas,
        axes: shed.axes,
    };
    let live: Vec<usize> = (0..seeds.len()).filter(|&k| at[k].is_some()).collect();
    let mut by_axis = vec![None; tree.axes.len()];
    for (j, &k) in live.iter().enumerate() {
        by_axis[at[k].expect("live")] = Some(j);
    }
    let bearers: Vec<usize> = live
        .iter()
        .map(|&k| {
            tree.axes[at[k].expect("live")]
                .origin
                .parent()
                .expect("borne")
        })
        .collect();
    // Scale and girth on each root's realized wood.
    let drawn: Vec<f64> = live
        .iter()
        .zip(&bearers)
        .map(|(&k, &b)| tree.axes[b].phytomers[node(&seeds[k])].scale)
        .collect();
    let base = scale(&mut tree, species, threads);
    let scales: Vec<(f64, f64)> = bearers
        .iter()
        .zip(&drawn)
        .map(|(&b, &d)| (base[b], d))
        .collect();
    let radii = clock.time(|| roots.girth(&live, &scales));
    let radius = |i: usize| by_axis[i].map(|j: usize| radii[j]);
    thicken_given(&mut tree, species, &Girth::default(), &radius, threads);
    place(&mut tree, species, None, threads)?;
    // Lay on each bearer's frame, and load the tree by each root's wood;
    // a root that can reach the ground is laid on the CPU (host decision
    // 3, `landing.rs`).
    let mut loads = clock.time(|| roots.lay(&tree, &live, &bearers))?;
    let mut landing = Landing::new(species, (rules, rows), (plan, threads), land);
    let at_roots = (&seeds[..], &live[..], &bearers[..]);
    landing.lay(&tree, at_roots, (&scales, &radii), &mut loads)?;
    if any(species) {
        let load = |i: usize| {
            by_axis[i].map(|j: usize| Load {
                mass: loads[j].0,
                moment: loads[j].1,
            })
        };
        let (levers, _) = levers_given(&tree, species, &load, threads);
        place(&mut tree, species, Some(&levers), threads)?;
        clock.time(|| roots.bend(&tree, &live, &bearers))?;
        landing.bend(&tree, at_roots, (&scales, &radii))?;
    }
    let grounded = landing.roots.iter().map(|(j, _)| live[*j]).collect();
    let settled = Settled {
        structure: tree,
        at,
        live,
        bearers,
        phases: clock.phases,
        grounded,
        landed: landing.roots,
    };
    Ok((settled, seeds))
}

/// The node of its bearer a root stands on.
pub(crate) fn node(seed: &RootSeed) -> usize {
    match seed.handed.bud.axis.origin {
        Origin::Lateral { node, .. } => node,
        _ => unreachable!("a fine root is a lateral bud"),
    }
}

fn whorl(bud: &Bud) -> f64 {
    match bud.axis.origin {
        Origin::Lateral { whorl, .. } => whorl,
        _ => unreachable!("a fine root is a lateral bud"),
    }
}

/// The presences of the draws that made `bud`'s first axis: a lateral's
/// as its bearer made it; a woken bud's own draw's, and its carrying and
/// reach times what its bearer kept of it.
pub(crate) fn birth(bud: &Bud) -> [u64; 2] {
    match bud.wake {
        None => bud.draws.birth.map(q),
        Some((p, kept)) => {
            let [own, carried] = p.sleeper.made(p.carried, p.aged.2).map(q);
            [own, times(carried, q(kept))]
        }
    }
}
