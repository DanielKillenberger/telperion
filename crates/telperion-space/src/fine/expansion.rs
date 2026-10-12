//! The structure above the cut settled with its fine roots through the
//! four phases of realized mechanics (fn-209 F1 task 6), whichever
//! executor grows the roots: the CPU reference (`realized.rs`) or the
//! GPU's (`telperion-render`). The structure above the cut is grown and
//! settled here exactly as the whole tree settles it; each phase hands
//! the roots one record each down and takes one record each up (Astra
//! change 3: a record per root, never per node).
use super::lay::lay;
use super::step::Step;
use super::{Clock, Plan};
use crate::closed_form::predictor::Predictor;
use crate::error::{refuse, Error, Result};
use crate::contract::presence::{q, times};
use crate::geometry::scale;
use crate::girth::Girth;
use crate::grow::alone::Bud;
use crate::grow::realized::{streamed, Assigned, Handed, Hands};
use crate::grow::{Request, Rules};
use crate::shed::{shed_given, Living};
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};

/// A subtree's living as its bearer's fade reads it: the last time
/// anything in it lived (cycles after its birth) and its most present
/// living apex.
pub type Lived = (f32, f32);

/// A root's load at its base: its mass and its first moment, in the
/// tree's frame.
pub type Loaded = (f64, Vec3);

/// A fine root as an executor takes it: its bud, its first axis's
/// vigour as made and its node's buds a whorl, and the presences of the
/// draws that made its first axis, as its own grower takes them.
pub struct RootSeed {
    pub made: (f64, f64),
    pub birth: [f32; 2],
    pub(crate) handed: Handed,
}

/// What grows and settles the fine roots, phase by phase.
pub trait Roots {
    /// Phase 1 begun early (task 9): the roots made so far and not yet
    /// taken (`seeds`, by their stand-ins' order) offered while the
    /// structure above the cut grows on; how many of the first the
    /// executor took to grow now. Those taken are `grow`'s first seeds, in
    /// the same order. A woken root's seed is offered whole: what its
    /// bearer keeps of it, known once the structure has grown, sizes only
    /// its first axis's vigour (`made.0` and `birth[1]`), which `grow`'s
    /// seed carries. By default none is taken: every root grows in
    /// `grow`.
    fn sow(&mut self, seeds: &[RootSeed]) -> Result<usize> {
        let _ = seeds;
        Ok(0)
    }
    /// Phase 1: every root grown whole from its seed, marked, assigned
    /// and shed by its own subtree; each one's living, by root, and the
    /// phytomers grown in all. `seeds` are every root, by their
    /// stand-ins' order, those `sow` took first.
    fn grow<'a>(&'a mut self, seeds: &'a [RootSeed]) -> Step<'a, Result<(Vec<Lived>, u64)>>;
    /// The exchange where any PA keeps its tree as grown (`retained`,
    /// `leaf_girth`; host decision on fn-211 task 12, PHASES.md): down,
    /// each root's bearer's base and its node's drawn scale as grown
    /// by root; up, each one's leaf partials as grown.
    fn grown_partials(&mut self, _stands: &[(f32, f64)]) -> Result<Vec<[f32; 2]>> {
        refuse(
            "retained",
            "this executor's roots do not carry the as-grown exchange (fn-211 task 12)",
        )
    }
    /// The exchange's second half: down, the tree's leaf mean and each
    /// root's bearers' fades at its base, by root; up, each one's
    /// base radius as grown, its own shed pipes kept for phase 2.
    fn grown_radii(&mut self, _mean: Option<f64>, _above: &[f32]) -> Result<Vec<f64>> {
        refuse(
            "retained",
            "this executor's roots do not carry the as-grown exchange (fn-211 task 12)",
        )
    }
    /// Phase 2: the live roots (`live`, by root) scaled from their
    /// bearers' base scales and their nodes' drawn scales (`scales`) and
    /// thickened; each one's base radius.
    fn girth<'a>(
        &'a mut self,
        live: &'a [usize],
        scales: &'a [(f64, f64)],
    ) -> Step<'a, Result<Vec<f64>>>;
    /// Phase 3: each live root laid from its bearer (`bearers`, axes of
    /// `tree` as laid before sag); each one's load where any PA sags.
    fn lay<'a>(
        &'a mut self,
        tree: &'a Structure,
        live: &'a [usize],
        bearers: &'a [usize],
    ) -> Step<'a, Result<Vec<Loaded>>>;
    /// Phase 4: each live root laid again from its bearer as laid under
    /// sag, from its own first lay's frames.
    fn bend<'a>(
        &'a mut self,
        tree: &'a Structure,
        live: &'a [usize],
        bearers: &'a [usize],
    ) -> Step<'a, Result<()>>;
}

