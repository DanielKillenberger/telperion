//! Synthetic offline fixtures for the tuning command.
//!
//! Every artefact built here is fabricated so that the real command can be
//! driven past `Config::verify` without a paid call. None of it is evidence:
//! the judgments are invented, the receipts are not receipts, and nothing in
//! this module may be copied into `.flow/evidence/`.
#![allow(dead_code)]
use serde_json::{json, Value};
use std::{fs, path::PathBuf};
use telperion_jev::{
    sha256_hex,
    tuning::{
        actions::Dial,
        engine::Run,
        reference_first::{Inventory, Priority, ReferenceImage, ReferenceRequest, Trait, VERSION},
    },
};

pub const SYNTHETIC: &str = "SYNTHETIC OFFLINE TEST FIXTURE - invented, never evidence";
pub const JUDGMENT_MODEL: &str = "jev-fixture-1";
pub const VISION_MODEL: &str = "vision-fixture-1";
pub const EFFORT: &str = "medium";
pub const PREPARATION_TOKENS: u64 = 1000;
pub const PRESET: &str = "european-beech";

pub struct Fixture {
    pub root: PathBuf,
    pub config_path: PathBuf,
    pub out: PathBuf,
    pub config: Value,
    pub dials: Vec<Dial>,
}

impl Fixture {
    pub fn rewrite(&self, config: &Value) {
        fs::write(
            &self.config_path,
            serde_json::to_vec_pretty(config).unwrap(),
        )
        .unwrap();
    }
    pub fn run_json(&self) -> Value {
        serde_json::from_slice(&fs::read(self.out.join("run.json")).unwrap()).unwrap()
    }
    pub fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn write(path: &PathBuf, value: &Value) -> String {
    let bytes = serde_json::to_vec_pretty(value).unwrap();
    fs::write(path, &bytes).unwrap();
    sha256_hex(&bytes)
}

fn script(path: &PathBuf, body: &str) {
    fs::write(path, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

fn image(root: &std::path::Path, view: &str, seed: u32) -> Value {
    let path = root.join(format!("{view}-{seed}.png"));
    let bytes = format!("synthetic raster {view} {seed}").into_bytes();
    fs::write(&path, &bytes).unwrap();
    json!({"path":path,"sha256":sha256_hex(&bytes),"view":view,"seed":seed})
}

/// The adapter the runtime shells out to. It answers from the request alone,
/// so it needs no network and no key.
const ADAPTER: &str = r#"import json, sys
e = json.load(sys.stdin)
r = e["request"]
model = sys.argv[sys.argv.index("--model") + 1]
effort = sys.argv[sys.argv.index("--effort") + 1]
if e["stage"] == "inventory":
    print(json.dumps({"status": "ok", "request_sha256": e["request_sha256"],
        "prompt_sha256": e["prompt_sha256"], "model": model, "effort": effort,
        "usage": {"input_tokens": 700, "output_tokens": 300},
        "answer": {"traits": [{"id": "trait-core", "priority": "core",
            "observation": "synthetic core recognition trait",
            "reference_ids": [r["references"][0]["id"]], "uncertain": False}],
            "observations": ["synthetic fixture observation"]}}))
    raise SystemExit
if e["stage"] == "sheet":
    labels = [str(i + 1) for i in range(len(r["renders"]))]
    steps = [{"from": a, "to": b, "grade": "none"} for a, b in zip(labels, labels[1:])]
    print(json.dumps({"status": "ok", "request_sha256": e["request_sha256"],
        "prompt_sha256": e["prompt_sha256"], "model": model, "effort": effort,
        "usage": {"input_tokens": 90, "output_tokens": 30},
        "answer": {"priorities": [{"priority_id": p["id"], "closest": labels[0],
            "ranking": labels, "steps": steps} for p in r["priorities"]],
            "overall": labels, "wrong": [], "breaks": [],
            "improved": "synthetic: nothing moved", "missing": "synthetic"}}))
    raise SystemExit
c = r["comparison"]
inv = r["inventory"]
findings = [{"observation": "synthetic supported match",
    "evidence_ids": ["render-0", "reference-0"],
    "impact": "supported", "uncertain": False, "causal_hypothesis": None}]
# Only the production comparison carries the owner's requirements, so only it
# reports the blocking gap; a blind replay case is judged on its own evidence.
if r.get("production_requirements"):
    findings.append({"observation": "synthetic blocking gap in the crown",
        "evidence_ids": ["render-0", "reference-0"],
        "impact": "blocker", "uncertain": False,
        "causal_hypothesis": "unproven synthetic mechanism"})
cov = [{"trait_id": t["id"], "status": "pass",
        "evidence_ids": ["render-0", "reference-0"],
        "explanation": "synthetic fixture disposition"} for t in inv["traits"]]
print(json.dumps({"status": "ok", "request_sha256": e["request_sha256"],
    "prompt_sha256": e["prompt_sha256"], "model": model, "effort": effort,
    "usage": {"input_tokens": 120, "output_tokens": 40},
    "answer": {"passes": ["pass"] * len(c["required"]), "defects": [],
        "observations": ["synthetic fixture observation"],
        "findings": findings,
        "coverage": cov}}))
"#;

const MEASURE: &str = r#"#!/bin/sh
out=""
while [ $# -gt 0 ]; do
  case "$1" in --output) out="$2"; shift 2 ;; *) shift ;; esac
done
cat > "$out" <<'JSONL'
{"event":"started"}
{"event":"completed","numeric_status":"pass","checks":{"synthetic":{"status":"pass"}},"metrics":{"nodes":{"value":1000},"growth":{"node_capped":false,"level_capped":false,"attraction_capped":false}}}
JSONL
"#;

const HEADLESS: &str = r#"#!/bin/sh
out=""
while [ $# -gt 0 ]; do
  case "$1" in --out) out="$2"; shift 2 ;; *) shift ;; esac
done
printf 'synthetic still %s' "$out" > "$out"
"#;

const COMPARE: &str = r#"import json, os, sys
a = sys.argv
captures = a[a.index("--captures") + 1]
case = a[a.index("--case") + 1]
out = a[a.index("--out") + 1]
refs = json.load(open(a[a.index("--references") + 1]))["references"]
row = {"width_over_height": 1.0, "crown_base": 1.0, "occupied": 1.0,
       "outline_deviation": 1.0, "centre": {"mean": 1.0}}
for r in refs:
    json.dump({"photograph": row, "still": row},
              open(os.path.join(out, "%s-%s-compare.json" % (case, r["id"])), "w"))
"#;

/// A configuration whose `verify()` passes offline, with reference-first
/// enabled so preparation charging is exercised.
pub fn verifying_fixture(budget: Value) -> Fixture {
    let root = std::env::temp_dir().join(format!(
        "tuning-fixture-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&root).unwrap();
    let out = root.join("out");

    let dials: Vec<Dial> = serde_json::from_value(json!([{"id":"crookedness",
        "path":"/skeleton/habit/crookedness","meaning":"turn variation",
        "min":0.,"max":15.,"small":1.,"substantial":3.,"integer":false}]))
    .unwrap();

    let protocol = root.join("adapter.py");
    script(&protocol, ADAPTER);
    let measure = root.join("measure.sh");
    script(&measure, MEASURE);
    let headless = root.join("headless.sh");
    script(&headless, HEADLESS);
    let compare = root.join("compare.py");
    script(&compare, COMPARE);
    let profiles = root.join("profiles.json");
    write(&profiles, &json!({"note":SYNTHETIC}));

    let shot = |foliage: &str| {
        json!({"aspect":[2,3],"foliage":foliage,
            "light":{"overcast":0.5,"sunAzimuth":30.,"sunElevation":40.},
            "camera":{"azimuth":0.,"elevation":10.}})
    };
    let references_file = root.join("references.json");
    write(
        &references_file,
        &json!({"references":[{"id":"whole","shot":shot("leaf-on")}]}),
    );

    let reference = image(&root, "whole", 1);
    let anchor = image(&root, "anchor", 1);
    let references = vec![reference.clone()];

    // Stage A: the inventory the runtime pins, and the receipt that attributes it.
    let ref_request = ReferenceRequest {
        protocol: VERSION.into(),
        target_species: PRESET.into(),
        references: vec![ReferenceImage {
            id: "reference-0".into(),
            image: serde_json::from_value(reference.clone()).unwrap(),
        }],
        specimen_relationship: "unknown".into(),
    };
    let inventory = Inventory {
        request_sha256: ref_request.hash(),
        prompt_sha256: ref_request.prompt_hash(),
        request: ref_request,
        model: VISION_MODEL.into(),
        effort: EFFORT.into(),
        ledger: SYNTHETIC.into(),
        traits: vec![Trait {
            id: "trait-core".into(),
            priority: Priority::Core,
            observation: "synthetic core recognition trait".into(),
            reference_ids: vec!["reference-0".into()],
            uncertain: false,
        }],
        observations: vec![SYNTHETIC.into()],
    };
    let inventory_path = root.join("inventory.json");
    let inventory_sha = write(&inventory_path, &serde_json::to_value(&inventory).unwrap());
    let preparation_path = root.join("preparation.json");
    let preparation_sha = write(
        &preparation_path,
        &json!({"status":"ok","model":VISION_MODEL,"effort":EFFORT,
            "request_sha256":inventory.request_sha256,"prompt_sha256":inventory.prompt_sha256,
            "usage":{"input_tokens":700,"output_tokens":300},
            "answer":{"traits":inventory.traits,"observations":inventory.observations}}),
    );

    let config = json!({
        "preset":PRESET,"seed":1,"initial_overrides":{},
        "owner_notes":"synthetic owner notes for an offline fixture",
        "dials":dials,
        "measure_binary":measure,"profiles":profiles,"profile_id":PRESET,
        "matched":{"headless":headless,"compare_script":compare,"references":references_file,
            "refs":root,"catalogue":root,"scratch":root.join("scratch"),
            "height":64,"numeric_references":["whole"]},
        "vision":{"program":"python3","args":[protocol,"--model",VISION_MODEL,"--effort",EFFORT],
            "model":VISION_MODEL,"effort":EFFORT,"timeout_seconds":60,
            "ledger":root.join("vision-ledger")},
        "sheet":{"adapter":{"program":"python3","args":[protocol,"--model",VISION_MODEL,"--effort",EFFORT],
            "model":VISION_MODEL,"effort":EFFORT,"timeout_seconds":60,
            "ledger":root.join("sheet-ledger")},"protocol":protocol},
        "references":references,
        "required":[{"item":"reference_character","view":"whole","seed":1},
            {"item":"reference_character","view":"whole","seed":42}],
        "checklist":SYNTHETIC,
        "quality_anchors":[{"image":anchor,"provenance":SYNTHETIC,"scope":"finish only"}],
        "reference_first":{"inventory":{"path":inventory_path,"sha256":inventory_sha},
            "preparation":{"path":preparation_path,"sha256":preparation_sha}},
        "judgment_model":JUDGMENT_MODEL,
        "ledger":root.join("jev-ledger"),
        "budget":budget});
    let config_path = root.join("config.json");
    fs::write(&config_path, serde_json::to_vec_pretty(&config).unwrap()).unwrap();
    Fixture {
        root,
        config_path,
        out,
        config,
        dials,
    }
}

/// A minimal run carrying the trials a progress review compares. Nothing here
/// dispatches: it exists so the request builder has a state to read.
pub fn progress_run(trials: Vec<telperion_jev::tuning::evaluation::Trial>) -> Run {
    Run {
        identity: "progress-fixture".into(),
        preset: PRESET.into(),
        seed: 1,
        effective: json!({}),
        overrides: json!({}),
        dials: vec![],
        owner_notes: "owner notes".into(),
        required: vec![],
        budget: Default::default(),
        usage_known: true,
        trials,
        current: Some(0),
        visual: None,
        stopped: None,
        machine_ready: false,
        pending: None,
        routes: vec![],
        approval: None,
        preparation_charge: None,
        priority_checkpoints: vec![],
        judgment_inputs: vec![],
        visual_bootstrap: true,
        reviewer_passed_unqualified: false,
        strides: Default::default(),
        unkept: None,
    }
}
