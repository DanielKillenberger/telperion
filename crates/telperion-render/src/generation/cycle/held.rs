//! The contract's functions whose results the units floor or round into
//! a stored integer or a decision's (DESIGN.md section 8, "Where a float
//! becomes an integer"): the wake's ticks, the time a failed unit lived
//! and its part of a tick, the survival and abortion thresholds and the
//! shade they read. Each is copied from `decide.wgsl` itself as
//! `<name>_held`, with every marked operation it reaches held to the
//! CPU's order (`held.wgsl`) in both arithmetics; every other float stays
//! plain.
use std::collections::BTreeSet;

const CONTRACT: &str = include_str!("decide.wgsl");

/// The functions the units call held.
const ROOTS: [&str; 7] = [
    "wake_ticks",
    "lived_ticks",
    "lived_fine",
    "neg_log2",
    "survival_fine",
    "abortion_fine",
    "exp2c",
];

/// Each function of `source`, by name, as written.
fn functions(source: &str) -> Vec<(&str, &str)> {
    let mut out = Vec::new();
    for (at, _) in source.match_indices("\nfn ") {
        let start = at + 1;
        let name_end = start + source[start..].find('(').expect("a parameter list");
        let end = start + source[start..].find("\n}\n").expect("a closing brace") + 3;
        out.push((&source[start + 3..name_end], &source[start..end]));
    }
    out
}

/// Whether `body` calls `name`.
fn calls(body: &str, name: &str) -> bool {
    let mut word = String::new();
    for c in body.chars() {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            continue;
        }
        if c == '(' && word == name {
            return true;
        }
        word.clear();
    }
    false
}

/// `body` with its marks held and each call of a name in `names` made to
/// its held copy.
fn renamed(body: &str, names: &BTreeSet<&str>) -> String {
    let body = body
        .replace("exact_kept(", "held_kept(")
        .replace("exact_div(", "held_div(");
    let (mut out, mut word) = (String::new(), String::new());
    for c in body.chars() {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            continue;
        }
        out.push_str(&word);
        if c == '(' && names.contains(word.as_str()) {
            out.push_str("_held");
        }
        word.clear();
        out.push(c);
    }
    out + &word
}

/// The held copies of the roots and of every function they reach whose
/// operations are marked.
pub(super) fn source() -> String {
    let fns = functions(CONTRACT);
    let marked = |body: &str| body.contains("exact_kept(") || body.contains("exact_div(");
    let mut reached: BTreeSet<&str> = ROOTS.into_iter().collect();
    loop {
        let more: Vec<&str> = fns
            .iter()
            .filter(|(n, _)| {
                !reached.contains(n) && reached.iter().any(|r| calls(body_of(&fns, r), n))
            })
            .map(|f| f.0)
            .collect();
        if more.is_empty() {
            break;
        }
        reached.extend(more);
    }
    // A function is held where it, or one it calls, holds a mark.
    let mut held: BTreeSet<&str> = reached
        .iter()
        .copied()
        .filter(|n| marked(body_of(&fns, n)))
        .collect();
    loop {
        let more: Vec<&str> = reached
            .iter()
            .copied()
            .filter(|n| !held.contains(n) && held.iter().any(|h| calls(body_of(&fns, n), h)))
            .collect();
        if more.is_empty() {
            break;
        }
        held.extend(more);
    }
    assert!(
        ROOTS.iter().all(|r| held.contains(r)),
        "a root with nothing to hold"
    );
    let copies: Vec<String> = fns
        .iter()
        .filter(|(n, _)| held.contains(n))
        .map(|(_, b)| renamed(b, &held))
        .collect();
    copies.join("\n")
}

fn body_of<'a>(fns: &[(&'a str, &'a str)], name: &str) -> &'a str {
    fns.iter().find(|f| f.0 == name).map_or("", |f| f.1)
}

#[cfg(test)]
mod tests {
    use super::source;

    #[test]
    fn the_held_copies_hold_every_mark_they_reach() {
        let held = source();
        assert!(
            !held.contains("exact_kept(") && !held.contains("exact_div("),
            "a mark left plain"
        );
        for name in [
            "wake_ticks_held",
            "lived_ticks_held",
            "neg_ln1m_held",
            "log2c_held",
            "lnc_held",
            "survival_fine_held",
        ] {
            assert!(held.contains(&format!("fn {name}(")), "{name} missing");
        }
        assert!(
            !held.contains("fn pow2_held("),
            "an unmarked function copied"
        );
    }
}