/// The structure above the cut, settled: the tree (its fine roots' first
/// axes standing where their stand-ins stood, unexpanded), each root's
/// stand-in's index (none: shed), and the live roots.
pub struct Settled {
    pub structure: Structure,
    pub at: Vec<Option<usize>>,
    pub live: Vec<usize>,
}

pub(crate) async fn settle_with(
    species: &Species,
    (request, predictor, rules): (Request, &Predictor, &Rules<'_>),
    plan: Plan,
    roots: &mut dyn Roots,
) -> Result<(Settled, Vec<RootSeed>)> {
    request.light.validate()?;
    let threads = match plan.threads {
        0 => crate::cores::available(),
        n => n,
    };
    let mut stages = Stages::default();
    let pas = species.states.len();
    let hands = Hands::new(pas, request.age, |pa, birth| {
        // A PA no zone draws makes no bud to hand on.
        predictor.count(pa, birth).is_some_and(|c| c <= plan.cut)
    });
    let (grown, seeds, sowing) = sow(species, (rules, hands, request.seed, threads), roots)?;
    stages.add(sowing);
    let mut clock = Clock::default();
    let (living, grew) = clock.timed(roots.grow(&seeds)).await?;
    stages.mark("phase 1 (executor)");
    let seats = seeds.iter().map(|s| (s.handed.at, node(s))).collect();
    let settled = settle_grown(
        species,
        (request, rules, plan, threads),
        Above { grown, seats },
        (living, grew),
        roots,
        (clock, stages),
    )
    .await?;
    Ok((settled, seeds))
}

/// The structure above the cut as grown, marked and assigned, and where
/// each root stands on it: its stand-in's index and its bearer's node.
pub(crate) struct Above {
    pub grown: Assigned,
    pub seats: Vec<(usize, usize)>,
}

/// The structure above the cut settled with its roots, grown by `roots`
/// (phase 1's living and phytomers given), through phases 2 to 4 and,
/// where any PA keeps its tree as grown, the exchange before them.
pub(crate) async fn settle_grown(
    species: &Species,
    (request, _rules, _plan, threads): (Request, &Rules<'_>, Plan, usize),
    Above { grown, seats }: Above,
    (living, grew): (Vec<Lived>, u64),
    roots: &mut dyn Roots,
    (mut clock, mut stages): (Clock, Stages),
) -> Result<Settled> {
    let pas = species.states.len();
    if grown.grown + grew > u64::from(request.budget) {
        return Err(Error::Budget {
            limit: request.budget,
        });
    }
    // Shed on what each root realized.
    let mut stood = vec![None; grown.axes.len()];
    for (k, &(at, _)) in seats.iter().enumerate() {
        stood[at] = Some(k);
    }
    let given = |i: usize| {
        stood[i].map(|k: usize| Living {
            lived: living[k].0,
            alive: living[k].1,
        })
    };
    let kept = crate::grow::keeps(species).then(|| grown.axes.clone());
    let at = (request.age, threads);
    let mean = match &kept {
        Some(axes) => Some(super::exchange::partials(species, at, axes, &seats, roots)?),
        None => None,
    };
    let (shed, _) = shed_given(grown.axes, species, request.age, &given, None);
    let girth = match (kept, mean) {
        (Some(axes), Some(mean)) => {
            let seated = (&axes[..], &seats[..], &stood[..]);
            super::exchange::attach(species, at, seated, (&shed, mean), roots)?
        }
        _ => Girth::default(),
    };
    stages.mark("the exchange");
    let at: Vec<Option<usize>> = seats
        .iter()
        .map(|&(i, _)| Some(shed.index[i]).filter(|&i| i != usize::MAX))
        .collect();
    let mut tree = Structure {
        age: request.age,
        pas,
        axes: shed.axes,
    };
    let live: Vec<usize> = (0..seats.len()).filter(|&k| at[k].is_some()).collect();
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
        .map(|(&k, &b)| tree.axes[b].phytomers[seats[k].1].scale)
        .collect();
    let base = scale(&mut tree, species, threads);
    stages.mark("shed and scale");
    let scales: Vec<(f64, f64)> = bearers
        .iter()
        .zip(&drawn)
        .map(|(&b, &d)| (base[b], d))
        .collect();
    lay(
        (species, &mut tree, roots, &girth),
        ((&live[..], &bearers[..]), &scales, &by_axis),
        (&mut clock, &mut stages),
        threads,
    )
    .await?;
    let settled = Settled {
        structure: tree,
        at,
        live,
    };
    Ok(settled)
}

/// Grows the structure above the cut, on a thread of its own where there
/// is more than one, offering `roots` each cycle's buds as they are made
/// (task 9): the structure grown, every root's seed by its stand-in's
/// order, and the stages' milliseconds: the growth above the cut, the
/// executor's sowing past its end, and the woken roots' seeds kept as
/// their bearers kept them.
#[allow(clippy::type_complexity)]
fn sow(
    species: &Species,
    (rules, hands, seed, threads): (&Rules, Hands, u64, usize),
    roots: &mut dyn Roots,
) -> Result<(Assigned, Vec<RootSeed>, Vec<(&'static str, f64)>)> {
    let started = crate::clock::seconds();
    let ms = |t: f64| (t - started) * 1e3;
    let mut sowing = Sowing {
        species,
        seed,
        made: Vec::new(),
        taken: 0,
        failed: None,
    };
    let (grown, ended, sown) = if threads <= 1 {
        // One thread, as on wasm where none can be made (fn-212 decision
        // D6): each cycle's buds are sown as the cycle ends, the same
        // seeds in the same order.
        let grown = streamed(rules, hands, seed, threads, &mut |made| {
            sowing.take(made, roots)
        });
        let ended = crate::clock::seconds();
        (grown, ended, ended)
    } else {
        let (tx, rx) = std::sync::mpsc::channel::<Vec<Handed>>();
        std::thread::scope(|scope| {
            let grower = scope.spawn(move || {
                let grown = streamed(rules, hands, seed, threads, &mut |made| {
                    // The driver's end of the channel lives while this does.
                    tx.send(made).expect("the driver takes the laterals")
                });
                (grown, crate::clock::seconds())
            });
            for buds in &rx {
                let waiting = rx.try_iter().flatten();
                sowing.take(buds.into_iter().chain(waiting).collect(), roots);
            }
            let sown = crate::clock::seconds();
            let (grown, ended) = grower.join().expect("the structure above the cut");
            (grown, ended, sown)
        })
    };
    let (grown, woken) = grown?;
    if let Some(e) = sowing.failed {
        return Err(e);
    }
    // Each woken root as its bearer kept it: its first axis's vigour.
    let mut seeds = sowing.made;
    let mut woken = woken.into_iter();
    for s in seeds.iter_mut().filter(|s| s.handed.bud.wake.is_some()) {
        let kept = woken.next().expect("each woken bud kept");
        assert_eq!(kept.at, s.handed.at, "the woken buds in order");
        (s.made.0, s.birth) = (kept.bud.axis.vigour, birth(&kept.bud));
        s.handed = kept;
    }
    let stages = vec![
        ("above the cut", ms(ended)),
        ("sowing past it", ms(sown.max(ended)) - ms(ended)),
        ("seeds", ms(crate::clock::seconds()) - ms(sown.max(ended))),
    ];
    Ok((grown, seeds, stages))
}

/// The roots made so far, and how many of them the executor took.
struct Sowing<'s> {
    species: &'s Species,
    seed: u64,
    made: Vec<RootSeed>,
    taken: usize,
    failed: Option<Error>,
}

impl Sowing<'_> {
    /// `buds` made into seeds and offered with every one not yet taken.
    fn take(&mut self, buds: Vec<Handed>, roots: &mut dyn Roots) {
        let (_species, _seed) = (self.species, self.seed);
        self.made.extend(buds.into_iter().map(|h| RootSeed {
            made: (h.bud.axis.vigour, whorl(&h.bud)),
            birth: birth(&h.bud),
            handed: h,
        }));
        if self.failed.is_none() && self.taken < self.made.len() {
            match roots.sow(&self.made[self.taken..]) {
                Ok(n) => self.taken += n,
                Err(e) => self.failed = Some(e),
            }
        }
    }
}

/// Each stage's milliseconds, from the last mark.
pub(crate) struct Stages {
    last: f64,
    out: Vec<(&'static str, f64)>,
}

impl Default for Stages {
    fn default() -> Self {
        Self {
            last: crate::clock::seconds(),
            out: Vec::new(),
        }
    }
}

impl Stages {
    /// Stages timed elsewhere, ending now.
    fn add(&mut self, stages: Vec<(&'static str, f64)>) {
        self.out.extend(stages);
        self.last = crate::clock::seconds();
    }

    pub(super) fn mark(&mut self, stage: &'static str) {
        let now = crate::clock::seconds();
        self.out.push((stage, (now - self.last) * 1e3));
        self.last = now;
    }
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
pub(crate) fn birth(bud: &Bud) -> [f32; 2] {
    match bud.wake {
        None => bud.draws.birth.map(q),
        Some((p, kept)) => {
            let [own, carried] = p.sleeper.made(p.carried, p.aged.2).map(q);
            [own, times(carried, q(kept))]
        }
    }
}
