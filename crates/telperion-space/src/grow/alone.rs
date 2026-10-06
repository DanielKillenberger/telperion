//! A chain grown alone (fn-209 probe P2). A chain is a seed or lateral
//! axis and every continuation and relay that carries it on. Grown alone,
//! the laterals it makes and the sleeping buds that wake on it are buds it
//! bears: made, keyed and recorded, never grown. Growth reads no other
//! axis where no bud reads light (beech, spruce, palm), so a chain grown
//! alone is the chain the whole-tree grower grows, draw for draw.
use super::shoot::{Rules, Shoot};
use super::{bud, parallel, Apex, Grower};
use crate::dormant::{self, Sleeper, Woken};
use crate::error::{refuse, Error, Result};
use crate::lineage::Key;
use crate::presence::{assign, Draws, Windows};
use crate::shed::lasting;
use crate::species::{PaState, Species};
use crate::structure::{Axis, Origin};

/// A sleeping bud that woke on a chain grown alone, before its chain is
/// grown: what `woke` needs, and the axis that carried it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Pending {
    sleeper: Sleeper,
    aged: (usize, f64, f64),
    carried: f64,
    cycle: u32,
    bearer: usize,
    next: usize,
}

/// A bud a chain bears, as its bearer made it: all its own chain grows
/// from. `axis` is the bud's first axis as made (its lineage, PA, birth,
/// origin in its bearer's chain and its vigour as made); a woken bud
/// carries what waking it reads.
#[derive(Debug, Clone)]
pub(crate) struct Bud {
    pub axis: Axis,
    draws: Draws,
    root: u64,
    horizon: f64,
    wake: Option<(Pending, f64)>,
}

impl Bud {
    /// Whether it slept and woke: its dormant history rides with it.
    pub fn woken(&self) -> bool {
        self.wake.is_some()
    }

    /// The lineage its chain of ages began from.
    pub fn root(&self) -> u64 {
        self.root
    }

    /// What it and the laterals or relays it stands on keep it for past
    /// its last growth.
    pub fn horizon(&self) -> f64 {
        self.horizon
    }

    /// A woken bud's waking: the sleeping bud, the stage it wakes in and
    /// its time there, and the cycle it woke in.
    pub fn woke(&self) -> Option<(Sleeper, (usize, f64), u32)> {
        self.wake
            .map(|(p, _)| (p.sleeper, (p.aged.0, p.aged.1), p.cycle))
    }

    /// The axis of its bearer's chain it stands on.
    pub fn bearer(&self) -> usize {
        self.axis.origin.parent().expect("a bud is borne")
    }
}

/// A chain grown: its axes and the buds it bears, by their place among
/// its axes (index 0 stands for its bearer where it has one), and the
/// phytomers grown.
pub(crate) struct Chain {
    pub axes: Vec<Axis>,
    /// The chain's first axis.
    pub root: usize,
    pub buds: Vec<Bud>,
    pub grown: u64,
    /// The cycles it was grown through.
    pub cycles: u32,
}

/// What every chain of a tree reads: grown once per tree.
pub(crate) fn rules<'a>(species: &'a Species, age: u32, budget: u32) -> Result<Rules<'a>> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    if species.reads_light() {
        return refuse(
            "light",
            "a chain grows alone only where no bud reads light (probe P2)",
        );
    }
    let laterals: Vec<_> = species.states.iter().map(PaState::laterals).collect();
    let lasting = lasting(species, &laterals);
    Ok(Rules {
        species,
        laterals,
        lasting,
        certain: super::Certain::Skip,
        windows: Windows::new(species, age),
        age,
        budget,
        grown: 0,
        alone: true,
    })
}

