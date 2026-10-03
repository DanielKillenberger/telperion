//! Shared by the walk tests and the strips example: a three-PA tree that
//! uses every setting, and each continuous setting with its range and the
//! scale it is walked on. The shape measures are in `measure.rs`.
#![allow(dead_code)]
mod measure;
pub use measure::*;
use telperion_space::{grow, Form, NodeLaw, PaState, Request, Species, Structure, Zone};

pub const STEPS: u32 = 200;
pub const AGE: u32 = 12;

fn zone(nodes: NodeLaw, buds: u8, lateral: [f64; 3]) -> Zone {
    Zone {
        nodes,
        buds,
        lateral: lateral.to_vec(),
    }
}

/// A trunk of two zones, limbs that abort, relay, straighten and age into
/// twigs, and twigs that are shed: every setting has structure to act on.
pub fn species() -> Species {
    let trunk = PaState {
        // The trunk's apex never stops within the tree's age.
        lifespan: 2 * AGE,
        next: None,
        viability: 1.0,
        zones: vec![
            zone(NodeLaw::Poisson { mean: 2.0 }, 1, [0.0, 0.15, 0.3]),
            zone(NodeLaw::Uniform { min: 2, max: 3 }, 2, [0.0, 0.6, 0.1]),
        ],
        shedding: None,
        internode: 1.0,
        insertion: 0.0,
        divergence: 2.4,
        abortion: 0.0,
        relay: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        straightening: 0.0,
        form: Form::default(),
    };
    let limb = PaState {
        lifespan: 6,
        next: Some(2),
        viability: 0.92,
        zones: vec![zone(NodeLaw::Poisson { mean: 2.0 }, 1, [0.0, 0.1, 0.5])],
        shedding: None,
        internode: 0.4,
        insertion: 0.7,
        divergence: 2.4,
        abortion: 0.1,
        relay: 0.3,
        readiness: 1.0,
        rhythm: 1.0,
        straightening: 0.3,
        form: Form::default(),
    };
    let twig = PaState {
        lifespan: 3,
        next: None,
        viability: 0.85,
        zones: vec![zone(
            NodeLaw::Uniform { min: 1, max: 2 },
            1,
            [0.0, 0.0, 0.3],
        )],
        shedding: Some(1),
        internode: 0.25,
        insertion: 0.6,
        divergence: 2.4,
        abortion: 0.0,
        relay: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        straightening: 0.0,
        form: Form::default(),
    };
    Species {
        states: vec![trunk, limb, twig],
    }
}

/// One continuous setting by its path, its range, and how to set it. A
/// probability is walked in log-odds, its own scale here: a draw grows in
/// over a share of the room it leaves to certainty, so each step of the
/// probability itself crowds ever more change near 0 and 1.
pub struct Setting {
    pub name: String,
    pub low: f64,
    pub high: f64,
    pub odds: bool,
    /// Walk units per unit of the setting: a rate per metre is walked in
    /// radians over the metres its axis grows.
    pub stretch: f64,
    set: Box<SetValue>,
}

type SetValue = dyn Fn(&mut Species, f64) + Send + Sync;

/// How near a walked probability comes to certainty.
pub const EDGE: f64 = 0.005;

fn logit(p: f64) -> f64 {
    (p / (1.0 - p)).ln()
}

impl Setting {
    pub fn at(&self, value: f64) -> Species {
        let mut s = species();
        (self.set)(&mut s, value);
        s
    }

    /// The walk's coordinate of a value: log-odds for a probability.
    pub fn coordinate(&self, value: f64) -> f64 {
        if self.odds {
            logit(value)
        } else {
            value * self.stretch
        }
    }

    /// The value at walk coordinate `x`.
    pub fn value(&self, x: f64) -> f64 {
        if self.odds {
            1.0 / (1.0 + (-x).exp())
        } else {
            x / self.stretch
        }
    }

    /// The walk coordinate at step `i` of `STEPS`.
    pub fn step(&self, i: u32) -> f64 {
        let (a, b) = (self.coordinate(self.low), self.coordinate(self.high));
        a + (b - a) * f64::from(i) / f64::from(STEPS)
    }

    /// The tree's shape at walk coordinate `x`.
    pub fn shape(&self, x: f64, seed: u64) -> Shape {
        let value = self.value(x);
        let tree = tree(&self.at(value), seed)
            .unwrap_or_else(|e| panic!("{} at {value}, seed {seed}: {e}", self.name));
        Shape::of(&tree)
    }
}

type Set = fn(&mut Species, usize, f64);

