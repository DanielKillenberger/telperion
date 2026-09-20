//! Hash-bound joint evidence. Relationships describe provenance, not inferred biology.
use super::vision::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const BLIND_CHECKLIST: &str = "Review all supplied views jointly for believable reference character at the catalogue finish floor. Rank the largest grounded gaps; distinguish visible observations, uncertain causal hypotheses, acceptable variation and optional refinement. Compare cross-view constraints without assuming photographs show the same specimen or a causal change. Assess only supported required conclusions; missing, clipped or ambiguous evidence means unknown. No photographic pixel matching or photorealism requirement.";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub id: String,
    pub role: String,
    pub sha256: String,
    pub view: String,
    pub seed: u32,
    pub render_identity: Option<String>,
    pub condition: String,
    pub framing: Framing,
    pub visibility: String,
    pub geometry_group: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Framing {
    Unknown,
    Complete,
    Clipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationSource {
    pub path: std::path::PathBuf,
    pub sha256: String,
    pub excerpt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Packet {
    pub inputs: Vec<Input>,
    pub reference_relation: String,
    pub relation_source: Option<RelationSource>,
    pub shot_source: Option<RelationSource>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Impact {
    Supported,
    Blocker,
    RequiredUnknown,
    Variation,
    Optional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub observation: String,
    pub evidence_ids: Vec<String>,
    pub impact: Impact,
    pub uncertain: bool,
    pub causal_hypothesis: Option<String>,
}

impl Packet {
    pub fn from_request(r: &Request) -> Self {
        let mut inputs = Vec::new();
        for (role, images) in [
            ("render", r.images.clone()),
            ("reference", r.references.clone()),
            (
                "anchor",
                r.quality_anchors.iter().map(|a| a.image.clone()).collect(),
            ),
        ] {
            for (i, image) in images.iter().enumerate() {
                inputs.push(Input {
                    id: format!("{role}-{i}"),
                    role: role.into(),
                    sha256: image.sha256.clone(),
                    view: image.view.clone(),
                    seed: image.seed,
                    render_identity: (role == "render").then(|| r.identity.clone()),
                    condition: if role == "render" {
                        "same_geometry_visibility_view_not_unloaded_leaf_off"
                    } else {
                        "unknown"
                    }
                    .into(),
                    framing: Framing::Unknown,
                    visibility: "unknown".into(),
                    geometry_group: (role == "render")
                        .then(|| format!("{}:seed:{}", r.identity, image.seed)),
                });
            }
        }
        Self {
            inputs,
            reference_relation: "unknown".into(),
            relation_source: None,
            shot_source: None,
        }
    }
    pub fn with_shots(mut self, path: &std::path::Path) -> Result<Self, String> {
        let bytes = source_bytes(path)?;
        self.shot_source = Some(RelationSource {
            path: path.into(),
            sha256: crate::sha256_hex(&bytes),
            excerpt: "\"references\"".into(),
        });
        let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        for input in self.inputs.iter_mut().filter(|i| i.role == "render") {
            let rows = value["references"]
                .as_array()
                .ok_or("missing shot metadata")?;
            let row = rows
                .iter()
                .find(|r| r["id"] == input.view)
                .ok_or("missing view metadata")?;
            input.visibility = match row["shot"]["foliage"].as_str() {
                Some("hidden") => "hidden",
                Some("leaf-on") => "shown",
                None if row["shot"]["foliage"].is_null() => "unknown",
                _ => return Err("invalid foliage visibility metadata".into()),
            }
            .into();
        }
        Ok(self)
    }
    pub fn verify(&self, r: &Request) -> Result<(), String> {
        let mut expected = Self::from_request(r);
        if let Some(source) = &self.shot_source {
            expected = expected.with_shots(&source.path)?;
            if expected.shot_source.as_ref().unwrap().sha256 != source.sha256
                || source.excerpt != "\"references\""
            {
                return Err("changed shot metadata".into());
            }
        }
        if self.inputs.len() != expected.inputs.len() || self.inputs.len() > 12 {
            return Err("incomplete joint inputs".into());
        }
        let mut ids = HashSet::new();
        for (got, want) in self.inputs.iter().zip(&expected.inputs) {
            if !ids.insert(&got.id)
                || got.id != want.id
                || got.role != want.role
                || got.sha256 != want.sha256
                || got.view != want.view
                || got.seed != want.seed
                || got.render_identity != want.render_identity
                || got.condition != want.condition
                || got.visibility != want.visibility
                || got.geometry_group != want.geometry_group
            {
                return Err("joint input provenance mismatch".into());
            }
        }
        match (&*self.reference_relation, &self.relation_source) {
            ("unknown", None) => {}
            ("same_specimen" | "different_specimens", Some(source)) => {
                let meta = std::fs::metadata(&source.path).map_err(|e| e.to_string())?;
                if !meta.is_file() || meta.len() > 1024 * 1024 || source.excerpt.trim().is_empty() {
                    return Err("invalid relation source".into());
                }
                let bytes = source_bytes(&source.path)?;
                if crate::sha256_hex(&bytes) != source.sha256
                    || !String::from_utf8_lossy(&bytes).contains(&source.excerpt)
                {
                    return Err("unverified reference relationship".into());
                }
            }
            _ => return Err("reference relationship requires authored source".into()),
        }
        Ok(())
    }
    pub fn verify_findings(&self, findings: &[Finding]) -> Result<(), String> {
        if findings.len() > 16 {
            return Err("too many joint findings".into());
        }
        for f in findings {
            let unique = f.evidence_ids.iter().collect::<HashSet<_>>();
            if f.observation.trim().is_empty()
                || f.observation.len() > 2048
                || f.evidence_ids.is_empty()
                || f.evidence_ids.len() > 12
                || unique.len() != f.evidence_ids.len()
                || f.evidence_ids
                    .iter()
                    .any(|id| !self.inputs.iter().any(|i| &i.id == id))
                || f.causal_hypothesis.as_ref().is_some_and(|s| s.len() > 2048)
            {
                return Err("invalid joint finding evidence".into());
            }
        }
        Ok(())
    }
}

fn source_bytes(path: &std::path::Path) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let meta = std::fs::metadata(path).map_err(|e| e.to_string())?;
    if !meta.is_file() || meta.len() > 1024 * 1024 {
        return Err("source must be bounded regular file".into());
    }
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("source is not regular file".into());
    }
    let mut bytes = Vec::new();
    file.take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > 1024 * 1024 {
        return Err("source exceeds bound".into());
    }
    Ok(bytes)
}
