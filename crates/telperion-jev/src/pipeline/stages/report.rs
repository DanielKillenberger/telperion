//! The run's report: one artifact and one page a person reads.
//!
//! The stage reads every earlier artifact that exists, the decision list and
//! the sidecar, and writes `report.json` beside `report.md`. It judges
//! nothing and asks nothing: the sources are the checksums the fetch stage
//! recorded, the fields are the levels the quality gate scored beside the
//! values selection filled, the curves are the fit's own numbers, and the
//! ledger references are the ones the artifacts' headers already carry.
//!
//! The status is `halted` while any decision is open and `complete` only when
//! every one is resolved, so a run that stopped at a gap says so. The cost
//! section sums what every stage's artifact records it spent, Firecrawl
//! credits and Jev calls, per stage and in total. No probability is written
//! here, and the stills are named, never judged.

use serde_json::{json, Map, Value};

use crate::pipeline::canon::{file_sha256, read_json, write_atomic};
use crate::pipeline::cost::Cost;
use crate::pipeline::gap::metrics;
use crate::pipeline::stage::{Context, Paths, StageError, STAGES};

use super::inputs;

pub const STAGE: &str = "report";
/// The artifacts whose bodies the report reads, in the order the run wrote
/// them; every stage's artifact is read for its cost.
const EARLIER: [&str; 8] = [
    "fetch", "screen", "quality", "select", "verify", "fit", "gate", "generate",
];

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { status: String },
}

pub fn run(paths: &Paths) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let mut read = Map::new();
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut ledger: Vec<String> = Vec::new();
    let mut costs = Map::new();
    let mut total = Cost::default();
    for name in STAGES.iter().filter(|name| **name != STAGE) {
        let path = ctx.paths.artifact(name);
        if !path.exists() {
            continue;
        }
        let artifact = read_json(&path)?;
        let cost: Cost = serde_json::from_value(artifact["cost"].clone()).unwrap_or_default();
        total.add(&cost);
        costs.insert(name.to_string(), json!(cost));
        pairs.push((format!("{name}.json"), file_sha256(&path)?));
        if !EARLIER.contains(name) {
            continue;
        }
        ledger.extend(strings(&artifact["ledger"]));
        read.insert(name.to_string(), artifact["body"].clone());
    }
    // `metrics.json` is an input, not just a read: writing the run's numbers
    // has to expire the report's key, or a report written before them would
    // stay current and keep saying they are missing.
    for sidecar in [
        ctx.paths.decisions(),
        ctx.paths.sidecar(),
        metrics::metrics_path(&ctx.paths),
    ] {
        if sidecar.exists() {
            let name = sidecar.file_name().unwrap_or_default().to_string_lossy();
            pairs.push((name.into_owned(), file_sha256(&sidecar)?));
        }
    }
    ledger.sort();
    ledger.dedup();
    let borrowed: Vec<(&str, &str)> = pairs
        .iter()
        .map(|(name, sha)| (name.as_str(), sha.as_str()))
        .collect();
    let header = ctx.header(STAGE, "report", inputs(&borrowed), ledger.clone());
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }

    let decisions: Vec<Value> = ctx
        .decisions
        .iter()
        .map(|d| json!({"id": d.id, "kind": d.kind, "status": d.status}))
        .collect();
    let halted = decisions.iter().any(|d| d["status"] == json!("open"));
    // The run's three numbers are the gap loop's record (fn-63 R5). A run
    // with no metrics record is not complete, whatever its decisions say:
    // the report names the missing record rather than reporting a run whose
    // autonomy, quality and efficiency nobody can read.
    let numbers = metrics::metrics_path(&ctx.paths);
    let status = match (halted, numbers.exists()) {
        (true, _) => "halted",
        (false, false) => "incomplete",
        (false, true) => "complete",
    };
    let body = json!({
        "species": ctx.admitted.manifest.species,
        "status": status,
        "metrics": if numbers.exists() {
            read_json(&numbers).unwrap_or_else(|_| json!({"unreadable": true}))
        } else {
            json!({"missing": "metrics.json; run `species-pipeline gap metrics`"})
        },
        "sources": sources(&read),
        "fields": fields(&read),
        "curves": curves(&read),
        "decisions": decisions,
        "stills": read.get("generate").map_or(json!([]), |g| g["stills"].clone()),
        "costs": {"stages": costs, "total": total},
        "ledger": ledger,
    });
    write_atomic(&ctx.paths.dir.join("report.md"), page(&body).as_bytes())?;
    ctx.write(&header, body)?;
    Ok(Outcome::Ran {
        status: status.into(),
    })
}

/// Every admitted source with the checksums the fetch stage recorded.
fn sources(read: &Map<String, Value>) -> Vec<Value> {
    let Some(fetch) = read.get("fetch") else {
        return Vec::new();
    };
    fetch["sources"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(id, record)| {
            json!({
                "id": id,
                "final_url": record["final_url"],
                "raw_sha256": record["raw_sha256"],
                "markdown_sha256": record["markdown_sha256"],
            })
        })
        .collect()
}

/// Every evidence field: the level the quality gate scored, and the value
/// selection filled or the reason it is unavailable.
fn fields(read: &Map<String, Value>) -> Map<String, Value> {
    let quality = read.get("quality").cloned().unwrap_or_else(|| json!({}));
    let select = read.get("select").cloned().unwrap_or_else(|| json!({}));
    let mut out = Map::new();
    for (name, entry) in quality["fields"].as_object().into_iter().flatten() {
        let pointer = format!("/profiles/0/metrics/{name}");
        let filled = select["filled"].get(&pointer).cloned();
        let value = match filled {
            Some(metric) => json!({"filled": metric["range"], "unit": metric["unit"]}),
            None => json!({"unavailable": select["unavailable"][name]}),
        };
        out.insert(
            name.clone(),
            json!({"level": entry["level"], "bar": entry["bar"], "value": value}),
        );
    }
    out
}

