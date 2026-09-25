//! fn-151's deterministic guards, which block: the production boundary
//! (R1), entry coverage's triggers and roles (R2), and the artifact budgets
//! (R3). No Jev call is made here.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use telperion_jev::principles::boundary::{added, scan, Caller, Scan};
use telperion_jev::principles::budget::{budgets, change_defects, check};
use telperion_jev::principles::policy::{exceptions, policy, Exception};
use telperion_jev::principles::push::triggered;
use telperion_jev::principles::source::{wanted, Snapshot};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn workspace() -> Snapshot {
    Snapshot::from_dir(&root(), wanted).expect("workspace snapshot")
}

/// The workspace with `path` added and `mod <name>;` appended to the wasm
/// binding's crate root.
fn with_scratch(path: &str, name: &str, text: &str) -> Snapshot {
    let mut snap = workspace();
    snap.files.insert(path.into(), text.into());
    let lib = "crates/telperion-wasm/src/lib.rs";
    let root = snap.files[lib].clone();
    snap.files.insert(lib.into(), format!("{root}\nmod {name};\n"));
    snap
}

fn run(snap: &Snapshot) -> Scan {
    scan(snap, &policy().boundary, &exceptions())
}

#[test]
fn the_workspace_calls_no_build_stage_outside_the_pipeline() {
    let scan = run(&workspace());
    assert!(scan.passes(), "{:?} {:?} {:?}", scan.violations, scan.unresolved, scan.parse_errors);
    let ids: Vec<&str> = scan.excepted.iter().map(|(_, id)| id.as_str()).collect();
    assert!(ids.contains(&"EX-17") && ids.contains(&"EX-50"), "{ids:?}");
}

#[test]
fn a_scratch_production_caller_fails_and_test_code_passes() {
    let text = "use telperion_core::branching as grow;\n\
        pub fn chain(f: &telperion_core::Family) {\n    let _ = grow::generate(&f.skeleton, f.radii);\n}\n\
        #[cfg(test)]\nmod t {\n    fn check(f: &telperion_core::Family) { let _ = telperion_core::surface::build; let _ = f; }\n}\n";
    let mut snap = with_scratch("crates/telperion-wasm/src/scratch.rs", "scratch", text);
    snap.files.insert(
        "crates/telperion-wasm/tests/scratch.rs".into(),
        "fn t() { let _ = telperion_core::branching::generate; }".into(),
    );
    let scan = run(&snap);
    let found: Vec<(&str, usize, &str)> = scan
        .violations
        .iter()
        .map(|c| (c.file.as_str(), c.line, c.stage.as_str()))
        .collect();
    assert_eq!(
        found,
        vec![("crates/telperion-wasm/src/scratch.rs", 3, "telperion_core::branching::generate")]
    );
}

#[test]
fn an_unresolvable_alias_fails_with_its_location() {
    let cases = [
        ("use telperion_core::branching::*;\npub fn f() {}\n", 1, "glob import of telperion_core::branching"),
        ("pub type Leaves = telperion_core::field::Field;\n", 1, "type alias Leaves"),
    ];
    for (text, line, what) in cases {
        let scan = run(&with_scratch("crates/telperion-wasm/src/scratch.rs", "scratch", text));
        assert!(!scan.passes(), "{text}");
        let u = &scan.unresolved[0];
        assert_eq!((u.file.as_str(), u.line), ("crates/telperion-wasm/src/scratch.rs", line));
        assert!(u.what.starts_with(what), "{}", u.what);
    }
}

/// #82's shape: the binding's copied callers go and the pipeline's come in,
/// so nothing is added; a caller that only moves within its crate is not an
/// addition either, and a new one in another crate is.
#[test]
fn removed_and_added_callers_are_compared_together() {
    let at = |file: &str, symbol: &str, stage: &str| Caller {
        file: file.into(),
        line: 1,
        symbol: symbol.into(),
        stage: stage.into(),
    };
    let gen = "telperion_core::branching::generate";
    let base = Scan {
        violations: vec![at("crates/telperion-wasm/src/generate.rs", "telperion_wasm::generate::generate", gen)],
        ..Scan::default()
    };
    assert!(added(&base, &Scan::default()).is_empty(), "removal");
    let moved = Scan {
        violations: vec![at("crates/telperion-wasm/src/build.rs", "telperion_wasm::build::run", gen)],
        ..Scan::default()
    };
    assert!(added(&base, &moved).is_empty(), "a move within the crate");
    let copied = Scan {
        violations: vec![base.violations[0].clone(), at("crates/telperion-field/src/grow.rs", "telperion_field::grow::field", gen)],
        ..Scan::default()
    };
    let new = added(&base, &copied);
    assert_eq!(new.len(), 1);
    assert_eq!(new[0].file, "crates/telperion-field/src/grow.rs");
}

