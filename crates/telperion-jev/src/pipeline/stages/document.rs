//! The species documentation: a markdown copy of every admitted source, the
//! article distilled from them, and the citation check re-run over its claims.
//!
//! The copy's file format, the rights rule that shapes it and the article's
//! structure belong to the catalogue scripts, which this stage runs and never
//! restates. The stage owns what their output means for the run: a claim the
//! check lists raises a decision the runner stops on, so an unsupported
//! sentence cannot ship, and an article it leaves unfilled is logged.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

use regex::Regex;
use serde_json::{json, Value};

use crate::cite::{cite, load_source, ResearchClaim, SourceLoad};
use crate::pipeline::canon::{read_json, write_atomic};
use crate::pipeline::decision::{append_decisions, Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::stage::{log_command, Context, Paths, StageError};

use super::extract::cached_markdown;
use super::{body, inputs};

pub const STAGE: &str = "document";
const SOURCES_SCRIPT: &str = "scripts/catalogue-sources.mjs";
const ARTICLE_SCRIPT: &str = "scripts/catalogue-article.mjs";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { decisions: Vec<String> },
}

pub fn run(paths: &Paths, judge: &Judge<'_>) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let (_, select_sha) = body(&ctx, STAGE, "select")?;
    let (_, generate_sha) = body(&ctx, STAGE, "generate")?;
    // The article is written by a person or the add-species agent after
    // the scaffold; its bytes key the cite check, so a written article is
    // checked on the next run (fn-149).
    let species = ctx.admitted.manifest.species.clone();
    let article = std::path::Path::new("catalogue")
        .join(&species)
        .join("ARTICLE.md");
    let article_sha = std::fs::read(&article).map_or("absent".into(), |b| crate::sha256_hex(&b));
    let pinned = inputs(&[
        ("fetch.json", &fetch_sha),
        ("select.json", &select_sha),
        ("generate.json", &generate_sha),
        ("ARTICLE.md", &article_sha),
    ]);
    let mut header = ctx.header(STAGE, "document", pinned, vec![]);
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let bound: BTreeMap<String, String> =
        [("select.json".into(), select_sha)].into_iter().collect();
    let mut decisions = Vec::new();
    let copies = write_copies(&ctx, &fetch, &species)?;
    let (validated, work) = write_article(&ctx, &species)?;
    if !validated {
        let parts = DecisionParts {
            species: &species,
            stage: STAGE,
            kind: "article-unfilled",
            field: None,
            age_years: None,
        };
        decisions.push(Decision::new(
            parts,
            &[],
            bound.clone(),
            vec![],
            json!({"work": work}),
            &["accept", "write-article"],
            "The article still has unfilled sections or uncited claims.",
        ));
    }
    // The catalogue script writes the article into the species' catalogue
    // folder, read from the repository root as the script runs.
    let written = std::fs::read_to_string(&article).unwrap_or_default();
    let folder = article.parent().unwrap_or(std::path::Path::new("."));
    let (claims, loads) = claims_in(&ctx, judge, folder, &written);
    let failed = |err: crate::caller::CallerError| StageError::Failed {
        stage: STAGE.into(),
        reason: err.to_string(),
    };
    let report = cite(
        judge.transport,
        judge.key,
        &judge.ledger_dir,
        &claims,
        &loads,
    )
    .map_err(failed)?;
    let mut rows = Vec::new();
    for (claim, row) in claims.iter().zip(report.rows.iter()) {
        if !row.identity.is_empty() {
            header.ledger.push(row.identity.clone());
        }
        rows.push(json!({
            "claim": row.claim, "relation": row.relation, "listed": row.listed,
            "reason": row.reason, "section": row.section, "ledger": row.identity,
        }));
        if row.listed && row.relation != "unchecked" {
            let contradicts = row.relation == "contradicts";
            let kind = if contradicts {
                "article-claim-contradicted"
            } else {
                "article-claim-unsupported"
            };
            let parts = DecisionParts {
                species: &species,
                stage: STAGE,
                kind,
                field: Some(&claim.source_id),
                age_years: None,
            };
            let payload = json!({"claim": row.claim, "section": row.section, "relation": row.relation, "reason": row.reason});
            decisions.push(Decision::new(
                parts,
                &[],
                bound.clone(),
                vec![row.identity.clone()],
                payload,
                &["accept", "recite", "rewrite-sentence"],
                "The citation check listed this article claim for a person.",
            ));
        }
    }
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    ctx.write(
        &header,
        json!({
            "sources": copies, "article": article.display().to_string(),
            "article_validated": validated, "article_work": work, "claims": rows,
            "tokens": {"input": report.input_tokens, "output": report.output_tokens},
        }),
    )?;
    Ok(Outcome::Ran { decisions: ids })
}

