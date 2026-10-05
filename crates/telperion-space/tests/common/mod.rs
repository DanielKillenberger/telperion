//! Shared by the oracle and closed-form tests: the oracle fixture, its
//! parameter sets read as species, a tree's canonical signature and the
//! moments of a sample.
#![allow(dead_code)]
use serde_json::Value;
use std::f64::consts::PI;
use telperion_space::{
    grow, CountTable, Form, NodeLaw, Origin, PaState, Request, Species, Structure, Zone,
};

pub const BUDGET: u32 = 1_000_000;

pub fn sets() -> Vec<Value> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/greenlab-oracle.json"
    );
    let fixture: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    fixture["sets"].as_array().unwrap().clone()
}

/// Each age of the species a set is read as, by the simulator's PA it
/// stands for (from 0). The reference axis has no loops (fn-206), so a PA
/// that turns into itself, renewing its apex, is unrolled into as many
/// ages as the tree's cycles need, each moving on to the next.
pub fn ages(set: &Value) -> Vec<usize> {
    let max = age(set) as usize;
    let mut ages = Vec::new();
    for (i, p) in set["pa"].as_array().unwrap().iter().enumerate() {
        let renews = p["terminal"].as_u64().unwrap() as usize == i + 1;
        let lifespan = p["macro"].as_u64().unwrap() as usize;
        let copies = if renews { max.div_ceil(lifespan) } else { 1 };
        ages.extend(std::iter::repeat_n(i, copies));
    }
    ages
}

/// A simulator parameter set as a species: one zone per growth unit, one
/// bud per node (alternate) or two (opposite), every apex immortal and
/// nothing shed. The geometry is ours: distichous, so the tree is planar.
/// The simulator's transitions all go to the next PA or to the PA itself
/// (`ages`).
pub fn species(set: &Value) -> Species {
    let pas = set["pa"].as_array().unwrap();
    let ages = ages(set);
    let first = |pa: usize| ages.iter().position(|&a| a == pa).unwrap();
    let buds = if set["variant"] == "opposite" {
        2.0
    } else {
        1.0
    };
    let poisson = set["poisson"].as_bool().unwrap();
    let states = ages
        .iter()
        .enumerate()
        .map(|(at, &i)| {
            let p = &pas[i];
            let int = |key: &str| p[key].as_u64().unwrap() as u32;
            let nodes = if poisson {
                NodeLaw::Poisson {
                    mean: p["lambda"].as_f64().unwrap(),
                }
            } else {
                NodeLaw::Uniform {
                    min: f64::from(int("nmin")),
                    max: f64::from(int("nmax")),
                }
            };
            let mut lateral = vec![0.0; ages.len()];
            for (j, prob) in p["p"].as_object().unwrap() {
                lateral[first(j.parse::<usize>().unwrap() - 1)] = prob.as_f64().unwrap();
            }
            let terminal = int("terminal") as usize;
            assert!(terminal <= i + 2, "the simulator's PA {} jumps", i + 1);
            // It moves on to the next PA, or to its own next copy.
            let moves = terminal == i + 2 || (terminal == i + 1 && ages.get(at + 1) == Some(&i));
            PaState {
                lifespan: f64::from(int("macro")),
                continuation: if moves { 1.0 } else { 0.0 },
                viability: 1.0,
                zones: vec![Zone {
                    nodes,
                    buds,
                    dormant: vec![0.0; lateral.len()],
                    delay: 0.0,
                    rate: 0.0,
                    lateral,
                }],
                shedding: f64::INFINITY,
                internode: 1.0 / (1.0 + 0.45 * i as f64),
                insertion: (50.0 - 6.0 * i as f64).to_radians(),
                divergence: PI,
                abortion: 0.0,
                abortion_rise: 0.0,
                relay: 0.0,
                relay_ended: 0.0,
                relay_failed: 0.0,
                relay_at: 1.0,
                epitony: 0.0,
                erection: 0.0,
                readiness: 1.0,
                rhythm: 1.0,
                leaf_area: 0.0,
                shade_hazard: 0.0,
                shade_size: 0.0,
                apical_control: 0.5,
                upkeep: 0.0,
                balance_hazard: 0.0,
                tolerance: 0.0,
                retained: 0.0,
                leaf_girth: 0.0,
                straightening: 0.0,
                form: Form::default(),
            }
        })
        .collect();
    Species { states }
}

