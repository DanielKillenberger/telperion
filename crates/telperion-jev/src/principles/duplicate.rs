//! Class 2: a function or script the change added or rewrote that shares
//! most of its body with another that survives, and an added early exit that
//! hands back a state other than an error. Extraction to one shared helper
//! and delegation leave no surviving twin, so they never match.

use std::collections::BTreeSet;

use super::candidates::{Candidate, Class, Context, Evidence};
use super::index::{shingles, similarity, Function};

const MIN_TOKENS: usize = 40;
const MIN_SIMILARITY: f64 = 0.4;
const SCRIPT_EXTENSIONS: &[&str] = &["py", "mjs", "ts", "tsx"];

pub fn find(ctx: &Context, head: &[Function], base: &[Function]) -> Vec<Candidate> {
    let mut out = functions(ctx, head, base);
    out.extend(scripts(ctx));
    out.extend(stops(ctx, head));
    out
}

fn touched(ctx: &Context, f: &Function) -> bool {
    ctx.change.adds_within(&f.path, f.lines)
}

fn functions(ctx: &Context, head: &[Function], base: &[Function]) -> Vec<Candidate> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for f in head.iter().filter(|f| f.tokens.len() >= MIN_TOKENS && touched(ctx, f)) {
        let twin = head
            .iter()
            .filter(|g| g.path != f.path && g.tokens.len() >= MIN_TOKENS)
            .map(|g| (similarity(&f.shingles, &g.shingles), g))
            .filter(|(s, _)| *s >= MIN_SIMILARITY)
            .max_by(|a, b| a.0.total_cmp(&b.0));
        let Some((score, twin)) = twin else { continue };
        let pair = if f.symbol < twin.symbol { (&f.symbol, &twin.symbol) } else { (&twin.symbol, &f.symbol) };
        if !seen.insert(pair) {
            continue;
        }
        let mut evidence: Vec<Evidence> = Vec::new();
        evidence.extend(ctx.evidence("after", &f.path, f.lines));
        evidence.extend(ctx.evidence("after", &twin.path, twin.lines));
        let removed = base
            .iter()
            .filter(|b| !head.iter().any(|h| h.symbol == b.symbol))
            .map(|b| (similarity(&f.shingles, &b.shingles), b))
            .filter(|(s, _)| *s >= MIN_SIMILARITY)
            .max_by(|a, b| a.0.total_cmp(&b.0));
        if let Some((_, gone)) = removed {
            evidence.extend(ctx.evidence("before", &gone.path, gone.lines));
        }
        out.push(Candidate {
            id: String::new(),
            class: Class::Duplicate,
            location: format!("{}:{}", f.path, f.lines.0),
            shape: format!(
                "`{}` shares {:.0}% of its body with `{}`, which survives",
                f.symbol,
                100.0 * score,
                twin.symbol
            ),
            weight: 2.0 + score,
            evidence,
        });
    }
    out
}

fn scripts(ctx: &Context) -> Vec<Candidate> {
    let mut out = Vec::new();
    let ext = |p: &str| p.rsplit('.').next().unwrap_or("").to_string();
    for path in &ctx.change.added_files {
        if !SCRIPT_EXTENSIONS.contains(&ext(path).as_str()) || path.contains(".test.") {
            continue;
        }
        let Some(text) = ctx.after.get(path) else { continue };
        let lines = text.lines().count();
        if lines < 30 {
            continue;
        }
        let tokens = shingles(&words(text));
        let twin = ctx
            .after
            .files
            .iter()
            .filter(|(p, _)| *p != path && ext(p) == ext(path) && !p.contains(".test."))
            .map(|(p, t)| (similarity(&tokens, &shingles(&words(t))), p))
            .filter(|(s, _)| *s >= MIN_SIMILARITY)
            .max_by(|a, b| a.0.total_cmp(&b.0));
        let Some((score, twin)) = twin else { continue };
        let mut evidence = Vec::new();
        evidence.extend(ctx.evidence("after", path, (1, lines)));
        evidence.extend(ctx.evidence("after", twin, (1, lines)));
        out.push(Candidate {
            id: String::new(),
            class: Class::Duplicate,
            location: format!("{path}:1"),
            shape: format!("new script `{path}` shares {:.0}% of its text with `{twin}`, which survives", 100.0 * score),
            weight: 2.0 + score,
            evidence,
        });
    }
    out
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect()
}

/// Early returns the change added that hand back a named state, not an
/// error: a pause, a halt or a route to a person. Neighbouring exits of the
/// same function go with it, so the reviewer can see what already stops.
fn stops(ctx: &Context, head: &[Function]) -> Vec<Candidate> {
    let mut out = Vec::new();
    for f in head.iter().filter(|f| touched(ctx, f)) {
        let Some(text) = ctx.after.get(&f.path) else { continue };
        let body: Vec<(usize, &str)> = text
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
            .filter(|(n, _)| (f.lines.0..=f.lines.1).contains(n))
            .collect();
        let exits: Vec<usize> = body
            .iter()
            .filter(|(_, l)| is_state_return(l))
            .map(|(n, _)| *n)
            .collect();
        let added = ctx.change.added.get(&f.path);
        for &n in exits.iter().filter(|n| added.is_some_and(|a| a.contains(n))) {
            let from = n.saturating_sub(3).max(f.lines.0);
            let mut evidence = Vec::new();
            evidence.extend(ctx.evidence("after", &f.path, (from, n + 1)));
            for &m in exits.iter().filter(|&&m| m != n).take(2) {
                evidence.extend(ctx.evidence("after", &f.path, (m.saturating_sub(2), m)));
            }
            out.push(Candidate {
                id: String::new(),
                class: Class::Duplicate,
                location: format!("{}:{n}", f.path),
                shape: format!("an added exit from `{}` hands back a state that stops the work", f.symbol),
                weight: 1.5,
                evidence,
            });
        }
    }
    out
}

/// `return Ok(Route::Owner)`-shaped: a return naming an enum variant that is
/// not `Ok`, `Err`, `Some` or `None` on its own.
fn is_state_return(line: &str) -> bool {
    let t = line.trim_start();
    if !t.starts_with("return ") || t.starts_with("return Err") {
        return false;
    }
    t.split(|c: char| !c.is_alphanumeric() && c != ':' && c != '_')
        .filter_map(|w| w.split_once("::"))
        .any(|(ty, variant)| {
            ty.chars().next().is_some_and(char::is_uppercase)
                && variant.chars().next().is_some_and(char::is_uppercase)
        })
}
