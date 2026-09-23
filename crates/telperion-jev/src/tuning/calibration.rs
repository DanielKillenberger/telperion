//! Frozen labels stay out of model state. Results are checked against immutable ledgers.
use crate::{
    caller::{evaluate, EvaluateRequest, Transport},
    ledger::LedgerEntry,
    sha256_hex,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub split: String,
    pub provenance: String,
    pub state: Value,
    pub questions: Value,
    pub expected: BTreeMap<String, String>,
    pub unsafe_answers: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: String,
    pub question_version: String,
    pub table_sha256: String,
    pub kind: String,
    pub model: String,
    pub min_accuracy: f64,
    pub min_confidence: f64,
    pub cases: Vec<Case>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultSet {
    pub manifest_sha256: String,
    pub ledgers: BTreeMap<String, PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub total: usize,
    pub correct: usize,
    pub unsafe_answers: usize,
    pub abstentions: usize,
    pub tokens: u64,
}

pub fn questions(kind: &str, state: &Value) -> Result<Value, String> {
    match kind {
        "magnitude" | "direction" => {
            let dial: super::actions::Dial = serde_json::from_value(state["dial"].clone())
                .map_err(|_| "case lacks authored dial")?;
            let current = state["current"]
                .as_f64()
                .ok_or("case lacks current value")?;
            Ok(
                serde_json::json!({"adjustment":if kind=="direction" {dial.direction_question(current)?} else {dial.question(current)?}}),
            )
        }
        "continuation" => Ok(super::continuation::questions()),
        "gap_magnitude" => Ok(super::stride::questions()),
        _ => Err("unknown calibration kind".into()),
    }
}

pub fn validate_manifest(m: &Manifest) -> Result<(), String> {
    if m.schema != "tuning-calibration-v1"
        || m.question_version.is_empty()
        || m.table_sha256.len() != 64
        || m.model.is_empty()
        || !m.min_accuracy.is_finite()
        || !(0.8..=1.).contains(&m.min_accuracy)
        || !m.min_confidence.is_finite()
        || !(0. ..=1.).contains(&m.min_confidence)
    {
        return Err("invalid calibration contract".into());
    }
    let mut ids = BTreeSet::new();
    for c in &m.cases {
        if !ids.insert(&c.id)
            || c.id.is_empty()
            || c.provenance.is_empty()
            || !["tuning", "heldout"].contains(&c.split.as_str())
            || c.expected.is_empty()
        {
            return Err("invalid calibration case".into());
        }
        let questions = questions(&m.kind, &c.state)?;
        if c.questions != questions {
            return Err("case questions differ from runtime question generator".into());
        }
        for (q, answer) in &c.expected {
            if c.questions[q]["criteria"].get(answer).is_none() {
                return Err("unsupported label".into());
            }
        }
    }
    if !m.cases.iter().any(|c| c.split == "heldout") || !m.cases.iter().any(|c| c.split == "tuning")
    {
        return Err("separate tuning and heldout cases required".into());
    }
    Ok(())
}

pub fn score(manifest_bytes: &[u8], result: &ResultSet) -> Result<Score, String> {
    if sha256_hex(manifest_bytes) != result.manifest_sha256 {
        return Err("changed calibration manifest".into());
    }
    let m: Manifest = serde_json::from_slice(manifest_bytes).map_err(|e| e.to_string())?;
    validate_manifest(&m)?;
    let mut score = Score {
        total: 0,
        correct: 0,
        unsafe_answers: 0,
        abstentions: 0,
        tokens: 0,
    };
    for case in &m.cases {
        let path = result
            .ledgers
            .get(&case.id)
            .ok_or("missing calibration case result")?;
        let entry: LedgerEntry =
            serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        if entry.error.is_some()
            || entry.model != m.model
            || entry.questions != case.questions
            || entry.state_sha256 != sha256_hex(&serde_json::to_vec(&case.state).unwrap())
        {
            return Err("stale calibration judgment".into());
        }
        let usage = entry.usage.as_ref().ok_or("unknown calibration usage")?;
        score.tokens = score
            .tokens
            .checked_add(usage.input_tokens)
            .and_then(|n| n.checked_add(usage.output_tokens))
            .ok_or("usage overflow")?;
        if case.split != "heldout" {
            continue;
        }
        for (q, expected) in &case.expected {
            score.total += 1;
            let chosen = entry.choice(q).ok_or("missing calibration answer")?;
            let confidence = entry
                .confidence(q)
                .ok_or("missing calibration confidence")?;
            if !confidence.is_finite() || !(0. ..=1.).contains(&confidence) {
                return Err("invalid confidence".into());
            }
            if chosen == "insufficient_evidence" || confidence < m.min_confidence {
                score.abstentions += 1;
            }
            if &chosen == expected && confidence >= m.min_confidence {
                score.correct += 1;
            }
            if case
                .unsafe_answers
                .get(q)
                .is_some_and(|v| v.contains(&chosen))
                || case.questions[q]["criteria"].get(&chosen).is_none()
            {
                score.unsafe_answers += 1;
            }
        }
    }
    Ok(score)
}

pub fn qualified(
    manifest_bytes: &[u8],
    result: &ResultSet,
    kind: &str,
    version: &str,
    table: &str,
) -> Result<Score, String> {
    let m: Manifest = serde_json::from_slice(manifest_bytes).map_err(|e| e.to_string())?;
    if m.kind != kind || m.question_version != version || m.table_sha256 != table {
        return Err("calibration does not cover current questions/table".into());
    }
    let s = score(manifest_bytes, result)?;
    let agreement = if kind == "continuation" {
        let policy = policy_score(&m, result)?;
        policy.total > 0
            && policy.correct as f64 / policy.total as f64 >= m.min_accuracy
            && policy.false_continue == 0
    } else {
        s.total > 0
            && s.correct as f64 / (s.total as f64) >= m.min_accuracy
            && s.unsafe_answers == 0
    };
    if !agreement {
        return Err("heldout validation failed".into());
    }
    Ok(s)
}

#[derive(Debug, Serialize)]
pub struct PolicyScore {
    pub version: &'static str,
    pub total: usize,
    pub correct: usize,
    pub false_continue: usize,
    pub false_pause: usize,
}

pub fn continues(answers: &BTreeMap<String, (String, f64)>, cut: f64) -> bool {
    [
        ("tractability", "supported"),
        ("progress", "supported"),
        ("risk", "bounded"),
    ]
    .iter()
    .all(|(q, expected)| {
        answers.get(*q).is_some_and(|(actual, confidence)| {
            actual == expected && confidence.is_finite() && *confidence >= cut && *confidence <= 1.
        })
    })
}

pub fn policy_score(m: &Manifest, result: &ResultSet) -> Result<PolicyScore, String> {
    let mut score = PolicyScore {
        version: "continuation-composed-v1",
        total: 0,
        correct: 0,
        false_continue: 0,
        false_pause: 0,
    };
    for c in m.cases.iter().filter(|c| c.split == "heldout") {
        let entry: LedgerEntry = serde_json::from_slice(
            &fs::read(result.ledgers.get(&c.id).ok_or("missing result")?)
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let expected = c
            .expected
            .iter()
            .map(|(k, v)| (k.clone(), (v.clone(), 1.)))
            .collect();
        let observed = c
            .expected
            .keys()
            .map(|k| {
                (
                    k.clone(),
                    (
                        entry.choice(k).unwrap_or_default(),
                        entry.confidence(k).unwrap_or(0.),
                    ),
                )
            })
            .collect();
        let wanted = continues(&expected, m.min_confidence);
        let got = continues(&observed, m.min_confidence);
        score.total += 1;
        score.correct += usize::from(wanted == got);
        score.false_continue += usize::from(!wanted && got);
        score.false_pause += usize::from(wanted && !got);
    }
    Ok(score)
}

pub fn run(
    manifest_bytes: &[u8],
    transport: &dyn Transport,
    key: &str,
    ledger: &Path,
    max_tokens: u64,
    output: &Path,
) -> Result<ResultSet, String> {
    let m: Manifest = serde_json::from_slice(manifest_bytes).map_err(|e| e.to_string())?;
    validate_manifest(&m)?;
    let mut result = ResultSet {
        manifest_sha256: sha256_hex(manifest_bytes),
        ledgers: BTreeMap::new(),
    };
    let mut journal = Journal::create(output, max_tokens)?;
    for case in &m.cases {
        // This conservative request bound is charged before dispatch. Unknown actual usage stops.
        let reserve = (serde_json::to_vec(&case.state).unwrap().len()
            + serde_json::to_vec(&case.questions).unwrap().len()) as u64
            + 4096;
        journal.reserve(reserve)?;
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "tuning-calibration",
                source: None,
                state: &case.state,
                questions: &case.questions,
                ledger_dir: ledger,
            },
        )
        .map_err(|e| e.to_string())?;
        let usage = entry
            .usage
            .as_ref()
            .ok_or("unknown calibration usage; stop")?;
        let tokens = usage
            .input_tokens
            .checked_add(usage.output_tokens)
            .ok_or("usage overflow")?;
        result.ledgers.insert(
            case.id.clone(),
            ledger.join(format!(
                "{}-{}-{}.json",
                entry.recorded_at.replace(':', ""),
                entry.tool,
                entry.id
            )),
        );
        journal.record(serde_json::to_value(&result).unwrap())?;
        journal.settle(tokens)?;
    }
    Ok(result)
}