/// One markdown copy per admitted source, written by the catalogue script from
/// the text the run already cached. A source whose cache entry is gone or has
/// moved is copied as unavailable rather than fetched again.
fn write_copies(ctx: &Context, fetch: &Value, species: &str) -> Result<Vec<Value>, StageError> {
    let mut written = Vec::new();
    for source in &ctx.admitted.manifest.sources {
        let cached = verified_cache(ctx, &source.id, &fetch["sources"][&source.id]);
        let mut argv = args(&[
            SOURCES_SCRIPT,
            "--species",
            species,
            "--source",
            &source.id,
            "--fetched",
            &today(),
        ]);
        if let Some(path) = &cached {
            argv.extend(args(&["--from", &path.display().to_string()]));
            if let Some(passages) = passages_file(ctx, &source.id)? {
                argv.extend(args(&["--passages", &passages.display().to_string()]));
            }
        } else {
            argv.push("--unavailable".into());
        }
        let (ok, printed, failed) = node(&ctx.paths, &argv)?;
        if !ok {
            let tail = failed.trim().lines().next_back().unwrap_or("no output");
            return Err(StageError::Failed {
                stage: STAGE.into(),
                reason: format!("{}: {tail}", source.id),
            });
        }
        written
            .push(json!({"id": source.id, "form": form_in(&printed), "cached": cached.is_some()}));
    }
    Ok(written)
}

/// The cached file `--from` names, once the checksum the fetch stage recorded
/// still matches its bytes.
fn verified_cache(ctx: &Context, id: &str, record: &Value) -> Option<PathBuf> {
    cached_markdown(ctx, STAGE, id, record).ok()?;
    Some(
        ctx.paths
            .cache()
            .join(record["cached"]["markdown"].as_str()?),
    )
}

/// The passages an extract keeps for one source: each provenance entry's JSON
/// pointer as the location and the span it copied as the quote, which the
/// script verifies is a literal run of the fetched text. None when provenance
/// holds nothing for this source, which the script writes as an empty extract.
/// Only a `copied` route is quotable; a described value's span is the judge's
/// paraphrase and would fail that verification for the right reason.
fn passages_file(ctx: &Context, id: &str) -> Result<Option<PathBuf>, StageError> {
    if !ctx.paths.sidecar().exists() {
        return Ok(None);
    }
    let provenance = read_json(&ctx.paths.sidecar())?;
    let entries = provenance["entries"].as_object().into_iter().flatten();
    let mine = entries.filter(|(_, entry)| {
        entry["source"].as_str() == Some(id) && entry["route"].as_str() == Some("copied")
    });
    let passage =
        |(pointer, entry): (&String, &Value)| json!({"location": pointer, "quote": entry["span"]});
    let passages: Vec<Value> = mine.map(passage).collect();
    if passages.is_empty() {
        return Ok(None);
    }
    let path = ctx.paths.run.join("document").join(format!("{id}.json"));
    write_atomic(&path, &serde_json::to_vec(&passages).unwrap_or_default())?;
    Ok(Some(path))
}

/// Scaffolds or refreshes the article. A non-zero exit is the work the script
/// lists for the writer - an unfilled section, an uncited claim - and not a
/// crash, so the stage records it and finishes.
fn write_article(ctx: &Context, species: &str) -> Result<(bool, Vec<String>), StageError> {
    let (ok, _, failed) = node(&ctx.paths, &args(&[ARTICLE_SCRIPT, "--species", species]))?;
    let work = failed
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty());
    Ok((ok, work.map(str::to_string).collect()))
}

