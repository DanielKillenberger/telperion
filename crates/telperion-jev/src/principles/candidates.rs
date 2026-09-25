//! Candidates: newly introduced or worsened behaviour of three kinds, each
//! with the before and after spans, callers or consumers, removed
//! implementations and the clauses that govern it. Code finds them; Jev only
//! judges the ones code hands it. Names, cfg gates, cargo features, commands
//! and exports are never a candidate on their own.

use std::path::Path;

use serde::{Deserialize, Serialize};

use super::boundary::{self, Caller};
use super::diff::Change;
use super::index::functions;
use super::modtree::production;
use super::policy::{exceptions, Policy};
use super::source::{wanted, Snapshot};
use super::{duplicate, switch, unread};

/// Bumped whenever extraction changes what it hands the reviewer.
pub const EXTRACTOR_VERSION: &str = "extract-1";
pub const MAX_CANDIDATES: usize = 12;
/// Input tokens one run may send, estimated at four bytes a token.
pub const MAX_INPUT_TOKENS: usize = 8_000;
/// The share of it phase one's evidence may take; phase two re-sends only
/// the cited excerpt of each selected candidate.
const PHASE_ONE_TOKENS: usize = 4_500;
const EXCERPT_LINES: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    /// A setting that selects a builder or suppresses existing structure.
    Switch,
    /// A surviving duplicate implementation, or a redundant blocking step.
    Duplicate,
    /// Output generated or uploaded that no consumer reads.
    Unread,
    /// A current proposal in a spec, judged in spec mode.
    Proposal,
}