pub fn setting(
    name: String,
    low: f64,
    high: f64,
    odds: bool,
    set: impl Fn(&mut Species, f64) + Send + Sync + 'static,
) -> Setting {
    Setting {
        name,
        low,
        high,
        odds,
        stretch: 1.0,
        set: Box::new(set),
    }
}

/// Every continuous setting of `species()`, each over a range that keeps
/// the tree standing, above the ground and within a test's time.
pub fn settings() -> Vec<Setting> {
    let base = species();
    let mut all = Vec::new();
    for (pa, state) in base.states.iter().enumerate() {
        let at = |field: &str| format!("states[{pa}].{field}");
        let low_viability = [0.9, 0.6, 0.6][pa];
        // Long twigs on the lowest limbs reach the ground.
        let (low_internode, high_internode) = [(0.5, 1.2), (0.1, 1.2), (0.1, 0.4)][pa];
        let table: [(&str, f64, f64, bool, Set); 9] = [
            ("viability", low_viability, 1.0 - EDGE, true, |s, pa, v| {
                s.states[pa].viability = v
            }),
            ("abortion", EDGE, 0.5, true, |s, pa, v| {
                s.states[pa].abortion = v
            }),
            ("relay", EDGE, 1.0 - EDGE, true, |s, pa, v| {
                s.states[pa].relay = v
            }),
            ("readiness", EDGE, 1.0 - EDGE, true, |s, pa, v| {
                s.states[pa].readiness = v
            }),
            ("rhythm", 0.0, 1.0, false, |s, pa, v| {
                s.states[pa].rhythm = v
            }),
            ("straightening", 0.0, 1.0, false, |s, pa, v| {
                s.states[pa].straightening = v
            }),
            (
                "internode",
                low_internode,
                high_internode,
                false,
                |s, pa, v| s.states[pa].internode = v,
            ),
            ("insertion", 0.2, 0.9, false, |s, pa, v| {
                s.states[pa].insertion = v
            }),
            (
                "divergence",
                0.3,
                std::f64::consts::PI,
                false,
                |s, pa, v| s.states[pa].divergence = v,
            ),
        ];
        // A trunk bent or wandering far enough leans its crown into the ground.
        let (bend, wander) = [(0.03, 0.1), (1.0, 1.0), (1.0, 1.0)][pa];
        // About the metres an axis of each PA grows in the walk tree.
        let metres = [54.0, 5.0, 1.0][pa];
        let form: [(&str, f64, f64, bool, Set); 4] = [
            ("form.tropism", 0.0, bend, false, |s, pa, v| {
                s.states[pa].form.tropism = v
            }),
            ("form.elevation", 0.0, 1.5, false, |s, pa, v| {
                s.states[pa].form.elevation = v
            }),
            ("form.wander", 0.0, wander, false, |s, pa, v| {
                s.states[pa].form.wander = v
            }),
            (
                "form.plane",
                0.0,
                std::f64::consts::PI,
                false,
                |s, pa, v| s.states[pa].form.plane = v,
            ),
        ];
        for (field, low, high, odds, set) in table.into_iter().chain(form) {
            let mut walked = setting(at(field), low, high, odds, move |s, v| set(s, pa, v));
            if matches!(field, "form.tropism" | "form.wander") {
                walked.stretch = metres;
            }
            all.push(walked);
        }
        for (z, zone) in state.zones.iter().enumerate() {
            let zat = |field: &str| format!("states[{pa}].zones[{z}].{field}");
            if let NodeLaw::Poisson { .. } = zone.nodes {
                all.push(setting(
                    zat("nodes.mean"),
                    0.5,
                    [3.5, 2.5, 3.5][pa],
                    false,
                    move |s, v| s.states[pa].zones[z].nodes = NodeLaw::Poisson { mean: v },
                ));
            }
            for j in pa..zone.lateral.len() {
                let others: f64 = zone
                    .lateral
                    .iter()
                    .enumerate()
                    .filter(|&(k, _)| k != j)
                    .map(|(_, p)| p)
                    .sum();
                let high = (1.0 - others - EDGE).min([0.6, 0.4, 0.45][pa]);
                all.push(setting(
                    zat(&format!("lateral[{j}]")),
                    EDGE,
                    high,
                    true,
                    move |s, v| s.states[pa].zones[z].lateral[j] = v,
                ));
            }
        }
    }
    all
}

pub fn tree(species: &Species, seed: u64) -> telperion_space::Result<Structure> {
    grow(
        species,
        Request {
            age: AGE,
            seed,
            budget: 2_000_000,
        },
    )
}