/// The fit's own numbers, with the reference values it compared at each age.
fn curves(read: &Map<String, Value>) -> Value {
    let Some(fit) = read.get("fit") else {
        return Value::Null;
    };
    json!({
        "rate": fit["rate"],
        "shape": fit["shape"],
        "derived_mature_age_years": fit["derived_mature_age_years"],
        "objective": fit["objective"],
        "rows": fit["rows"],
        "misses": fit["misses"],
        "unavailable": fit["unavailable"],
    })
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|v| v.as_str().map(str::to_string))
        .collect()
}

/// A cell's text: a string without its quotes, null as an empty cell, and any
/// other value as its compact JSON.
fn cell(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

fn table(head: &[&str], rows: Vec<Vec<String>>) -> String {
    if rows.is_empty() {
        return "None.\n".to_string();
    }
    let mut out = format!(
        "| {} |\n|{}|\n",
        head.join(" | "),
        " --- |".repeat(head.len())
    );
    for row in rows {
        out.push_str(&format!("| {} |\n", row.join(" | ")));
    }
    out
}

/// The page beside the artifact: one heading and one table per section.
fn page(body: &Value) -> String {
    let mut out = format!(
        "# {} pipeline report\n\nStatus: {}.\n\n## Sources\n\n",
        cell(&body["species"]),
        cell(&body["status"])
    );
    let rows = body["sources"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|s| {
            vec![
                cell(&s["id"]),
                cell(&s["final_url"]),
                cell(&s["raw_sha256"]),
                cell(&s["markdown_sha256"]),
            ]
        })
        .collect();
    out.push_str(&table(&["Source", "Final URL", "Raw", "Markdown"], rows));

    out.push_str("\n## Fields\n\n");
    let rows = body["fields"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(name, entry)| {
            let value = &entry["value"];
            let said = match value.get("filled") {
                Some(range) => format!("{} {}", cell(range), cell(&value["unit"])),
                None => cell(&value["unavailable"]),
            };
            vec![
                name.clone(),
                cell(&entry["level"]),
                cell(&entry["bar"]),
                said,
            ]
        })
        .collect();
    out.push_str(&table(&["Field", "Level", "Bar", "Value"], rows));

    out.push_str("\n## Curves\n\n");
    let rows = body["curves"]["rows"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|row| {
            vec![
                cell(&row["age_years"]),
                cell(&row["reference_height_m"]),
                cell(&row["model_height_m"]),
                cell(&row["reference_dbh_m"]),
                cell(&row["model_dbh_m"]),
            ]
        })
        .collect();
    let curves = &body["curves"];
    if !curves.is_null() {
        out.push_str(&format!(
            "Rate {}, shape {}, mature at {} years.\n\n",
            cell(&curves["rate"]),
            cell(&curves["shape"]),
            cell(&curves["derived_mature_age_years"])
        ));
    }
    out.push_str(&table(
        &[
            "Age",
            "Reference height",
            "Model height",
            "Reference DBH",
            "Model DBH",
        ],
        rows,
    ));

    out.push_str("\n## Decisions\n\n");
    let rows = body["decisions"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|d| vec![cell(&d["id"]), cell(&d["kind"]), cell(&d["status"])])
        .collect();
    out.push_str(&table(&["Decision", "Kind", "Status"], rows));

    out.push_str("\n## Stills\n\n");
    let rows = body["stills"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|s| {
            let said = s["path"].as_str().unwrap_or_else(|| cell_error(s));
            vec![cell(&s["still"]), said.to_string()]
        })
        .collect();
    out.push_str(&table(&["Still", "Path or error"], rows));

    out.push_str("\n## The run's numbers\n\n");
    let numbers = &body["metrics"];
    if let Some(missing) = numbers["missing"].as_str() {
        out.push_str(&format!("Missing: {missing}.\n"));
    } else {
        out.push_str(&table(
            &["Autonomy", "Quality", "Efficiency"],
            vec![vec![
                format!(
                    "{} gaps, {} routed, {} taken by the loop",
                    cell(&numbers["autonomy"]["gaps"]),
                    cell(&numbers["autonomy"]["routed"]),
                    cell(&numbers["autonomy"]["decisions"]["proceed"])
                ),
                format!(
                    "{} value rounds, {} reversals",
                    cell(&numbers["quality"]["rounds_total"]),
                    numbers["quality"]["reversals"]
                        .as_array()
                        .map_or(0, Vec::len)
                ),
                format!(
                    "{} in and {} out tokens, {} ms, {} captures",
                    cell(&numbers["efficiency"]["input_tokens"]),
                    cell(&numbers["efficiency"]["output_tokens"]),
                    cell(&numbers["efficiency"]["wall_clock_ms"]),
                    cell(&numbers["efficiency"]["captures"])
                ),
            ]],
        ));
    }

    out.push_str("\n## Cost\n\n");
    let cost_row = |name: &str, cost: &Value| {
        vec![
            name.to_string(),
            cell(&cost["runs"]),
            cell(&cost["firecrawl_credits"]),
            cell(&cost["firecrawl_method"]),
            cell(&cost["jev_calls"]),
        ]
    };
    let mut rows: Vec<Vec<String>> = body["costs"]["stages"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(name, cost)| cost_row(name, cost))
        .collect();
    rows.push(cost_row("total", &body["costs"]["total"]));
    out.push_str(&table(
        &["Stage", "Runs", "Firecrawl credits", "Counted", "Jev calls"],
        rows,
    ));
    out
}

fn cell_error(still: &Value) -> &str {
    still["error"].as_str().unwrap_or("not drawn")
}
