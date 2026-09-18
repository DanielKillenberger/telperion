//! The model-swap comparison: the canonical artifacts of two runs are
//! byte-identical, every difference is reported per artifact and per JSON
//! Pointer, the ledger is excluded, and each command log holds only runbook
//! commands.

use std::fs;
use std::path::Path;

use serde::Serialize;
use serde_json::Value;

use super::stage::{logged_stages, Paths, STAGES};

/// The artifacts the comparison covers, relative to the pipeline directory.
pub const COMPARED: [&str; 7] = [
    "packet/profile.json",
    "packet/references.json",
    "packet/species.json",
    "packet/specimens.json",
    "provenance.json",
    "decisions.json",
    "report.json",
];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Difference {
    pub artifact: String,
    pub pointer: String,
    pub left: Option<Value>,
    pub right: Option<Value>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Comparison {
    pub identical: Vec<String>,
    pub differences: Vec<Difference>,
    pub commands_outside_runbook: Vec<(String, String)>,
}

impl Comparison {
    pub fn passed(&self) -> bool {
        self.differences.is_empty() && self.commands_outside_runbook.is_empty()
    }
}

/// Compares the covered artifacts of two runs and checks both command logs.
pub fn compare(left: &Path, right: &Path) -> Comparison {
    let mut identical = Vec::new();
    let mut differences = Vec::new();
    for artifact in COMPARED {
        let a = fs::read(left.join(artifact)).ok();
        let b = fs::read(right.join(artifact)).ok();
        if a.is_some() && a == b {
            identical.push(artifact.to_string());
            continue;
        }
        let parse =
            |bytes: Option<Vec<u8>>| bytes.and_then(|b| serde_json::from_slice::<Value>(&b).ok());
        differences.extend(diff_values(
            artifact,
            "",
            parse(a).as_ref(),
            parse(b).as_ref(),
        ));
    }
    let mut commands_outside_runbook = Vec::new();
    for (side, dir) in [("left", left), ("right", right)] {
        for stage in logged_stages(&Paths::new(dir)).unwrap_or_default() {
            if !STAGES.contains(&stage.as_str()) {
                commands_outside_runbook.push((side.to_string(), stage));
            }
        }
    }
    Comparison {
        identical,
        differences,
        commands_outside_runbook,
    }
}

/// Every leaf where the two values differ, by JSON Pointer.
pub fn diff_values(
    artifact: &str,
    pointer: &str,
    left: Option<&Value>,
    right: Option<&Value>,
) -> Vec<Difference> {
    match (left, right) {
        (Some(Value::Object(a)), Some(Value::Object(b))) => {
            let keys: std::collections::BTreeSet<&String> = a.keys().chain(b.keys()).collect();
            keys.into_iter()
                .flat_map(|key| {
                    diff_values(
                        artifact,
                        &format!("{pointer}/{}", escape(key)),
                        a.get(key),
                        b.get(key),
                    )
                })
                .collect()
        }
        (Some(Value::Array(a)), Some(Value::Array(b))) => (0..a.len().max(b.len()))
            .flat_map(|i| diff_values(artifact, &format!("{pointer}/{i}"), a.get(i), b.get(i)))
            .collect(),
        (l, r) if l == r => Vec::new(),
        (l, r) => vec![Difference {
            artifact: artifact.to_string(),
            pointer: if pointer.is_empty() {
                "/".into()
            } else {
                pointer.to_string()
            },
            left: l.cloned(),
            right: r.cloned(),
        }],
    }
}

fn escape(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

pub fn format_comparison(c: &Comparison) -> String {
    let mut out = String::new();
    for artifact in &c.identical {
        out.push_str(&format!("identical\t{artifact}\n"));
    }
    for d in &c.differences {
        out.push_str(&format!(
            "differs\t{}\t{}\t{}\t{}\n",
            d.artifact,
            d.pointer,
            d.left
                .as_ref()
                .map(Value::to_string)
                .unwrap_or_else(|| "<absent>".into()),
            d.right
                .as_ref()
                .map(Value::to_string)
                .unwrap_or_else(|| "<absent>".into())
        ));
    }
    for (side, command) in &c.commands_outside_runbook {
        out.push_str(&format!("outside runbook\t{side}\t{command}\n"));
    }
    out.push_str(if c.passed() {
        "swap: PASS\n"
    } else {
        "swap: FAIL\n"
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::canon::write_canonical;
    use crate::pipeline::stage::log_command;
    use serde_json::json;
    use std::path::PathBuf;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jev-swap-{tag}-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn identical_artifacts_pass_and_a_changed_leaf_is_named_by_pointer() {
        let (a, b) = (scratch("a"), scratch("b"));
        for dir in [&a, &b] {
            write_canonical(
                &dir.join("provenance.json"),
                &json!({"entries": {"/x": {"span": "6 m"}}}),
            )
            .unwrap();
            write_canonical(
                &dir.join("decisions.json"),
                &json!({"decisions": [{"id": "one"}]}),
            )
            .unwrap();
            log_command(&Paths::new(dir), &["fetch".into()], 0).unwrap();
        }
        assert!(compare(&a, &b).passed());
        write_canonical(
            &b.join("decisions.json"),
            &json!({"decisions": [{"id": "two"}]}),
        )
        .unwrap();
        let c = compare(&a, &b);
        assert!(!c.passed());
        assert_eq!(c.differences.len(), 1);
        assert_eq!(c.differences[0].artifact, "decisions.json");
        assert_eq!(c.differences[0].pointer, "/decisions/0/id");
        assert_eq!(c.differences[0].left, Some(json!("one")));
    }

    #[test]
    fn a_command_outside_the_runbook_fails_the_trial_and_names_it() {
        let (a, b) = (scratch("c"), scratch("d"));
        log_command(&Paths::new(&a), &["cat".into(), "manifest.json".into()], 0).unwrap();
        let c = compare(&a, &b);
        assert_eq!(
            c.commands_outside_runbook,
            vec![("left".to_string(), "cat".to_string())]
        );
        assert!(format_comparison(&c).ends_with("swap: FAIL\n"));
    }

    #[test]
    fn an_absent_artifact_on_one_side_is_one_difference_at_the_root() {
        let (a, b) = (scratch("e"), scratch("f"));
        write_canonical(&a.join("report.json"), &json!({"status": "complete"})).unwrap();
        let c = compare(&a, &b);
        assert_eq!(c.differences.len(), 1);
        assert_eq!(c.differences[0].pointer, "/");
        assert_eq!(c.differences[0].right, None);
    }
}