/// A new invocation cannot overwrite a prior run's spend. Failed dispatch retains its reservation.
pub struct Journal {
    path: PathBuf,
    value: Value,
}

impl Journal {
    pub fn create(path: &Path, limit: u64) -> Result<Self, String> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        let value = serde_json::json!({"limit":limit,"spent":0,"pending":0,"result":null});
        file.write_all(&serde_json::to_vec(&value).unwrap())
            .map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        Ok(Self {
            path: path.into(),
            value,
        })
    }
    /// Loads an existing journal so a resumed replay can see which cases were
    /// already dispatched, or creates a fresh one.
    pub fn open(path: &Path, limit: u64) -> Result<Self, String> {
        if !path.exists() {
            return Self::create(path, limit);
        }
        let value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        Ok(Self {
            path: path.into(),
            value,
        })
    }
    /// What was recorded for this case, if it has already been dispatched.
    pub fn case(&self, id: &str) -> Option<&Value> {
        self.value.get("cases")?.get(id)
    }
    pub fn note(&mut self, id: &str, entry: Value) -> Result<(), String> {
        if !self.value["cases"].is_object() {
            self.value["cases"] = serde_json::json!({});
        }
        self.value["cases"][id] = entry;
        self.save()
    }
    fn save(&self) -> Result<(), String> {
        let temp = self.path.with_extension("pending");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)
            .map_err(|e| e.to_string())?;
        file.write_all(&serde_json::to_vec_pretty(&self.value).unwrap())
            .map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        fs::rename(&temp, &self.path).map_err(|e| e.to_string())
    }
    pub fn reserve(&mut self, tokens: u64) -> Result<(), String> {
        let spent = self.value["spent"].as_u64().unwrap();
        if self.value["pending"] != 0
            || spent
                .checked_add(tokens)
                .is_none_or(|n| n > self.value["limit"].as_u64().unwrap())
        {
            return Err("token allowance unavailable".into());
        }
        self.value["spent"] = serde_json::json!(spent + tokens);
        self.value["pending"] = serde_json::json!(tokens);
        self.save()
    }
    pub fn settle(&mut self, actual: u64) -> Result<(), String> {
        let pending = self.value["pending"].as_u64().unwrap();
        let spent = self.value["spent"].as_u64().unwrap() - pending;
        self.value["spent"] =
            serde_json::json!(spent.checked_add(actual).ok_or("usage overflow")?);
        self.value["pending"] = serde_json::json!(0);
        self.save()?;
        if actual > pending
            || self.value["spent"].as_u64().unwrap() > self.value["limit"].as_u64().unwrap()
        {
            return Err("provider exceeded token reservation; stop".into());
        }
        Ok(())
    }
    pub fn record(&mut self, result: Value) -> Result<(), String> {
        self.value["result"] = result;
        self.save()
    }
}
