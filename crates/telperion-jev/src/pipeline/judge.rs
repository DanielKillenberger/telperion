//! One Jev evaluation from a stage. The caller writes the ledger entry and
//! derives its identity from the state checksum, the questions and the model
//! name, so identical requests share one identity across runs and drivers;
//! the stage records that identity and the entry keeps the probabilities.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::ledger::{LedgerEntry, SourceRef};

use super::canon::{read_json, write_canonical, CanonError};

/// The caller a stage holds: transport, key and the run's ledger directory.
pub struct Judge<'a> {
    pub transport: &'a dyn Transport,
    pub key: &'a str,
    pub ledger_dir: PathBuf,
}

pub struct Judgment {
    pub entry: LedgerEntry,
    /// The entry's derived identity, the reference artifacts record.
    pub reference: String,
}

impl Judge<'_> {
    pub fn ask(
        &self,
        tool: &str,
        source: Option<&SourceRef>,
        state: &Value,
        questions: &Value,
    ) -> Result<Judgment, CallerError> {
        let entry = evaluate(
            self.transport,
            self.key,
            EvaluateRequest {
                tool,
                source,
                state,
                questions,
                ledger_dir: &self.ledger_dir,
            },
        )?;
        let reference = entry.identity.clone();
        record_reference(&self.ledger_dir, &entry)
            .map_err(|err| CallerError::Transport(err.to_string()))?;
        Ok(Judgment { entry, reference })
    }
}

/// `ledger/index.json`: identity -> the entry's own id and tool, so a report
/// resolves a reference without scanning the entry files.
pub fn record_reference(ledger_dir: &Path, entry: &LedgerEntry) -> Result<(), CanonError> {
    let path = ledger_dir.join("index.json");
    let mut index: BTreeMap<String, Value> = if path.exists() {
        serde_json::from_value(read_json(&path)?["references"].clone()).unwrap_or_default()
    } else {
        BTreeMap::new()
    };
    index.entry(entry.identity.clone()).or_insert_with(
        || json!({"tool": entry.tool, "entry": entry.id, "state_sha256": entry.state_sha256}),
    );
    write_canonical(
        &path,
        &json!({"schema": "ledger-index", "schema_version": 1, "references": index}),
    )
    .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::derived_identity;

    #[test]
    fn the_identity_depends_on_state_questions_and_model_only() {
        let q = json!({"kind": {"type": "choice"}});
        let base = derived_identity("abc", &q, "jev-latest");
        assert_eq!(base.len(), 24);
        assert_eq!(base, derived_identity("abc", &q, "jev-latest"));
        assert_ne!(base, derived_identity("abd", &q, "jev-latest"));
        assert_ne!(
            base,
            derived_identity("abc", &json!({"kind": 1}), "jev-latest")
        );
        assert_ne!(base, derived_identity("abc", &q, "jev-2026"));
    }

    #[test]
    fn the_index_keeps_the_first_entry_per_identity() {
        let dir = std::env::temp_dir().join(format!(
            "jev-judge-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        let mut entry = LedgerEntry {
            id: "one".into(),
            tool: "screen".into(),
            state_sha256: "abc".into(),
            source: None,
            model: "jev-latest".into(),
            questions: json!({}),
            answers: json!({}),
            usage: None,
            elapsed_ms: 0,
            recorded_at: "2026-09-18T00:00:00Z".into(),
            error: None,
            identity: "same".into(),
        };
        record_reference(&dir, &entry).unwrap();
        entry.id = "two".into();
        record_reference(&dir, &entry).unwrap();
        let index = read_json(&dir.join("index.json")).unwrap();
        assert_eq!(index["references"]["same"]["entry"], "one");
    }
}
