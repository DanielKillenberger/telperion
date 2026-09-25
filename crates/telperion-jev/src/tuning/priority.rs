//! Human chooses objectives, not parameter values. Approval is not visual acceptance.
use super::{
    evaluation::Image,
    joint::Impact,
    state::{Cell, Visual},
};
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub id: String,
    pub role: String,
    pub image: Image,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub id: String,
    pub observation: String,
    pub evidence_ids: Vec<String>,
    pub views: Vec<String>,
    /// The track this objective belongs to. None belongs to every track.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track: Option<String>,
}
impl Gap {
    /// Whether this objective is one the named track tunes toward.
    pub fn belongs_to(&self, track: &str) -> bool {
        self.track.as_deref().is_none_or(|t| t == track)
    }
    /// The same objective, whatever track the approval assigned it.
    fn same(&self, other: &Gap) -> bool {
        Gap {
            track: None,
            ..self.clone()
        } == Gap {
            track: None,
            ..other.clone()
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub run_identity: String,
    pub scope_sha256: String,
    pub visual: Visual,
    pub evidence: Vec<Evidence>,
    pub gaps: Vec<Gap>,
    pub proposed_top_three: Vec<String>,
    /// The approval this pause proposes: the owner's priorities, then every
    /// expressible core and secondary inventory trait, in inventory order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub proposed: Vec<Gap>,
    /// Inventory traits that are not objectives, and why.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub left_out: Vec<super::objectives::LeftOut>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Approval {
    pub checkpoint_sha256: String,
    pub scope_sha256: String,
    pub ordered: Vec<Gap>,
}

/// How many objectives one approval may order.
pub const MAX_OBJECTIVES: usize = 32;

pub fn scope(species: &str, objectives: &str, required: &[Cell], references: &[Image]) -> String {
    sha256_hex(&serde_json::to_vec(&json!({"species":species,"objectives":objectives,"required":required,"references":references.iter().map(|i|json!({"sha256":i.sha256,"view":i.view,"seed":i.seed})).collect::<Vec<_>>()})).unwrap())
}
fn text(s: &str) -> bool {
    !s.trim().is_empty() && s.len() <= 2048
}
fn gap_valid(gap: &Gap, evidence: &[Evidence]) -> bool {
    text(&gap.id)
        && text(&gap.observation)
        && !gap.evidence_ids.is_empty()
        && gap.evidence_ids.len() <= 12
        && gap.evidence_ids.iter().collect::<HashSet<_>>().len() == gap.evidence_ids.len()
        && gap
            .evidence_ids
            .iter()
            .all(|id| evidence.iter().any(|e| &e.id == id))
        && !gap.views.is_empty()
        && gap.views.iter().collect::<HashSet<_>>().len() == gap.views.len()
        && gap.views.iter().all(|v| {
            evidence
                .iter()
                .any(|e| e.role == "render" && &e.image.view == v)
        })
}
impl Checkpoint {
    pub fn new(
        run_identity: &str,
        scope_sha256: &str,
        visual: Visual,
        evidence: Vec<Evidence>,
    ) -> Result<Self, String> {
        let gaps = visual
            .findings
            .iter()
            .enumerate()
            .filter(|(_, f)| f.impact != Impact::Supported)
            .filter_map(|(i, f)| {
                let mut views = Vec::new();
                for e in &evidence {
                    if e.role == "render"
                        && f.evidence_ids.contains(&e.id)
                        && !views.contains(&e.image.view)
                    {
                        views.push(e.image.view.clone());
                    }
                }
                (!views.is_empty()).then(|| Gap {
                    id: format!("finding-{i}"),
                    observation: f.observation.clone(),
                    evidence_ids: f.evidence_ids.clone(),
                    views,
                    track: None,
                })
            })
            .collect::<Vec<_>>();
        let proposed_top_three = gaps.iter().take(3).map(|g| g.id.clone()).collect();
        let value = Self {
            run_identity: run_identity.into(),
            scope_sha256: scope_sha256.into(),
            visual,
            evidence,
            gaps,
            proposed_top_three,
            proposed: vec![],
            left_out: vec![],
        };
        value.verify()?;
        Ok(value)
    }
    pub fn hash(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).unwrap())
    }
    pub fn verify(&self) -> Result<(), String> {
        if !text(&self.run_identity)
            || self.scope_sha256.len() != 64
            || self.visual.identity.is_empty()
            || self.visual.ledger.is_empty()
            || self.evidence.is_empty()
            || self.evidence.len() > 24
        {
            return Err("incomplete priority evidence packet".into());
        }
        let mut ids = HashSet::new();
        for e in &self.evidence {
            if !text(&e.id)
                || !ids.insert(&e.id)
                || !matches!(e.role.as_str(), "render" | "reference" | "anchor")
            {
                return Err("invalid priority evidence identity".into());
            }
            e.image.verify()?;
        }
        if !["render", "reference"]
            .iter()
            .all(|role| self.evidence.iter().any(|e| &e.role == role))
        {
            return Err("priority checkpoint needs render and reference evidence".into());
        }
        if self.gaps.len() > 16
            || self.gaps.iter().any(|g| !gap_valid(g, &self.evidence))
            || self.proposed_top_three
                != self
                    .gaps
                    .iter()
                    .take(3)
                    .map(|g| g.id.clone())
                    .collect::<Vec<_>>()
        {
            return Err("invalid proposed gap ranking".into());
        }
        let mut proposed = HashSet::new();
        if self.proposed.len() > MAX_OBJECTIVES
            || self
                .proposed
                .iter()
                .any(|g| !proposed.insert(&g.id) || !gap_valid(g, &self.evidence))
        {
            return Err("invalid proposed objectives".into());
        }
        Ok(())
    }
}
impl Approval {
    pub fn verify(&self, checkpoint: &Checkpoint, current_scope: &str) -> Result<(), String> {
        checkpoint.verify()?;
        if self.checkpoint_sha256 != checkpoint.hash()
            || self.scope_sha256 != checkpoint.scope_sha256
            || self.scope_sha256 != current_scope
            || self.ordered.len() > MAX_OBJECTIVES
        {
            return Err("missing or stale owner priority scope".into());
        }
        let mut ids = HashSet::new();
        for gap in &self.ordered {
            if !ids.insert(&gap.id) || !gap_valid(gap, &checkpoint.evidence) {
                return Err("invalid owner priority".into());
            }
            // The owner's own priorities are theirs to reword; a finding or an
            // inventory trait is approved as offered, a track aside.
            let mut offered = checkpoint.gaps.iter().chain(&checkpoint.proposed);
            match offered.find(|g| g.id == gap.id) {
                _ if gap.id.starts_with("owner-") => {}
                Some(original) if !original.same(gap) => {
                    return Err("modified reviewer gap must be an attributed owner addition".into());
                }
                Some(_) => {}
                None => return Err("new gap must use owner- identity".into()),
            }
            if !["render", "reference"].iter().all(|role| {
                gap.evidence_ids.iter().any(|id| {
                    checkpoint
                        .evidence
                        .iter()
                        .any(|e| &e.id == id && &e.role == role)
                })
            }) {
                return Err("owner priority needs render/reference citations".into());
            }
        }
        Ok(())
    }
}
pub fn requirements(base: &[Cell], approval: Option<&Approval>) -> Vec<Cell> {
    let mut cells = base.to_vec();
    if let Some(approval) = approval {
        let mut views = Vec::new();
        for cell in base {
            let pair = (cell.view.clone(), cell.seed);
            if !views.contains(&pair) {
                views.push(pair);
            }
        }
        for gap in &approval.ordered {
            for (view, seed) in views.iter().filter(|(v, _)| gap.views.contains(v)) {
                cells.push(Cell {
                    item: format!("owner-priority:{}: {}", gap.id, gap.observation),
                    view: view.clone(),
                    seed: *seed,
                });
            }
        }
    }
    cells
}