/// A count per age folded back onto the simulator's PAs.
pub fn fold(set: &Value, table: &CountTable) -> CountTable {
    let ages = ages(set);
    let pas = set["pa"].as_array().unwrap().len();
    let mut folded = CountTable::new(pas, table.cycles);
    for (at, &pa) in ages.iter().enumerate() {
        for cycle in 1..=table.cycles {
            folded.add(pa, cycle, table.get(at, cycle));
        }
    }
    folded
}

pub fn age(set: &Value) -> u32 {
    set["maxCA"].as_u64().unwrap() as u32
}

pub fn tree(species: &Species, age: u32, seed: u64) -> Structure {
    grow(
        species,
        Request {
            age,
            seed,
            budget: BUDGET,
            light: telperion_space::Light::NEUTRAL,
        },
    )
    .unwrap()
}

/// The canonical string the oracle script writes for the simulator's trees:
/// an axis is its PA (from 1), its growth units in order, each the sorted
/// strings of its phytomers' laterals, then its continuation; an axis that
/// grew nothing is empty.
pub fn signature(tree: &Structure, ages: &[usize]) -> String {
    let mut laterals = vec![Vec::new(); tree.axes.len()];
    let mut continuation = vec![None; tree.axes.len()];
    for (i, axis) in tree.axes.iter().enumerate() {
        match axis.origin {
            Origin::Seed => {}
            Origin::Lateral { parent, node, .. } => laterals[parent].push((node, i)),
            Origin::Continuation { parent } => continuation[parent] = Some(i),
            Origin::Relay { .. } => panic!("the simulators make no relays"),
        }
    }
    word(tree, 0, &laterals, &continuation, ages)
}

fn word(
    tree: &Structure,
    i: usize,
    lat: &[Vec<(usize, usize)>],
    next: &[Option<usize>],
    ages: &[usize],
) -> String {
    let axis = &tree.axes[i];
    let tail = next[i]
        .map(|c| word(tree, c, lat, next, ages))
        .unwrap_or_default();
    if axis.phytomers.is_empty() && tail.is_empty() {
        return String::new();
    }
    let mut units: Vec<(u32, Vec<String>)> = Vec::new();
    for (n, phytomer) in axis.phytomers.iter().enumerate() {
        let mut kids: Vec<String> = lat[i]
            .iter()
            .filter(|(node, _)| *node == n)
            .map(|&(_, k)| word(tree, k, lat, next, ages))
            .filter(|s| !s.is_empty())
            .collect();
        kids.sort();
        let entry = format!("p[{}]", kids.join(";"));
        match units.last_mut() {
            Some((cycle, words)) if *cycle == phytomer.cycle => words.push(entry),
            _ => units.push((phytomer.cycle, vec![entry])),
        }
    }
    let body: String = units
        .iter_mut()
        .map(|(_, words)| {
            words.sort();
            format!("({})", words.join(","))
        })
        .collect();
    let tail = if tail.is_empty() {
        tail
    } else {
        format!(">{tail}")
    };
    format!("A{}{body}{tail}", ages[axis.pa] + 1)
}

pub fn fnv1a(text: &str) -> String {
    let hash = text.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    format!("{hash:016x}")
}

/// Mean, variance and fourth central moment, as the oracle script records them.
#[derive(Debug, Clone, Copy)]
pub struct Moments {
    pub n: f64,
    pub mean: f64,
    pub var: f64,
    pub m4: f64,
}

impl Moments {
    pub fn of(values: &[f64]) -> Self {
        let n = values.len() as f64;
        let mean = values.iter().sum::<f64>() / n;
        let central = |k: i32| values.iter().map(|v| (v - mean).powi(k)).sum::<f64>() / n;
        Self {
            n,
            mean,
            var: central(2),
            m4: central(4),
        }
    }

    pub fn read(value: &Value, n: f64) -> Self {
        let get = |key: &str| value[key].as_f64().unwrap();
        Self {
            n,
            mean: get("mean"),
            var: get("var"),
            m4: get("m4"),
        }
    }

    /// Standard errors of the mean and of the variance; the variance's is
    /// (m4 - var^2 (n-3)/(n-1)) / n, which a two-point count does not zero.
    pub fn errors(&self) -> (f64, f64) {
        let n = self.n;
        let var_error =
            ((self.m4 - self.var * self.var * (n - 3.0) / (n - 1.0)).max(0.0) / n).sqrt();
        ((self.var / n).sqrt(), var_error)
    }
}
