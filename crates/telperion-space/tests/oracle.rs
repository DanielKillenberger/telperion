//! R3: Letort's GreenLab simulators, run unchanged in headless Chromium
//! (scripts/greenlab-oracle.mjs), are the oracle. Deterministic sets match
//! tree for tree; stochastic sets match in the mean and the spread of every
//! count per PA and cycle, over as many seeds as the oracle ran.
mod common;
use common::{age, fnv1a, sets, species, tree, Moments};
use serde_json::Value;
use telperion_space::expected_counts;

/// Standard errors a difference may span. About 330 comparisons; a true
/// match exceeds 4.5 in any of them with probability near 0.2 percent.
const Z: f64 = 4.5;

fn deterministic(set: &Value) -> bool {
    set["deterministic"].as_bool().unwrap_or(false)
}

#[test]
fn deterministic_runs_grow_the_simulators_trees_exactly() {
    let fixed: Vec<Value> = sets().into_iter().filter(deterministic).collect();
    assert_eq!(fixed.len(), 3);
    for set in &fixed {
        let grown = tree(&species(set), age(set), 1);
        let counts = grown.counts();
        for (pa, row) in set["counts"].as_array().unwrap().iter().enumerate() {
            for (c, oracle) in row.as_array().unwrap().iter().enumerate() {
                let ours = counts.get(pa, c + 1);
                assert_eq!(
                    ours,
                    oracle.as_f64().unwrap(),
                    "{}: PA {} cycle {}",
                    set["name"],
                    pa + 1,
                    c + 1
                );
            }
        }
        let signature = common::signature(&grown);
        assert_eq!(
            signature.len() as u64,
            set["signature_length"].as_u64().unwrap(),
            "{}",
            set["name"]
        );
        assert_eq!(
            fnv1a(&signature),
            set["signature"].as_str().unwrap(),
            "{}",
            set["name"]
        );
    }
}

/// A difference in standard errors; a difference with no spread on either
/// side must be none.
fn z(label: &str, delta: f64, ours: f64, theirs: f64) -> f64 {
    let spread = ours.hypot(theirs);
    if spread == 0.0 {
        assert!(delta.abs() < 1e-9, "{label}: {delta} with no spread");
        return 0.0;
    }
    delta.abs() / spread
}

/// The z-scores of the differences in mean and in variance.
fn scores(label: &str, ours: Moments, theirs: Moments) -> (f64, f64) {
    let (ours_mean, ours_var) = ours.errors();
    let (theirs_mean, theirs_var) = theirs.errors();
    (
        z(label, ours.mean - theirs.mean, ours_mean, theirs_mean),
        z(label, ours.var - theirs.var, ours_var, theirs_var),
    )
}

#[test]
fn stochastic_structures_match_the_simulators_in_distribution() {
    let mut worst = (0.0f64, String::new());
    let mut compared = 0;
    for set in sets().iter().filter(|s| !deterministic(s)) {
        let name = set["name"].as_str().unwrap();
        let count = set["seeds"]["count"].as_u64().unwrap();
        let species = species(set);
        let tables: Vec<_> = (0..count)
            .map(|seed| tree(&species, age(set), seed).counts())
            .collect();
        let pas = species.states.len();
        for pa in 0..pas {
            let totals: Vec<f64> = tables.iter().map(|t| t.total(pa)).collect();
            let mut cells = vec![(
                format!("{name} PA {} total", pa + 1),
                totals,
                &set["oracle"]["totals"][pa],
            )];
            for cycle in 1..=age(set) as usize {
                let values = tables.iter().map(|t| t.get(pa, cycle)).collect();
                let oracle = &set["oracle"]["cells"][pa][cycle - 1];
                cells.push((
                    format!("{name} PA {} cycle {cycle}", pa + 1),
                    values,
                    oracle,
                ));
            }
            for (label, values, oracle) in cells {
                let (z_mean, z_var) = scores(
                    &label,
                    Moments::of(&values),
                    Moments::read(oracle, count as f64),
                );
                compared += 2;
                for (z, what) in [(z_mean, "mean"), (z_var, "variance")] {
                    if z > worst.0 {
                        worst = (z, format!("{label} {what}"));
                    }
                }
            }
        }
    }
    println!(
        "{compared} comparisons, the worst {:.2} standard errors: {}",
        worst.0, worst.1
    );
    assert!(
        worst.0 <= Z,
        "{} differs by {:.2} standard errors",
        worst.1,
        worst.0
    );
}

#[test]
fn the_closed_form_expectation_is_the_simulators_mean() {
    for set in sets().iter().filter(|s| !deterministic(s)) {
        let expected = expected_counts(&species(set), age(set)).unwrap();
        let count = set["seeds"]["count"].as_f64().unwrap();
        for pa in 0..expected.pas {
            for cycle in 1..=expected.cycles {
                let oracle = Moments::read(&set["oracle"]["cells"][pa][cycle - 1], count);
                let label = format!("{} PA {} cycle {cycle}", set["name"], pa + 1);
                let (error, _) = oracle.errors();
                let z_mean = z(&label, expected.get(pa, cycle) - oracle.mean, 0.0, error);
                assert!(z_mean <= Z, "{label}: {:.2} standard errors", z_mean);
            }
        }
    }
}