/// Preserve packet IDs for assessed inputs, plus available unassessed render links.
pub fn evidence(
    visual: &Visual,
    renders: &[Image],
    references: &[Image],
    anchors: &[Image],
) -> Result<Vec<Evidence>, String> {
    let mut result = Vec::new();
    if let Some(packet) = &visual.joint {
        for input in &packet.inputs {
            let source = match input.role.as_str() {
                "render" => renders,
                "reference" => references,
                "anchor" => anchors,
                _ => return Err("unknown image role".into()),
            };
            let image = source
                .iter()
                .find(|i| i.sha256 == input.sha256 && i.view == input.view && i.seed == input.seed)
                .ok_or("priority image link unavailable")?;
            result.push(Evidence {
                id: input.id.clone(),
                role: input.role.clone(),
                image: image.clone(),
            });
        }
    } else {
        for (role, images) in [("render", renders), ("reference", references)] {
            for (i, image) in images.iter().enumerate() {
                result.push(Evidence {
                    id: format!("{role}-{i}"),
                    role: role.into(),
                    image: image.clone(),
                });
            }
        }
    }
    for (i, image) in renders.iter().enumerate() {
        if !result
            .iter()
            .any(|e| e.image.sha256 == image.sha256 && e.role == "render")
        {
            result.push(Evidence {
                id: format!("render-unassessed-{i}"),
                role: "render".into(),
                image: image.clone(),
            });
        }
    }
    Ok(result)
}
