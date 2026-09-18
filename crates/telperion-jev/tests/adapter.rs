//! Fetch adapter contract (fn-58 R2). No test here reaches the network: the
//! fixture adapter reads files and the CLI adapter runs a fake program.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use telperion_jev::pipeline::adapter::firecrawl::{
    parse_args, research_args, scrape_args, search_args,
};
use telperion_jev::pipeline::adapter::tables::{
    age_indexed_rows, coverage, markdown_tables, table_rows_for,
};
use telperion_jev::pipeline::adapter::{
    checksums, is_pdf, AdapterError, FetchAdapter, FirecrawlCli, FixtureAdapter, RawSource, Scrape,
};

const OWIC: &str = "https://research.fs.usda.gov/silvics/oregon-white-oak";

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/adapter")
}

fn fixture_adapter() -> FixtureAdapter {
    FixtureAdapter::new(fixtures())
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-adapter-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Writes a shell script standing in for the installed CLI. It records its
/// arguments and echoes the JSON shape the real CLI printed on 2026-09-18.
static CLI_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// The guard serializes the tests that fork the fake CLI: a script written by
/// one test while another forks is `Text file busy` (ETXTBSY) on Linux.
fn fake_cli(dir: &Path, body: &str) -> (String, std::sync::MutexGuard<'static, ()>) {
    let guard = CLI_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let path = dir.join("fake-firecrawl");
    let script = format!(
        "#!/bin/sh\nfor a in \"$@\"; do printf '%s\\n' \"$a\" >> \"{}/argv.txt\"; done\n{body}\n",
        dir.display()
    );
    fs::write(&path, script).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    (path.display().to_string(), guard)
}

fn recorded_argv(dir: &Path) -> Vec<String> {
    fs::read_to_string(dir.join("argv.txt"))
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

const CANNED: &str = r##"
case "$1" in
  scrape)
    printf '%s' '{"markdown":"# Oregon White Oak","rawHtml":"<html>oak</html>","links":["https://example.org"],"metadata":{"statusCode":200,"sourceURL":"https://research.fs.usda.gov/silvics/oregon-white-oak","url":"https://research.fs.usda.gov/silvics/oregon-white-oak","contentType":"text/html; charset=utf-8"}}'
    ;;
  search)
    printf '%s' '{"success":true,"data":{"web":[{"url":"https://research.fs.usda.gov/treesearch/39910","title":"Growth of Oregon white oak (Quercus garryana)","description":"Our results show the sensitivity of Oregon white oak to competition.","position":1}]},"creditsUsed":2}'
    ;;
  research)
    printf '%s' '{"success":true,"partial":false,"results":[{"paperId":"4510443524802724439","primaryId":"pmcid:PMC2987550","ids":{"doi":["10.1016/j.foreco.2010.07.055"],"pmcid":["PMC2987550"]},"title":"Do individual-tree growth models correctly represent height:diameter ratios?","abstract":"Height:diameter ratios are an important measure of stand stability.","score":0.88}]}'
    ;;
  parse)
    printf '%s' '{"markdown":"| Alter | I |\n| --- | --- |\n| 20 | 9,8 |","metadata":{"statusCode":200}}'
    ;;
esac
"##;

#[test]
fn fixture_adapter_round_trips_scrape_search_research_and_parse() {
    let adapter = fixture_adapter();

    let scrape = adapter.scrape(OWIC).unwrap();
    assert_eq!(scrape.final_url, OWIC);
    assert_eq!(scrape.content_type, "text/html; charset=utf-8");
    assert!(scrape.raw.starts_with(b"<!DOCTYPE html>"));
    assert!(scrape.markdown.starts_with("# Oregon White Oak"));

    let hits = adapter
        .search("Quercus garryana mature height", 10)
        .unwrap();
    assert_eq!(hits.len(), 2);
    assert_eq!(
        hits[0].url,
        "https://owic.oregonstate.edu/oregon-white-oak-quercus-garryana"
    );
    assert_eq!(
        adapter
            .search("Quercus garryana mature height", 1)
            .unwrap()
            .len(),
        1
    );

    let papers = adapter
        .research(
            "Norway spruce height diameter ratio individual tree growth models",
            5,
        )
        .unwrap();
    assert_eq!(papers.len(), 1);
    assert_eq!(
        papers[0].url,
        "https://doi.org/10.1016/j.foreco.2010.07.055"
    );

    let markdown = adapter
        .parse_pdf(Path::new("/tmp/ertragstafeln.pdf"))
        .unwrap();
    assert!(markdown.contains("| 120 | 41,3 |"));
}

