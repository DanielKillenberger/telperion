//! The decision rule. A candidate warns only when Jev's phase-one mechanism
//! clears its principle's cut, phase two finds the breach shown above its
//! cut and covered by no legitimate reading or exception below its cut
//! (never a product of the two), and code validates the excerpt Jev cited
//! as one the change added.
//! Anything else abstains. A principle's mode decides whether a warning is
//! logged (shadow), shown (warn) or blocks (block); every mode starts shadow.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ask::{principle_of, INSUFFICIENT, NONE};
use super::candidates::{Candidate, Class, Extraction};
use super::policy::Policy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Shadow,
    Warn,
    Block,
}

impl Mode {
    pub fn of(text: &str) -> Mode {
        match text {
            "warn" => Mode::Warn,
            "block" => Mode::Block,
            _ => Mode::Shadow,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Finding {
    pub candidate: String,
    pub class: Class,
    pub principle: String,
    pub mechanism: String,
    pub location: String,
    /// The excerpts Jev cited, validated by code.
    pub evidence: Vec<String>,
    pub select: f64,
    pub confirm: f64,
    pub covered: f64,
    pub mode: Mode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Outcome {
    /// Every finding that cleared both cuts, whatever its principle's mode.
    pub findings: Vec<Finding>,
    /// Candidate id and why no finding came of it.
    pub abstained: Vec<(String, String)>,
    /// Set when the run cannot say clean: overflow, timeout, no key.
    pub incomplete: Option<String>,
}

impl Outcome {
    pub fn shown(&self) -> impl Iterator<Item = &Finding> {
        self.findings.iter().filter(|f| f.mode != Mode::Shadow)
    }
    pub fn blocking(&self) -> impl Iterator<Item = &Finding> {
        self.findings.iter().filter(|f| f.mode == Mode::Block)
    }
}

/// A candidate phase one selected, with the probability of its mechanism.
#[derive(Debug, Clone)]
pub struct Selected<'a> {
    pub candidate: &'a Candidate,
    pub mechanism: String,
    pub probability: f64,
    pub evidence: String,
}

fn probability(answers: &Value, question: &str, option: &str) -> f64 {
    answers
        .get(question)
        .and_then(|a| a.get("probabilities"))
        .and_then(|p| p.get(option))
        .and_then(Value::as_f64)
        .unwrap_or(0.0)
}

fn choice(answers: &Value, question: &str) -> Option<String> {
    answers.get(question)?.get("choice")?.as_str().map(str::to_string)
}

/// Phase one's top mechanism per candidate; `none` and
/// `insufficient_evidence` select nothing. With `cut`, only a mechanism at
/// or above its principle's `select_min` is kept.
pub fn select<'a>(x: &'a Extraction, phase_one: &Value, policy: &Policy, cut: bool) -> (Vec<Selected<'a>>, Vec<(String, String)>) {
    let mut picked = Vec::new();
    let mut abstained = Vec::new();
    for c in &x.candidates {
        let question = format!("{}_mechanism", c.id);
        let Some(mechanism) = choice(phase_one, &question) else {
            abstained.push((c.id.clone(), "no phase-one answer".into()));
            continue;
        };
        if mechanism == NONE || mechanism == INSUFFICIENT {
            abstained.push((c.id.clone(), mechanism));
            continue;
        }
        let p = probability(phase_one, &question, &mechanism);
        let principle = principle_of(c.class, &mechanism).and_then(|id| policy.principle(id));
        let Some(principle) = principle else {
            abstained.push((c.id.clone(), format!("unknown mechanism {mechanism}")));
            continue;
        };
        if cut && p < principle.select_min {
            abstained.push((c.id.clone(), format!("{mechanism} at {p:.2} under {:.2}", principle.select_min)));
            continue;
        }
        let evidence = choice(phase_one, &format!("{}_evidence", c.id)).unwrap_or_default();
        picked.push(Selected { candidate: c, mechanism, probability: p, evidence });
    }
    (picked, abstained)
}

/// Phase two's verdicts on the selected candidates, with code's check that
/// the cited excerpt exists and holds a line the change added.
pub fn decide(x: &Extraction, picked: &[Selected], phase_two: &Value, policy: &Policy) -> Outcome {
    let mut out = Outcome::default();
    for s in picked {
        let c = s.candidate;
        let principle = principle_of(c.class, &s.mechanism).and_then(|id| policy.principle(id));
        let Some(principle) = principle else { continue };
        if s.probability < principle.select_min {
            out.abstained.push((c.id.clone(), format!("{} under its cut", s.mechanism)));
            continue;
        }
        let noul = |q: &str| phase_two.get(format!("{}_{q}", c.id)).and_then(|a| a.get("noul")).and_then(Value::as_f64);
        let (Some(confirm), Some(covered)) = (noul("shown"), noul("covered")) else {
            out.abstained.push((c.id.clone(), "no confirming answer".into()));
            continue;
        };
        if confirm < principle.confirm_min {
            out.abstained.push((c.id.clone(), format!("not shown ({confirm:.2})")));
            continue;
        }
        if covered > principle.cover_max {
            out.abstained.push((c.id.clone(), format!("a legitimate reading or exception covers it ({covered:.2})")));
            continue;
        }
        let cited = c.evidence.iter().find(|e| e.id == s.evidence);
        let valid = cited.is_some_and(|e| e.added || c.class == Class::Proposal);
        if !valid {
            out.abstained.push((c.id.clone(), format!("cited excerpt `{}` shows nothing the change added", s.evidence)));
            continue;
        }
        let e = cited.expect("validated above");
        out.findings.push(Finding {
            candidate: c.id.clone(),
            class: c.class,
            principle: principle.id.clone(),
            mechanism: s.mechanism.clone(),
            location: c.location.clone(),
            evidence: vec![format!("{} {}:{}-{}", e.side, e.path, e.lines.0, e.lines.1)],
            select: s.probability,
            confirm,
            covered,
            mode: Mode::of(&principle.mode),
        });
    }
    if x.overflowed() {
        out.incomplete = Some(format!("{} candidates over the cap or the token budget", x.dropped));
    }
    out
}

/// A principle's warnings move shadow to warn, and warn to block, only when
/// the owner's verdicts on its findings from real pushes reach this
/// precision over at least `GRADUATION_MIN` reviewed findings.
pub const GRADUATION_PRECISION: f64 = 0.95;
pub const GRADUATION_MIN: usize = 20;

pub fn may_advance(confirmed: usize, dismissed: usize) -> bool {
    let reviewed = confirmed + dismissed;
    reviewed >= GRADUATION_MIN && confirmed as f64 >= GRADUATION_PRECISION * reviewed as f64
}
