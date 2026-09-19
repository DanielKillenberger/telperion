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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CellStatus {
    Pass,
    Fail,
    Unknown,
}

pub fn ready(required: &[Cell], identity: &str, assessment: &Visual) -> bool {
    !required.is_empty()
        && !identity.is_empty()
        && assessment.identity == identity
        && !assessment.model.is_empty()
        && !assessment.ledger.is_empty()
        && assessment.defects.is_empty()
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

/// Reservations are charged before dispatch and persisted, including interrupted work.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Budget {
    pub evaluations: u64,
    pub images: u64,
    pub tokens: u64,
    pub rounds: u64,
    pub max_evaluations: u64,
    pub max_images: u64,
    pub max_tokens: u64,
    pub max_rounds: u64,
}

impl Budget {
    pub fn reserve(
        &mut self,
        evaluations: u64,
        images: u64,
        tokens: u64,
        rounds: u64,
    ) -> Result<(), String> {
        let next = [
            self.evaluations.checked_add(evaluations),
            self.images.checked_add(images),
            self.tokens.checked_add(tokens),
            self.rounds.checked_add(rounds),
        ];
        let limits = [
            self.max_evaluations,
            self.max_images,
            self.max_tokens,
            self.max_rounds,
        ];
        if next
            .iter()
            .zip(limits)
            .any(|(v, limit)| v.is_none_or(|v| v > limit))
        {
            return Err("hard budget exhausted".into());
        }
        self.evaluations = next[0].unwrap();
        self.images = next[1].unwrap();
        self.tokens = next[2].unwrap();
        self.rounds = next[3].unwrap();
        Ok(())
    }
}
