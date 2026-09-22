use serde_json::json;
use telperion_jev::{
    ledger::{derived_identity, LedgerEntry, Usage},
    tuning::continuation::{ExternalLedger, ExternalUsage},
};
#[test]
fn pinned_external_usage_is_additive_and_cannot_be_replayed_or_forged() {
    let path = std::env::temp_dir().join(telperion_jev::ledger::new_entry_id());
    let mut e = LedgerEntry {
        id: "external-id".into(),
        tool: "study".into(),
        model: "jev-1".into(),
        state_sha256: "a".repeat(64),
        questions: json!({"q":{"type":"noul"}}),
        answers: json!({"q":{"type":"noul","noul":0.9}}),
        usage: Some(Usage {
            input_tokens: 7,
            output_tokens: 3,
        }),
        source: None,
        elapsed_ms: 1,
        recorded_at: "now".into(),
        error: None,
        identity: String::new(),
    };
    e.identity = derived_identity(&e.state_sha256, &e.questions, &e.model);
    let bytes = serde_json::to_vec(&e).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    let pin = ExternalLedger {
        path: path.clone(),
        sha256: telperion_jev::sha256_hex(&bytes),
        id: e.id.clone(),
        tool: e.tool.clone(),
        model: e.model.clone(),
        identity: e.identity.clone(),
    };
    let good = ExternalUsage {
        previous_tokens: 20,
        next_tokens: 30,
        reason: "scoped external study".into(),
        ledgers: vec![pin.clone()],
    };
    assert_eq!(good.verify(20, 30, &Default::default()).unwrap(), 30);
    for variant in [
        "previous",
        "next",
        "cap",
        "duplicate",
        "replay",
        "hash",
        "missing",
        "unknown",
        "overflow",
        "identity",
        "answers",
        "native",
    ] {
        std::fs::write(&path, &bytes).unwrap();
        let mut u = good.clone();
        let mut old = std::collections::HashSet::new();
        let mut cap = 30;
        match variant {
            "previous" => u.previous_tokens = 19,
            "next" => u.next_tokens = 29,
            "cap" => cap = 29,
            "duplicate" => u.ledgers.push(pin.clone()),
            "replay" => {
                old.insert(pin.id.clone());
            }
            "hash" => u.ledgers[0].sha256 = "b".repeat(64),
            "missing" => u.ledgers[0].path = path.with_extension("absent"),
            "native" => u.ledgers[0].tool = "tuning".into(),
            _ => {
                let mut changed = e.clone();
                match variant {
                    "unknown" => changed.usage = None,
                    "overflow" => {
                        changed.usage = Some(Usage {
                            input_tokens: u64::MAX,
                            output_tokens: 1,
                        })
                    }
                    "identity" => changed.identity = "wrong".into(),
                    _ => changed.answers = json!({}),
                };
                let b = serde_json::to_vec(&changed).unwrap();
                std::fs::write(&path, &b).unwrap();
                u.ledgers[0].sha256 = telperion_jev::sha256_hex(&b);
            }
        }
        assert!(u.verify(20, cap, &old).is_err(), "{variant}");
    }
    std::fs::remove_file(path).unwrap();
}
