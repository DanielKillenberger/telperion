//! Shared by the walk tests and the strips example: a three-PA tree that
//! uses every setting, and each continuous setting with its range and the
//! scale it is walked on. The shape measures are in `measure.rs`.
#![allow(dead_code)]
mod bend;
mod measure;
pub use bend::sag_bend;
pub use measure::*;
use telperion_space::{grow, Form, Light, NodeLaw, PaState, Request, Species, Structure, Zone};

pub const STEPS: u32 = 200;
pub const AGE: u32 = 12;

fn zone(nodes: NodeLaw, buds: u8, lateral: [f64; 3]) -> Zone {
    Zone {
        nodes,
        buds,
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
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
        abortion_rise: 0.0,
        relay: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
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
        abortion_rise: 0.0,
        relay: 0.3,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
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
        abortion_rise: 0.0,
        relay: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
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
    /// The site's light at a value (fn-197): none shades but in a light walk.
    light: Box<LightAt>,
}

type LightAt = dyn Fn(f64) -> Light + Send + Sync;

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
        let tree = lit(&self.at(value), (self.light)(value), seed)
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
        light: Box::new(|_| Light::NEUTRAL),
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
        // Sag over a range from none to limbs bowed to the ground; the
        // trunk only leans under its crown.
        let sag = [2e-4, 1e-4, 3e-4][pa];
        let form: [(&str, f64, f64, bool, Set); 14] = [
            ("abortion_rise", 0.0, 3.0, false, |s, pa, v| {
                s.states[pa].abortion_rise = v
            }),
            ("erection", 0.0, 0.5, false, |s, pa, v| {
                s.states[pa].erection = v
            }),
            ("relay_at", 0.0, 1.0, false, |s, pa, v| {
                s.states[pa].relay_at = v
            }),
            ("epitony", 0.0, 1.0, false, |s, pa, v| {
                s.states[pa].epitony = v
            }),
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
            ("form.dominance", 0.0, 1.0, false, |s, pa, v| {
                s.states[pa].form.dominance = v
            }),
            ("form.ripening", 0.0, 20.0, false, |s, pa, v| {
                s.states[pa].form.ripening = v
            }),
            ("form.exponent", 2.0, 3.0, false, |s, pa, v| {
                s.states[pa].form.exponent = v
            }),
            ("form.roll", 0.0, std::f64::consts::PI, false, |s, pa, v| {
                s.states[pa].form.roll = v
            }),
            ("form.sag", 0.0, sag, false, |s, pa, v| {
                s.states[pa].form.sag = v
            }),
            ("form.secondary", 0.0, 1.0, false, |s, pa, v| {
                s.states[pa].form.secondary = v
            }),
        ];
        for (field, low, high, odds, set) in table.into_iter().chain(form) {
            let mut walked = setting(at(field), low, high, odds, move |s, v| set(s, pa, v));
            if matches!(field, "form.tropism" | "form.wander") {
                walked.stretch = metres;
            }
            // Sag is walked in the radians it bends its most loaded axis.
            if field == "form.sag" {
                walked.stretch = sag_bend(&tree(&base, 1).unwrap(), pa);
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
    // A stop that is always relayed, as Troll's modules are: the
    // module-ending chance crossing a draw moves the crown by degree from
    // the continuation to a relay in the module's curvature zone.
    all.push(setting(
        "states[0].abortion, relay 1".into(),
        EDGE,
        0.5,
        true,
        |s, v| {
            let trunk = &mut s.states[0];
            trunk.relay = 1.0;
            trunk.relay_at = 0.5;
            trunk.epitony = 0.6;
            trunk.insertion = 0.3;
            trunk.straightening = 1.0;
            trunk.abortion = v;
        },
    ));
    // Girth moves the shape only where wood bends under its load: secondary
    // growth walked on sagging limbs, from wood that keeps its established
    // width to the pipe model's (fn-205 R3).
    all.push(setting(
        "states[1].form.secondary, sag 1e-4".into(),
        0.0,
        1.0,
        false,
        |s, v| {
            s.states[1].form.sag = 1e-4;
            s.states[1].form.secondary = v;
        },
    ));
    all.extend(sleeping_settings());
    all
}

/// Sleeping buds on the walk tree (fn-202): twigs along the limbs, as a
/// spruce bough's draperies, and limbs along the trunk, as an oak's
/// epicormic shoots, waking after a delay at a yearly rate.
pub fn sleeping(s: &mut Species) {
    let limb = &mut s.states[1].zones[0];
    limb.dormant[2] = 0.3;
    (limb.delay, limb.rate) = (1.5, 0.4);
    let trunk = &mut s.states[0].zones[1];
    trunk.dormant[1] = 0.15;
    (trunk.delay, trunk.rate) = (3.0, 0.3);
}

fn asleep(name: &str, low: f64, high: f64, odds: bool, edit: fn(&mut Species, f64)) -> Setting {
    setting(format!("sleeping: {name}"), low, high, odds, move |s, v| {
        sleeping(s);
        edit(s, v)
    })
}

/// The sleeping probability walked, and the limbs' survival and abortion
/// walked with buds asleep on them, which decide whether their bearer
/// still lives when the buds wake.
fn sleeping_settings() -> Vec<Setting> {
    vec![
        asleep("states[1].zones[0].dormant[2]", EDGE, 0.6, true, |s, v| {
            s.states[1].zones[0].dormant[2] = v
        }),
        asleep("states[0].zones[1].dormant[1]", EDGE, 0.4, true, |s, v| {
            s.states[0].zones[1].dormant[1] = v
        }),
        asleep("states[1].viability", 0.6, 1.0 - EDGE, true, |s, v| {
            s.states[1].viability = v
        }),
        asleep("states[1].abortion", EDGE, 0.5, true, |s, v| {
            s.states[1].abortion = v
        }),
    ]
}

/// The release law walked: each zone's rate and delay.
pub fn release_settings() -> Vec<Setting> {
    vec![
        asleep("states[1].zones[0].rate", 0.05, 2.0, false, |s, v| {
            s.states[1].zones[0].rate = v
        }),
        asleep("states[1].zones[0].delay", 0.0, 8.0, false, |s, v| {
            s.states[1].zones[0].delay = v
        }),
        asleep("states[0].zones[1].rate", 0.05, 2.0, false, |s, v| {
            s.states[0].zones[1].rate = v
        }),
        asleep("states[0].zones[1].delay", 0.0, 8.0, false, |s, v| {
            s.states[0].zones[1].delay = v
        }),
        // The woken axes' rising abortion hazard and secondary erection
        // count the time they slept.
        asleep(
            "states[0].zones[1].delay, limbs rise and erect",
            0.0,
            8.0,
            false,
            |s, v| {
                s.states[0].zones[1].delay = v;
                s.states[1].abortion_rise = 1.0;
                s.states[1].erection = 0.5;
            },
        ),
        asleep(
            "states[1].zones[0].rate, twigs rise and erect",
            0.05,
            2.0,
            false,
            |s, v| {
                s.states[1].zones[0].rate = v;
                s.states[2].abortion = 0.2;
                s.states[2].abortion_rise = 1.0;
                s.states[2].erection = 0.5;
            },
        ),
    ]
}

pub fn tree(species: &Species, seed: u64) -> telperion_space::Result<Structure> {
    lit(species, Light::NEUTRAL, seed)
}

pub fn lit(species: &Species, light: Light, seed: u64) -> telperion_space::Result<Structure> {
    grow(
        species,
        Request {
            age: AGE,
            seed,
            budget: 2_000_000,
            light,
        },
    )
}

/// The light every light walk stands in but the one it walks: leaves at
/// every angle under the standard overcast sky.
pub const SITE: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// The walk tree in leaf, its limbs and twigs as near certain to live as
/// an oak's boughs and twigs, shade raising their death hazard and
/// shortening their growth units.
pub fn leafy(s: &mut Species) {
    for state in &mut s.states {
        state.leaf_area = 0.3;
    }
    s.states[1].viability = 0.995;
    s.states[2].viability = 0.99;
    for pa in [1, 2] {
        s.states[pa].shade_hazard = 1.0;
        s.states[pa].shade_size = 0.5;
    }
}

fn shaded(name: &str, low: f64, high: f64, edit: fn(&mut Species, f64)) -> Setting {
    let mut walked = setting(format!("light: {name}"), low, high, false, move |s, v| {
        leafy(s);
        edit(s, v)
    });
    walked.light = Box::new(|_| SITE);
    walked
}

/// Every setting walked in leaf under the site's light, the full lay
/// grown with the tree included (host decision 8): `leafy` first, then
/// the setting.
pub fn in_leaf(all: Vec<Setting>) -> Vec<Setting> {
    all.into_iter()
        .map(|walked| {
            let set = walked.set;
            Setting {
                name: format!("in leaf: {}", walked.name),
                set: Box::new(move |s: &mut Species, v: f64| {
                    leafy(s);
                    set(s, v)
                }),
                light: Box::new(|_| SITE),
                ..walked
            }
        })
        .collect()
}

/// fn-197 step 3: light's settings walked on the leafy walk tree: each
/// PA's shade hazard (φ) and shade size (ψ), its leaf area, and the
/// site's extinction and sky.
pub fn light_settings() -> Vec<Setting> {
    let mut all = vec![
        shaded("states[1].shade_hazard", 0.0, 3.0, |s, v| {
            s.states[1].shade_hazard = v
        }),
        shaded("states[2].shade_hazard", 0.0, 3.0, |s, v| {
            s.states[2].shade_hazard = v
        }),
        shaded("states[1].shade_size", 0.0, 2.0, |s, v| {
            s.states[1].shade_size = v
        }),
        shaded("states[2].shade_size", 0.0, 2.0, |s, v| {
            s.states[2].shade_size = v
        }),
        shaded("leaf_area", 0.0, 0.6, |s, v| {
            s.states.iter_mut().for_each(|st| st.leaf_area = v)
        }),
    ];
    let mut sky = shaded("sky", 0.0, 1.0, |_, _| {});
    sky.light = Box::new(|v| Light { sky: v, ..SITE });
    let mut extinction = shaded("extinction", 0.0, 1.0, |_, _| {});
    extinction.light = Box::new(|v| Light {
        extinction: v,
        ..SITE
    });
    all.extend([sky, extinction]);
    all
}
