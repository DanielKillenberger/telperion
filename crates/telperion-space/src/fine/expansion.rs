//! The structure above the cut settled with its fine roots through the
//! four phases of realized mechanics (fn-209 F1 task 6), whichever
//! executor grows the roots: the CPU reference (`realized.rs`) or the
//! GPU's (`telperion-render`). The structure above the cut is grown and
//! settled here exactly as the whole tree settles it; each phase hands
//! the roots one record each down and takes one record each up (Astra
//! change 3: a record per root, never per node).
use super::landing::Landing;
use super::lay::lay;
use super::spare::Spare;
use super::step::Step;
use super::surface::input;
use super::{Clock, Plan};
use crate::closed_form::boundary::Rows;
use crate::closed_form::predictor::Predictor;
use crate::error::{Error, Result};
use crate::fixed::presence::{q, times};
use crate::geometry::scale;
use crate::grow::alone::Bud;
use crate::grow::realized::{streamed, Assigned, Handed, Hands};
use crate::grow::{Request, Rules};
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
/// stand-in's index (none: shed), the live roots and their bearers, the
/// seconds each phase took, and the roots laid on the CPU.
pub struct Settled {
    pub structure: Structure,
    pub at: Vec<Option<usize>>,
    pub live: Vec<usize>,
    pub bearers: Vec<usize>,
    pub phases: Vec<f64>,
    /// Each stage of the settling, the executor's phases among them, and
    /// its milliseconds, in turn (task 8).
    pub stages: Vec<(&'static str, f64)>,
    /// The roots laid on the CPU because they can reach the ground (host
    /// decision 3), by root; and those roots, by their place among the
    /// live.
    pub grounded: Vec<usize>,
    pub(crate) landed: Vec<(usize, super::realized::Root)>,
    /// Each live root's base radius as phase 2 handed it up, by its place
    /// among the live: what its seam's codominance is read from (fn-212
    /// task 7, host option 3).
    pub radii: Vec<f64>,
    /// The live roots grown ahead on the spare cores for the landing
    /// (task 9, `spare.rs`).
    pub ahead: usize,
}

/// Grows `species` above `plan`'s cut and settles it with its fine
/// roots, grown by `roots`, every root that can reach the ground laid on
/// the CPU reference executor; and the roots' seeds. The cut reads the
/// predictor's counts (host decision 9); the ground bound's reach is read
/// from the closed form's rows, on a core of its own while the executor
/// grows the roots (task 8).
pub async fn settle(
    species: &Species,
    request: Request,
    plan: Plan,
    roots: &mut dyn Roots,
) -> Result<(Settled, Vec<RootSeed>)> {
    let mut stages = Stages::default();
    let rules = crate::grow::alone::rules(species, request.age, request.budget)?;
    stages.mark("rules");
    let predictor = Predictor::new(species, &rules.windows, request.age);
    stages.mark("predictor");
    let (mut settled, seeds) =
        settle_with(species, (request, &predictor, &rules), plan, roots, true).await?;
    stages.out.append(&mut settled.stages);
    settled.stages = stages.out;
    Ok((settled, seeds))
}

pub(crate) async fn settle_with(
    species: &Species,
    (request, predictor, rules): (Request, &Predictor, &Rules<'_>),
    plan: Plan,
    roots: &mut dyn Roots,
    land: bool,
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
    let reach = land.then(|| reach(species, request.age));
    let (grown, seeds, sowing) = sow(species, (rules, hands, request.seed, threads), roots)?;
    stages.add(sowing);
    let mut clock = Clock::default();
    let (living, grew) = clock.timed(roots.grow(&seeds)).await?;
    stages.mark("phase 1 (executor)");
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
    stages.mark("shed and scale");
    let scales: Vec<(f64, f64)> = bearers
        .iter()
        .zip(&drawn)
        .map(|(&b, &d)| (base[b], d))
        .collect();
    // The roots likeliest to reach the ground grown ahead on the spare
    // cores while the rest settles (task 9, `spare.rs`).
    let rows = reach.map(read);
    stages.mark("the reach's rows");
    // No spare cores in a browser: the landing grows its own there.
    let native = cfg!(not(target_family = "wasm"));
    let spare = (rows.as_ref()).and_then(|_| native.then(|| Spare::new(live.len())));
    let at_roots = (&seeds[..], &live[..], &bearers[..]);
    let mut landing = Landing::new(species, (rules, rows.as_ref()), (plan, threads));
    landing.spare = spare.as_ref();
    let laying = lay(
        (species, &mut tree, roots),
        (at_roots, &scales, &by_axis),
        (&mut landing, &mut clock, &mut stages),
        (threads, rows.as_ref()),
    );
    // Natively the driver waits on the executor while the spare cores
    // grow ahead; in a browser it awaits it (host decision 1).
    #[cfg(not(target_family = "wasm"))]
    let live_seeds: Vec<&RootSeed> = live.iter().map(|&k| &seeds[k]).collect();
    #[cfg(not(target_family = "wasm"))]
    let laid = std::thread::scope(|scope| {
        let live_seeds = &live_seeds;
        if let Some(spare) = &spare {
            for _ in 0..helpers(threads) {
                scope.spawn(|| spare.work((species, rules, live_seeds, &scales)));
            }
        }
        let laid = super::step::wait(laying);
        spare.as_ref().inspect(|s| s.stop());
        laid
    });
    #[cfg(target_family = "wasm")]
    let laid = laying.await;
    let laid = laid.map(|radii| (landing.roots, radii, spare.as_ref().map_or(0, Spare::grown)));
    // What was grown ahead and not landed is freed off the build's path.
    if cfg!(target_family = "wasm") {
        drop(spare);
    } else {
        std::thread::spawn(move || drop(spare));
    }
    let (landed, radii, ahead) = laid?;
    let grounded = landed.iter().map(|(j, _)| live[*j]).collect();
    let settled = Settled {
        structure: tree,
        at,
        live,
        bearers,
        phases: clock.phases,
        stages: stages.out,
        grounded,
        landed,
        radii,
        ahead,
    };
    Ok((settled, seeds))
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
        let (species, seed) = (self.species, self.seed);
        self.made.extend(buds.into_iter().map(|h| RootSeed {
            bud: input(species, seed, Some(&h.bud)),
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
pub(super) struct Stages {
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

/// The cores that grow the ground bound's roots ahead (`spare.rs`) while
/// the executor runs: all but the driver's.
#[cfg(not(target_family = "wasm"))]
fn helpers(threads: usize) -> usize {
    threads.saturating_sub(1).max(1)
}

/// The closed form's rows of `species` at `age`, for the ground bound's
/// reach, read on a core of their own: one core reads them faster than
/// many, each with its own tables (REALIZED.md, decision 9's timing). A
/// browser makes no thread, so it reads them in turn.
#[cfg(not(target_family = "wasm"))]
fn reach(species: &Species, age: u32) -> std::thread::JoinHandle<Rows> {
    let species = species.clone();
    std::thread::spawn(move || Rows::new(&species, age, 1))
}
#[cfg(not(target_family = "wasm"))]
fn read(reach: std::thread::JoinHandle<Rows>) -> Rows {
    reach.join().expect("the reach's rows")
}
#[cfg(target_family = "wasm")]
fn reach(species: &Species, age: u32) -> Rows {
    Rows::new(species, age, 1)
}
#[cfg(target_family = "wasm")]
fn read(rows: Rows) -> Rows {
    rows
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
