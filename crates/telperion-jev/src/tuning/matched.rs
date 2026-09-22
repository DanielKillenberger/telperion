//! Native matched captures and the existing compare script; no model in this path.
use super::evaluation::{Comparison, Image, MatchedRenderer, METRICS};
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::PathBuf, process::Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Matched {
    pub headless: PathBuf,
    pub compare_script: PathBuf,
    pub references: PathBuf,
    pub refs: PathBuf,
    pub catalogue: PathBuf,
    pub scratch: PathBuf,
    pub height: u32,
    pub numeric_references: Vec<String>,
    #[serde(default)]
    pub weights: BTreeMap<String, [f64; 5]>,
    #[serde(default)]
    pub reference_weights: BTreeMap<String, f64>,
}

pub fn run(command: &mut Command) -> Result<(), String> {
    let output = command.output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "{}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

pub fn read(path: &std::path::Path) -> Result<Value, String> {
    serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

pub fn image(path: PathBuf, view: String, seed: u32) -> Result<Image, String> {
    let sha256 = sha256_hex(&fs::read(&path).map_err(|e| e.to_string())?);
    Ok(Image {
        path,
        sha256,
        view,
        seed,
    })
}

impl Matched {
    pub fn records(&self) -> Result<Vec<Value>, String> {
        let root = read(&self.references)?;
        let records = root["references"]
            .as_array()
            .ok_or("missing references")?
            .iter()
            .filter(|r| r.get("shot").is_some())
            .cloned()
            .collect::<Vec<_>>();
        if records.is_empty() || self.height == 0 {
            return Err("no matched shots or height".into());
        }
        for record in &records {
            let id = record["id"].as_str().ok_or("missing view id")?;
            if id.is_empty()
                || !id
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                return Err("invalid reference id".into());
            }
        }
        if self.numeric_references.is_empty()
            || self
                .numeric_references
                .iter()
                .any(|id| !records.iter().any(|r| r["id"].as_str() == Some(id)))
        {
            return Err("missing numeric reference".into());
        }
        Ok(records)
    }
}

impl Matched {
    pub fn capture(
        &self,
        preset: &str,
        seed: u32,
        family: &Value,
        key: &str,
        compare: bool,
    ) -> Result<Vec<Comparison>, String> {
        let records = self.records()?;
        let records = records
            .into_iter()
            .filter(|r| {
                self.numeric_references
                    .iter()
                    .any(|id| r["id"].as_str() == Some(id))
            })
            .collect::<Vec<_>>();
        // A fresh directory makes partial outputs impossible to replay as new results.
        let dir = self
            .scratch
            .join(format!("{}-{}", key, crate::ledger::new_entry_id()));
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let family_path = dir.join("family.json");
        fs::write(&family_path, serde_json::to_vec(family).unwrap()).map_err(|e| e.to_string())?;
        let selected = dir.join("references.json");
        let mut reference_file = read(&self.references)?;
        reference_file["references"] = json!(records);
        fs::write(&selected, serde_json::to_vec(&reference_file).unwrap())
            .map_err(|e| e.to_string())?;
        let mut comparisons = vec![];
        for record in &records {
            let id = record["id"].as_str().unwrap();
            let shot = &record["shot"];
            let number = |v: &Value| {
                v.as_f64()
                    .filter(|v| v.is_finite())
                    .ok_or("invalid shot number")
            };
            let ratio = number(&shot["aspect"][0])? / number(&shot["aspect"][1])?;
            if !ratio.is_finite() || ratio <= 0. || ratio > 10. {
                return Err("invalid aspect".into());
            }
            let size = format!(
                "{}x{}",
                (self.height as f64 * ratio).round() as u32,
                self.height
            );
            let view = if shot["foliage"] == "hidden" {
                "bare"
            } else {
                "whole"
            };
            let overcast = number(&shot["light"]["overcast"])?;
            let dim = 1. - 0.8 * overcast;
            let mut images = vec![];
            for twin in [false, true] {
                let path = dir.join(format!("{key}-{id}{}.png", if twin { "-twin" } else { "" }));
                let scene = json!({
                    "sunAzimuth":if twin {(number(&shot["camera"]["azimuth"])?+180.)%360.}
                        else {number(&shot["light"]["sunAzimuth"])?},
                    "sunElevation":if twin {5.} else {number(&shot["light"]["sunElevation"])?},
                    "sunRed":3.*dim,"sunGreen":2.85*dim,"sunBlue":2.6*dim,
                    "skyZenithRed":0.18+0.37*overcast,"skyZenithGreen":0.30+0.36*overcast,
                    "skyZenithBlue":0.62+0.18*overcast});
                run(Command::new("timeout")
                    .arg("300")
                    .arg(&self.headless)
                    .args([
                        "--preset",
                        preset,
                        "--seed",
                        &seed.to_string(),
                        "--view",
                        view,
                        "--size",
                        &size,
                    ])
                    .arg("--out")
                    .arg(&path)
                    .arg("--family")
                    .arg(&family_path)
                    .args([
                        "--camera",
                        &shot["camera"].to_string(),
                        "--scene",
                        &scene.to_string(),
                        "--no-figure",
                    ]))?;
                images.push(image(path, id.into(), seed)?);
            }
            comparisons.push(Comparison {
                reference: id.into(),
                reference_weight: *self.reference_weights.get(id).unwrap_or(&1.),
                metric_weights: *self.weights.get(id).unwrap_or(&[1.; 5]),
                target: [0.; 5],
                observed: [None; 5],
                images,
            });
        }
        if !compare {
            return Ok(comparisons);
        }
        run(Command::new("timeout")
            .args(["300", "uv", "run"])
            .arg(&self.compare_script)
            .arg("--references")
            .arg(&selected)
            .arg("--captures")
            .arg(&dir)
            .arg("--refs")
            .arg(&self.refs)
            .arg("--catalogue")
            .arg(&self.catalogue)
            .args(["--case", key])
            .arg("--out")
            .arg(&dir))?;
        for c in &mut comparisons {
            let result = read(&dir.join(format!("{key}-{}-compare.json", c.reference)))?;
            for (i, name) in METRICS.iter().enumerate() {
                let field = |side: &str| {
                    if *name == "centre" {
                        &result[side][name]["mean"]
                    } else {
                        &result[side][name]
                    }
                };
                c.target[i] = field("photograph")
                    .as_f64()
                    .ok_or("unreadable reference number")?;
                c.observed[i] = field("still").as_f64();
            }
        }
        Ok(comparisons)
    }
}

impl MatchedRenderer for Matched {
    fn render(
        &self,
        preset: &str,
        seed: u32,
        family: &Value,
        key: &str,
    ) -> Result<Vec<Comparison>, String> {
        self.capture(preset, seed, family, key, true)
    }
}