/// Every claim the article makes, one per cited source. Each is checked
/// against the copy beside the article, so the check reads the text a reader
/// following the link would.
fn claims_in(
    ctx: &Context,
    judge: &Judge<'_>,
    folder: &std::path::Path,
    article: &str,
) -> (Vec<ResearchClaim>, Vec<SourceLoad>) {
    let urls = source_urls(ctx);
    let mut claims = Vec::new();
    let mut loads = Vec::new();
    for (line, id) in cited(article) {
        let copy = folder.join("sources").join(format!("{id}.md"));
        loads.push(load_source(judge.transport, &copy.display().to_string()));
        let url = urls.get(&id).cloned().unwrap_or_default();
        claims.push(ResearchClaim {
            claim: line,
            url,
            source_id: id,
            unresolved: None,
        });
    }
    (claims, loads)
}

/// The article's claims and their citations: every body line outside a
/// generated block that is not a heading, paired with each
/// `[ID](sources/ID.md)` link it carries.
fn cited(article: &str) -> Vec<(String, String)> {
    let citation = Regex::new(r"\[([A-Z0-9][A-Z0-9-]*)\]\(sources/([A-Z0-9][A-Z0-9-]*)\.md\)")
        .expect("the citation pattern compiles");
    let mut found = Vec::new();
    let mut generated = false;
    for line in article.lines() {
        if line.contains("<!-- generated:") {
            generated = true;
        } else if line.contains("<!-- /generated -->") {
            generated = false;
        } else if !generated && !line.trim().is_empty() && !line.starts_with('#') {
            // A label linking to another source's copy is the article check's
            // failure to report, not a claim against that source.
            let own = citation.captures_iter(line).filter(|c| c[1] == c[2]);
            found.extend(own.map(|c| (line.trim().to_string(), c[1].to_string())));
        }
    }
    found
}

/// The url of every source, as the catalogue's `sources.json` records it.
fn source_urls(ctx: &Context) -> BTreeMap<String, String> {
    let record = read_json(&ctx.paths.dir.join("sources.json")).unwrap_or_default();
    let sources = record["sources"].as_array().cloned().unwrap_or_default();
    let pair = |s: &Value| {
        Some((
            s["id"].as_str()?.to_string(),
            s["url"].as_str()?.to_string(),
        ))
    };
    sources.iter().filter_map(pair).collect()
}

/// One catalogue script, run from the repository root and appended to the
/// run's command log, with what it printed and whether it succeeded.
fn node(paths: &Paths, argv: &[String]) -> Result<(bool, String, String), StageError> {
    let output = Command::new("node")
        .args(argv)
        .output()
        .map_err(|err| StageError::Failed {
            stage: STAGE.into(),
            reason: format!("node {}: {err}", argv.join(" ")),
        })?;
    let mut logged = vec!["node".to_string()];
    logged.extend(argv.iter().cloned());
    log_command(paths, &logged, output.status.code().unwrap_or(-1))?;
    let printed = |bytes: &[u8]| String::from_utf8_lossy(bytes).into_owned();
    Ok((
        output.status.success(),
        printed(&output.stdout),
        printed(&output.stderr),
    ))
}

/// The copy's form, as the script printed it in `wrote <path> (<form>, N
/// bytes)`. The rights rule that chose it is the script's alone.
fn form_in(printed: &str) -> String {
    let (_, tail) = printed.rsplit_once('(').unwrap_or(("", "unknown,"));
    tail.split(',')
        .next()
        .unwrap_or("unknown")
        .trim()
        .to_string()
}

fn args(argv: &[&str]) -> Vec<String> {
    argv.iter().map(|arg| arg.to_string()).collect()
}

/// The date the copies record as their fetch date, pinnable like every other
/// clock in the pipeline.
fn today() -> String {
    let now = crate::pipeline::stage::now();
    now.get(..10).unwrap_or(&now).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_claim_is_a_body_line_per_cited_source_and_never_a_generated_block() {
        let article = "---\nspecies: x\n---\n<!-- generated: header -->\nAsh [A1](sources/A1.md)\n\
            <!-- /generated -->\n\n## Bark\n\nThe bark greys [A1](sources/A1.md), deeply [B2](sources/B2.md).\n\
            \nA line with no citation.\n\nA mislinked one [A1](sources/B2.md).\n";
        let found = cited(article);
        let ids: Vec<&str> = found.iter().map(|(_, id)| id.as_str()).collect();
        assert_eq!(ids, vec!["A1", "B2"]);
        assert_eq!(
            form_in("wrote catalogue/x/sources/A1.md (extract, 812 bytes)\n"),
            "extract"
        );
    }
}