#[test]
fn fixture_adapter_fails_on_an_unknown_url() {
    let err = fixture_adapter()
        .scrape("https://example.org/missing")
        .unwrap_err();
    match err {
        AdapterError::Failed { url, error } => {
            assert_eq!(url, "https://example.org/missing");
            assert!(error.contains("not in the fixture index"), "{error}");
        }
        other => panic!("{other}"),
    }
}

#[test]
fn checksums_separate_the_raw_response_from_the_markdown() {
    let base = fixture_adapter().scrape(OWIC).unwrap();
    let record = checksums(&base);

    let mut changed = base.clone();
    changed.markdown.push('!');
    let after = checksums(&changed);

    assert_eq!(after.raw_sha256, record.raw_sha256);
    assert_eq!(after.raw_bytes, record.raw_bytes);
    assert_ne!(after.markdown_sha256, record.markdown_sha256);
    assert_eq!(after.markdown_bytes, record.markdown_bytes + 1);
    assert_eq!(record.final_url, OWIC);

    let json = serde_json::to_string(&record).unwrap();
    let sorted = r#"{"content_type":"#;
    assert!(json.starts_with(sorted), "{json}");
    assert!(json.find("markdown_sha256").unwrap() < json.find("raw_sha256").unwrap());
}

#[test]
fn the_spruce_table_yields_eleven_age_indexed_rows_with_commas_converted() {
    let markdown = fs::read_to_string(fixtures().join("ertragstafeln.md")).unwrap();
    let tables = markdown_tables(&markdown);
    assert_eq!(tables.len(), 2);

    let rows = age_indexed_rows(&tables[0]);
    assert_eq!(rows.len(), 11);
    assert_eq!(rows.first().unwrap().age_years, 20.0);
    assert_eq!(rows.last().unwrap().age_years, 120.0);
    // The 30-year row, second site class: "12,6" is 12.6 metres.
    assert_eq!(rows[1].values[1], Some(12.6));
    // "n. a." is not a number.
    assert_eq!(rows[0].values[5], None);
    assert_eq!(rows[0].values[0], Some(9.8));

    let via_index = table_rows_for(&markdown, 0).unwrap();
    assert_eq!(via_index, rows);
    assert!(table_rows_for(&markdown, 9).is_none());

    let complete = coverage(&rows, 11);
    assert_eq!(complete.found, 11);
    assert_eq!(complete.expected, 11);
    assert!(complete.complete);
}

#[test]
fn a_flattened_table_is_a_coverage_gap() {
    let markdown = fs::read_to_string(fixtures().join("ertragstafeln.md")).unwrap();
    let rows = table_rows_for(&markdown, 1).unwrap();
    assert_eq!(rows.len(), 4);

    let gap = coverage(&rows, 11);
    assert_eq!(gap.found, 4);
    assert_eq!(gap.expected, 11);
    assert!(!gap.complete);
}

#[test]
fn is_pdf_reads_the_content_type_then_the_url() {
    let cases: &[(&str, &str, bool)] = &[
        ("application/pdf", "https://example.org/a", true),
        (
            "application/pdf; charset=binary",
            "https://example.org/a",
            true,
        ),
        ("APPLICATION/PDF", "https://example.org/a", true),
        (
            "text/html; charset=utf-8",
            "https://example.org/a.pdf",
            false,
        ),
        ("", "https://example.org/tafeln.pdf", true),
        ("", "https://example.org/tafeln.pdf?v=2024#page=3", true),
        (
            "application/octet-stream",
            "https://example.org/tafeln.PDF",
            true,
        ),
        ("", "https://example.org/page.html", false),
        ("text/html", "https://example.org/page.html", false),
    ];
    for (content_type, url, expected) in cases {
        assert_eq!(is_pdf(content_type, url), *expected, "{content_type} {url}");
    }
}