/// Grows the chain of `bud`, or the seed's (none), alone.
pub(crate) fn grow(rules: &Rules, seed: u64, bud: Option<&Bud>) -> Result<Chain> {
    let species = rules.species;
    let age = rules.age;
    let mut g = Grower {
        rules: rules.clone(),
        species,
        threads: 1,
        share: parallel::SHARE,
        horizon: Vec::new(),
        marked: Vec::new(),
        axes: Vec::new(),
        draws: Vec::new(),
        live: Vec::new(),
        units: Vec::new(),
        successor: Vec::new(),
        roots: Vec::new(),
        asleep: vec![Vec::new(); age as usize + 2],
        woken: Vec::new(),
        sketch: None,
        leaves: Vec::new(),
        wakes: Vec::new(),
    };
    let (from, root) = match bud {
        None => {
            let key = Key::root(seed);
            let (pa, reach) = species.lived(0).ok_or(Error::Collapsed)?;
            let draws = Draws {
                birth: [reach, 1.0],
                ..Draws::default()
            };
            g.adopt(
                bud_axis(key.onto(pa), pa, 0, Origin::Seed),
                draws,
                key.0,
                f64::NEG_INFINITY,
            );
            g.live.push(Apex {
                axis: 0,
                spent: 0.0,
            });
            (1, 0)
        }
        Some(b) => {
            // The bearer, as a stub: the place its bud's origin names.
            let stub = bud_axis(Key(0), b.axis.pa, 0, Origin::Seed);
            let horizon = match b.wake {
                Some(_) => b.horizon,
                None => f64::NEG_INFINITY,
            };
            g.adopt(stub, Draws::default(), 0, horizon);
            match b.wake {
                None => {
                    let mut axis = b.axis.clone();
                    set_parent(&mut axis.origin, 0);
                    g.adopt(axis, b.draws.clone(), b.root, b.horizon);
                    g.live.push(Apex {
                        axis: 1,
                        spent: 0.0,
                    });
                    (b.axis.birth + 1, 1)
                }
                Some((p, kept)) => {
                    let sleeper = Sleeper {
                        parent: 0,
                        ..p.sleeper
                    };
                    let mut shoot = Shoot::new(&g.rules, g.reads(), (&g.roots, &g.horizon));
                    shoot.woke(&sleeper, p.aged, p.carried, (0, 0), p.cycle)?;
                    let share = shoot.finish(Vec::new());
                    g.absorb(share);
                    g.woken.clear();
                    g.draws[1].birth[1] *= kept;
                    (p.cycle + 1, 1)
                }
            }
        }
    };
    let mut cycles = 0;
    for cycle in from..=age {
        if g.live.is_empty() && g.asleep[cycle as usize..].iter().all(Vec::is_empty) {
            break;
        }
        g.step(cycle, &mut |_| {})?;
        cycles += 1;
    }
    for apex in &g.live {
        let living = g.living(*apex);
        let draws = &mut g.draws[apex.axis];
        (draws.alive, draws.living) = (true, living);
    }
    let wakes = std::mem::take(&mut g.wakes);
    let woken: Vec<f64> = wakes
        .iter()
        .map(|p| {
            let w = Woken {
                axis: 0,
                bearer: p.bearer,
                next: p.next,
                cycle: p.cycle,
                share: 1.0 - p.sleeper.sleep,
            };
            w.kept(&g.axes, &g.draws, &g.successor, age)
        })
        .collect();
    assign(&mut g.axes, &g.draws, g.species, 1);
    let mut buds = Vec::new();
    for (i, axis) in g.axes.iter().enumerate() {
        if i > root && matches!(axis.origin, Origin::Lateral { .. }) {
            buds.push(Bud {
                axis: axis.clone(),
                draws: Draws {
                    birth: g.draws[i].birth,
                    ..Draws::default()
                },
                root: g.roots[i],
                horizon: g.horizon[i],
                wake: None,
            });
        }
    }
    for (p, kept) in wakes.into_iter().zip(woken) {
        let s = p.sleeper;
        let (first, _) = species.lived(s.pa).expect("aged");
        let origin = Origin::Lateral {
            parent: s.parent,
            node: s.node,
            slot: s.slot,
            whorl: s.whorl,
            woken: true,
        };
        let mut axis = bud_axis(Key(s.key).onto(first), first, p.cycle - 1, origin);
        let lead = rules.windows.presence(s.lead, s.wood) * s.placed;
        axis.vigour = lead * p.carried * p.aged.2 * kept;
        let horizon = g.horizon[s.parent];
        buds.push(Bud {
            axis,
            draws: Draws::default(),
            root: s.key,
            horizon,
            wake: Some((p, kept)),
        });
    }
    let grown = g.rules.grown;
    Ok(Chain {
        axes: g.axes,
        root,
        buds,
        grown,
        cycles,
    })
}

fn bud_axis(key: Key, pa: usize, birth: u32, origin: Origin) -> Axis {
    bud(key, pa, birth, origin)
}

fn set_parent(origin: &mut Origin, to: usize) {
    match origin {
        Origin::Seed => {}
        Origin::Lateral { parent, .. }
        | Origin::Continuation { parent }
        | Origin::Relay { parent, .. } => *parent = to,
    }
}

impl Grower<'_> {
    fn adopt(&mut self, axis: Axis, draws: Draws, root: u64, horizon: f64) {
        self.axes.push(axis);
        self.draws.push(draws);
        self.units.push(0.0);
        self.successor.push(Vec::new());
        self.roots.push(root);
        self.horizon.push(horizon);
    }

    /// The sleeping buds that wake in `cycle` on a chain grown alone:
    /// buds it bears, one on every axis that carries their bearer on.
    pub(super) fn borne_wakes(&mut self, asleep: Vec<Sleeper>, cycle: u32) -> Result<()> {
        for sleeper in asleep {
            let slept = f64::from(sleeper.slept) + sleeper.sleep;
            let Some(aged) = dormant::aged(self.rules.species, sleeper.pa, slept) else {
                continue;
            };
            for (carried, bearer) in
                dormant::carried(&self.axes, &self.draws, &self.successor, &sleeper, cycle)
            {
                let next = self.draws[bearer].units.len();
                self.wakes.push(Pending {
                    sleeper,
                    aged,
                    carried,
                    cycle,
                    bearer,
                    next,
                });
            }
        }
        Ok(())
    }
}
