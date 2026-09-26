use serde::{Deserialize, Serialize};

/// Weighted relative distance. Unreadable measurements carry full distance.
pub fn distance(terms: &[(Option<f64>, f64, f64)]) -> Result<f64, String> {
    let mut total = 0.;
    let mut weight = 0.;
    for &(observed, target, w) in terms {
        if !target.is_finite() || target <= 0. || !w.is_finite() || w < 0. {
            return Err("invalid target or weight".into());
        }
        total += observed
            .filter(|v| v.is_finite())
            .map_or(1., |v| (v - target).abs() / target)
            * w;
        weight += w;
    }
    if weight <= 0. || !total.is_finite() || !weight.is_finite() {
        return Err("empty or nonfinite weighted score".into());
    }
    Ok(total / weight)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Cell {
    pub item: String,
    pub view: String,
    pub seed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Visual {
    pub identity: String,
    pub model: String,
    pub ledger: String,
    pub cells: Vec<(Cell, CellStatus)>,
    /// Only blocking defects against the explicit catalogue-quality standard.
    pub defects: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observations: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<super::joint::Finding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub joint: Option<super::joint::Packet>,
    /// The reference-first trait dispositions this assessment bound, kept so a
    /// later assessment can be compared with it. The dispositions themselves
    /// live on the comparison result; only what a comparison needs is copied
    /// here. Empty under any other visual protocol.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage: Vec<TraitStatus>,
    /// The traits the generator cannot draw yet that the request named, with
    /// the reviewer's defects against them; left out of readiness (fn-136).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub known_gaps: Vec<super::unexpressed::KnownGap>,
}

/// One reference-first trait and what the reviewer made of it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TraitStatus {
    pub trait_id: String,
    pub status: CellStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CellStatus {
    Pass,
    Fail,
    Unknown,
}

/// The findings that keep a tree from ready: a blocker, a required unknown
/// or an uncertain support, unless the reviewer tied it to a known gap.
pub fn blocking_findings(assessment: &Visual) -> Vec<&super::joint::Finding> {
    use super::joint::Impact;
    let known = |f: &super::joint::Finding| {
        f.trait_id
            .as_ref()
            .is_some_and(|id| assessment.known_gaps.iter().any(|g| &g.trait_id == id))
    };
    assessment
        .findings
        .iter()
        .filter(|f| {
            matches!(f.impact, Impact::Blocker | Impact::RequiredUnknown)
                || (f.impact == Impact::Supported && f.uncertain)
        })
        .filter(|f| !known(f))
        .collect()
}

pub fn ready(required: &[Cell], identity: &str, assessment: &Visual) -> bool {
    !required.is_empty()
        && !identity.is_empty()
        && assessment.identity == identity
        && !assessment.model.is_empty()
        && !assessment.ledger.is_empty()
        && assessment.defects.is_empty()
        && blocking_findings(assessment).is_empty()
        && assessment.joint.as_ref().is_none_or(|p| {
            !p.inputs
                .iter()
                .any(|i| i.role == "render" && i.framing == super::joint::Framing::Clipped)
                && p.verify_findings(&assessment.findings).is_ok()
                && !assessment.findings.is_empty()
        })
        && assessment.cells.len() == required.len()
        && required.iter().all(|cell| {
            assessment.cells.iter().filter(|(c, _)| c == cell).count() == 1
                && assessment
                    .cells
                    .iter()
                    .any(|(c, pass)| c == cell && *pass == CellStatus::Pass)
        })
        && required
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            == required.len()
}

/// What a revision spent. Spend is always counted and never capped: what
/// ends a loop that keeps spending and keeps nothing is the no-progress stop
/// (`runaway`), never a cap (fn-149).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Budget {
    #[serde(default)]
    pub evaluations: u64,
    #[serde(default)]
    pub images: u64,
    #[serde(default)]
    pub tokens: u64,
    #[serde(default)]
    pub rounds: u64,
    #[serde(default)]
    pub visual_passes: u64,
}

impl Budget {
    pub fn reserve_visual(&mut self) {
        self.visual_passes = self.visual_passes.saturating_add(1);
    }
    pub fn reserve(&mut self, evaluations: u64, images: u64, tokens: u64, rounds: u64) {
        self.evaluations = self.evaluations.saturating_add(evaluations);
        self.images = self.images.saturating_add(images);
        self.tokens = self.tokens.saturating_add(tokens);
        self.rounds = self.rounds.saturating_add(rounds);
    }
}