impl Class {
    pub fn principles(self) -> &'static [&'static str] {
        match self {
            Class::Switch => &["P-NO-SWITCH", "P-BLOCKING-STEP"],
            Class::Duplicate => &["P-ONE-PIPELINE", "P-NO-FALLBACK", "P-BLOCKING-STEP"],
            Class::Unread => &["P-CONSUMER-READS"],
            Class::Proposal => &[
                "P-ONE-PIPELINE", "P-NO-FALLBACK", "P-NO-SWITCH", "P-CONSUMER-READS", "P-BLOCKING-STEP",
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Evidence {
    /// `e1`, `e2`, ... within its candidate.
    pub id: String,
    pub side: String,
    pub path: String,
    pub lines: (usize, usize),
    pub text: String,
    /// True when the excerpt holds a line the change added: only such an
    /// excerpt can show what a push introduced.
    #[serde(default)]
    pub added: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Candidate {
    pub id: String,
    pub class: Class,
    /// Where the change introduced it: `path:line` at the head.
    pub location: String,
    /// One line code wrote: what shape it found.
    pub shape: String,
    /// Code's ranking weight; higher is kept first under the cap.
    pub weight: f64,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Extraction {
    pub extractor: String,
    pub base: String,
    pub head: String,
    pub candidates: Vec<Candidate>,
    /// Candidates the cap or the token budget dropped.
    pub dropped: usize,
    pub boundary_base: Vec<Caller>,
    pub boundary_head: Vec<Caller>,
    /// Imports at the head the boundary guard cannot follow, `file:line what`.
    #[serde(default)]
    pub boundary_unresolved: Vec<String>,
    /// Changed paths a guard trigger names; the rest are left out.
    pub paths: Vec<String>,
}

impl Extraction {
    /// True when candidates were dropped: the run cannot report clean.
    pub fn overflowed(&self) -> bool {
        self.dropped > 0
    }
}

/// Everything a push introduces between two immutable revisions.
pub fn extract(repo: &Path, base: &str, head: &str, policy: &Policy) -> Result<Extraction, String> {
    let change = Change::of(repo, base, head)?;
    let ex = exceptions();
    // The base is read, parsed and scanned on its own thread; its syntax
    // trees stay there, since only its functions and callers are needed.
    let base_side = || -> Result<_, String> {
        let snap = Snapshot::from_git(repo, base, wanted)?;
        let scan = boundary::scan(&snap, &policy.boundary, &ex);
        let fns = functions(&production(&snap).0);
        Ok((snap, scan, fns))
    };
    let (b, h) = std::thread::scope(|s| {
        let b = s.spawn(base_side);
        let h = Snapshot::from_git(repo, head, wanted);
        (b.join().expect("base side"), h)
    });
    let (before, scan_base, base_fns) = b?;
    let after = h?;
    let scan_head = boundary::scan(&after, &policy.boundary, &ex);
    let (head_files, _) = production(&after);
    let head_fns = functions(&head_files);
    let ctx = Context { change: &change, before: &before, after: &after };
    let mut found = Vec::new();
    found.extend(switch::find(&ctx, &head_files, &base_fns));
    found.extend(duplicate::find(&ctx, &head_fns, &base_fns));
    found.extend(unread::find(&ctx, &head_files));
    let (candidates, dropped) = bound(found);
    Ok(Extraction {
        extractor: EXTRACTOR_VERSION.into(),
        base: base.into(),
        head: head.into(),
        candidates,
        dropped,
        boundary_base: scan_base.violations,
        boundary_unresolved: scan_head
            .unresolved
            .iter()
            .map(|u| format!("{}:{} {}", u.file, u.line, u.what))
            .chain(scan_head.parse_errors.iter().map(|e| format!("{} does not parse: {}", e.path, e.message)))
            .collect(),
        boundary_head: scan_head.violations,
        paths: change
            .paths()
            .filter(|p| {
                let t = policy.entry_triggers.iter().chain(&policy.artifact_triggers);
                t.clone().any(|t| p.starts_with(t.as_str()))
            })
            .cloned()
            .collect(),
    })
}

/// What an extractor reads: the change and both sides of it.
pub struct Context<'a> {
    pub change: &'a Change,
    pub before: &'a Snapshot,
    pub after: &'a Snapshot,
}

impl Context<'_> {
    /// Up to `EXCERPT_LINES` lines of `path` from `lines.0`, on one side.
    pub fn excerpt(&self, side: &str, path: &str, lines: (usize, usize)) -> Option<(String, (usize, usize))> {
        let snap = if side == "before" { self.before } else { self.after };
        let text = snap.get(path)?;
        let end = lines.1.min(lines.0 + EXCERPT_LINES - 1).max(lines.0);
        let body: Vec<&str> = text.lines().skip(lines.0.saturating_sub(1)).take(end + 1 - lines.0).collect();
        Some((body.join("\n"), (lines.0, end)))
    }

    pub fn evidence(&self, side: &str, path: &str, lines: (usize, usize)) -> Option<Evidence> {
        let (text, lines) = self.excerpt(side, path, lines)?;
        let added = side == "after" && self.change.adds_within(path, lines);
        Some(Evidence { id: String::new(), side: side.into(), path: path.into(), lines, text, added })
    }
}

/// Keeps the heaviest candidates inside the count cap and the token budget,
/// numbers them and their evidence, and counts what was dropped.
pub fn bound(mut found: Vec<Candidate>) -> (Vec<Candidate>, usize) {
    found.sort_by(|a, b| b.weight.total_cmp(&a.weight).then(a.location.cmp(&b.location)));
    let total = found.len();
    let mut kept = Vec::new();
    let mut bytes = 0;
    for mut c in found.into_iter().take(MAX_CANDIDATES) {
        let size: usize = c.evidence.iter().map(|e| e.text.len() + 60).sum::<usize>() + 200;
        if (bytes + size) / 4 > PHASE_ONE_TOKENS {
            continue;
        }
        bytes += size;
        c.id = format!("c{}", kept.len() + 1);
        for (i, e) in c.evidence.iter_mut().enumerate() {
            e.id = format!("e{}", i + 1);
        }
        kept.push(c);
    }
    let dropped = total - kept.len();
    (kept, dropped)
}