#[test]
fn the_cli_adapter_builds_its_arguments_and_reads_the_output() {
    let dir = scratch("cli");
    let (program, _cli) = fake_cli(&dir, CANNED);
    let mut cli = FirecrawlCli::with_program(program, dir.join("cache"));
    assert_eq!(cli.raw_from, RawSource::Cli);

    let scrape = cli.scrape(OWIC).unwrap();
    assert_eq!(scrape.final_url, OWIC);
    assert_eq!(scrape.content_type, "text/html; charset=utf-8");
    assert_eq!(scrape.raw, b"<html>oak</html>");
    assert_eq!(scrape.markdown, "# Oregon White Oak");
    assert_eq!(recorded_argv(&dir), scrape_args(OWIC));
    fs::remove_file(dir.join("argv.txt")).unwrap();

    let hits = cli.search("Oregon white oak growth", 3).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].url, "https://research.fs.usda.gov/treesearch/39910");
    assert!(hits[0].snippet.contains("sensitivity"));
    assert_eq!(
        recorded_argv(&dir),
        search_args("Oregon white oak growth", 3)
    );
    fs::remove_file(dir.join("argv.txt")).unwrap();

    let papers = cli
        .research("height diameter ratios Norway spruce", 5)
        .unwrap();
    assert_eq!(
        papers[0].url,
        "https://doi.org/10.1016/j.foreco.2010.07.055"
    );
    assert_eq!(
        recorded_argv(&dir),
        research_args("height diameter ratios Norway spruce", 5)
    );
    fs::remove_file(dir.join("argv.txt")).unwrap();

    let pdf = dir.join("ertragstafeln.pdf");
    let markdown = cli.parse_pdf(&pdf).unwrap();
    assert_eq!(age_indexed_rows(&markdown_tables(&markdown)[0]).len(), 1);
    assert_eq!(recorded_argv(&dir), parse_args(&pdf));

    // No argument list names an option that puts a model in the path.
    for args in [
        scrape_args(OWIC),
        search_args("q", 1),
        research_args("q", 1),
        parse_args(&pdf),
    ] {
        for arg in args {
            assert!(
                !["--schema", "--schema-file", "-Q", "--query", "agent"].contains(&arg.as_str()),
                "{arg}"
            );
            assert!(!arg.split(',').any(|format| format == "json"), "{arg}");
        }
    }

    cli.raw_from = RawSource::Direct;
    assert_eq!(cli.raw_from, RawSource::Direct);
}

#[test]
fn a_rejected_credential_is_unauthenticated_and_every_other_failure_is_failed() {
    let dir = scratch("unauth");
    let (program, first) = fake_cli(
        &dir,
        "echo 'Error: Request failed with status 401 Unauthorized' >&2\nexit 1",
    );
    let cli = FirecrawlCli::with_program(program, dir.join("cache"));
    match cli.scrape(OWIC).unwrap_err() {
        AdapterError::Unauthenticated(message) => {
            assert!(message.contains(OWIC), "{message}");
            assert!(message.contains("401"), "{message}");
        }
        other => panic!("{other}"),
    }

    drop(first);
    let dir = scratch("failed");
    let (program, _cli) = fake_cli(&dir, "echo 'Error: connect ETIMEDOUT' >&2\nexit 2");
    let cli = FirecrawlCli::with_program(program, dir.join("cache"));
    match cli.scrape(OWIC).unwrap_err() {
        AdapterError::Failed { url, error } => {
            assert_eq!(url, OWIC);
            assert!(error.contains("ETIMEDOUT"), "{error}");
        }
        other => panic!("{other}"),
    }

    let missing = FirecrawlCli::with_program("firecrawl-not-installed", scratch("missing"));
    match missing.search("q", 1).unwrap_err() {
        AdapterError::Command(message) => assert!(message.contains("firecrawl-not-installed")),
        other => panic!("{other}"),
    }
}

#[test]
fn a_pdf_scrape_names_a_file_under_the_cache_directory() {
    let dir = scratch("cache");
    let cli = FirecrawlCli::with_program("firecrawl", dir.join("cache"));
    let scrape = Scrape {
        final_url: "https://example.org/tafeln.pdf".into(),
        content_type: "application/pdf".into(),
        raw: b"%PDF-1.7".to_vec(),
        markdown: "| 20 | 9,8 |".into(),
    };
    let path = cli.cache_path_for(&scrape.raw);
    assert!(path.starts_with(dir.join("cache")));
    assert_eq!(
        path.file_name().unwrap().to_str().unwrap(),
        format!("{}.pdf", checksums(&scrape).raw_sha256)
    );
}