#[test]
fn exceptions_are_scoped_sourced_and_live() {
    let registry = exceptions();
    let ids: Vec<&str> = registry.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(ids, ["EX-17", "EX-50"]);
    for e in &registry {
        assert_eq!(e.defect(), None);
    }
    let scan = run(&workspace());
    for e in &registry {
        assert!(scan.excepted.iter().any(|(_, id)| *id == e.id), "{} covers no live caller", e.id);
    }
    let wide = Exception {
        callers: vec!["crates/telperion-render/".into()],
        ..registry[0].clone()
    };
    assert!(wide.defect().is_some_and(|d| d.contains("not a directory")));
    let claimed = Exception { source_kind: "approved in chat".into(), ..registry[0].clone() };
    assert!(claimed.defect().is_some_and(|d| d.contains("owner decision")));
}

#[test]
fn every_package_export_has_a_role_and_each_generation_entry_a_guard() {
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root().join("package.json")).unwrap()).unwrap();
    let entries = policy().entries;
    for export in manifest["exports"].as_object().unwrap().keys() {
        let entry = entries.iter().find(|e| &e.export == export);
        let entry = entry.unwrap_or_else(|| panic!("package export {export} has no role"));
        assert!(entry.role != "generation" || entry.guard.is_some(), "{export}");
    }
    let generation: Vec<&str> =
        entries.iter().filter(|e| e.role == "generation").map(|e| e.export.as_str()).collect();
    assert_eq!(generation, ["native", ".", "./field"]);
}

#[test]
fn entry_coverage_triggers_on_values_membership_and_wiring() {
    let triggers = policy().entry_triggers;
    let fires = |path: &str| !triggered(&[path.to_string()], &triggers).is_empty();
    for path in [
        "crates/telperion-core/src/presets/species.rs",
        "crates/telperion-core/src/presets.rs",
        "crates/telperion-field/src/grow.rs",
        "crates/telperion-wasm/src/generate.rs",
        "crates/telperion-core/src/pipeline/drawn.rs",
        "scripts/build-wasm.mjs",
    ] {
        assert!(fires(path), "{path}");
    }
    for path in ["docs/principles.md", "crates/telperion-render/src/scene/frame.rs", "harness/stage.ts"] {
        assert!(!fires(path), "{path}");
    }
}

fn shipped_sizes() -> BTreeMap<String, u64> {
    budgets()
        .artifacts
        .iter()
        .map(|b| (b.path.clone(), b.evidence.measured_bytes))
        .collect()
}

#[test]
fn the_rejected_slim_build_fails_its_budget_and_the_shipped_one_passes() {
    let b = budgets();
    assert!(check(&b, &shipped_sizes()).is_empty(), "0.1.4 as shipped");
    let mut first_attempt = shipped_sizes();
    first_attempt.insert("dist/telperion-field.wasm".into(), 526_965);
    let fails = check(&b, &first_attempt);
    assert_eq!(fails.len(), 1, "{fails:?}");
    assert!(fails[0].starts_with("dist/telperion-field.wasm: 526965 bytes, budget 362000 (+52.8%"), "{}", fails[0]);
    let mut missing = shipped_sizes();
    missing.remove("dist/field.js");
    assert!(check(&b, &missing)[0].starts_with("dist/field.js: not built"));
}

#[test]
fn a_budget_moved_without_new_evidence_fails() {
    let base = budgets();
    assert!(change_defects(Some(&base), &base).is_empty());
    let mut loosened = base.clone();
    loosened.artifacts[2].max_bytes = 540_000;
    let defects = change_defects(Some(&base), &loosened);
    assert_eq!(defects.len(), 1, "{defects:?}");
    assert!(defects[0].contains("without a new measurement"), "{}", defects[0]);
    let mut measured = loosened.clone();
    measured.artifacts[2].evidence.measured_bytes = 526_965;
    measured.artifacts[2].evidence.measured_on = "fn-150 first attempt".into();
    measured.artifacts[2].evidence.decisions = "PR #999 Decisions: D1".into();
    assert!(change_defects(Some(&base), &measured).is_empty());
}
