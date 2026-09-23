//! The Firecrawl CLI adapter (v1.23.3).
//!
//! Only the formats that return markdown, raw HTML, links and search results
//! are used. The `json` format, `--schema`, `--schema-file`, `--query` and the
//! `agent` command are never built into an argument list: each puts a model
//! that writes values into the fetch path.
//!
//! The CLI's `rawHtml` is a browser serialization, not the response bytes. A
//! scrape of `https://research.fs.usda.gov/silvics/oregon-white-oak` on
//! 2026-09-18 returned 105,177 bytes against the 103,163 bytes curl received,
//! differing from byte 15 on (`.flow/evidence/fn58/parked-checks.md`), so a
//! caller that checksums the original response picks `RawSource::Direct`.
//! That plain request trusts the host's certificate store (ureq's
//! `native-certs`), so a page Firecrawl scraped is not refused over a chain
//! the bundled roots do not carry, as the Missouri Botanical Garden page was
//! on the ash run of 2026-09-18.
//!
//! Credits: a scrape's metadata and a search's envelope carry `creditsUsed`;
//! the research index and parse report none, and those calls are estimated
//! at one credit each.

use std::cell::Cell;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use super::{is_pdf, AdapterError, FetchAdapter, Scrape, SearchHit, Spent};
use crate::sha256_hex;

/// Where a scrape's raw bytes come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawSource {
    /// The CLI's `rawHtml`, a serialization of the rendered DOM.
    Cli,
    /// A second plain GET, whose bytes are the response bytes.
    Direct,
}

pub struct FirecrawlCli {
    pub program: String,
    pub cache_dir: PathBuf,
    pub raw_from: RawSource,
    meter: Cell<Spent>,
}

impl FirecrawlCli {
    /// The installed CLI with its own cache directory, taking raw bytes from a
    /// plain GET.
    pub fn new() -> Self {
        Self {
            program: "firecrawl".into(),
            cache_dir: PathBuf::from(".firecrawl"),
            raw_from: RawSource::Direct,
            meter: Cell::new(Spent::default()),
        }
    }

    pub fn with_program(program: impl Into<String>, cache_dir: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            cache_dir: cache_dir.into(),
            raw_from: RawSource::Cli,
            meter: Cell::new(Spent::default()),
        }
    }

    /// The file a PDF's bytes are written to. The name is the checksum of the
    /// bytes, so two fetches of one document share one file.
    pub fn cache_path_for(&self, raw: &[u8]) -> PathBuf {
        self.cache_dir.join(format!("{}.pdf", sha256_hex(raw)))
    }

    /// Scrapes a PDF URL and writes its bytes under the cache directory.
    /// `parse_pdf` runs on the returned path.
    pub fn scrape_pdf(&self, url: &str) -> Result<(Scrape, PathBuf), AdapterError> {
        let (scrape, cached) = self.scrape_inner(url)?;
        match cached {
            Some(path) => Ok((scrape, path)),
            None => Err(AdapterError::Failed {
                url: url.to_string(),
                error: format!("response is not a PDF: {}", scrape.content_type),
            }),
        }
    }

    fn scrape_inner(&self, url: &str) -> Result<(Scrape, Option<PathBuf>), AdapterError> {
        let stdout = self.run(&scrape_args(url), url)?;
        let doc = parse_scrape(&stdout, url)?;
        let pdf = is_pdf(&doc.content_type, &doc.final_url);
        let raw = if pdf || self.raw_from == RawSource::Direct {
            fetch_raw(&doc.final_url)?.2
        } else {
            doc.raw_html.into_bytes()
        };
        let cached = if pdf {
            let path = self.cache_path_for(&raw);
            write_cached(&path, &raw)?;
            Some(path)
        } else {
            None
        };
        Ok((
            Scrape {
                final_url: doc.final_url,
                content_type: doc.content_type,
                raw,
                markdown: doc.markdown,
            },
            cached,
        ))
    }

    fn run(&self, args: &[String], subject: &str) -> Result<String, AdapterError> {
        let output = Command::new(&self.program)
            .args(args)
            .output()
            .map_err(|err| AdapterError::Command(format!("{}: {err}", self.program)))?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let mut spent = self.meter.get();
        spent.add(credits_used(&stdout));
        self.meter.set(spent);
        if output.status.success() {
            return Ok(stdout);
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        let message = format!("{stderr}{stdout}").trim().to_string();
        Err(classify(subject, &message))
    }
}

/// The credits one CLI response says it cost: the envelope's `creditsUsed`
/// (search), or the document metadata's (scrape). Absent on research and
/// parse output.
pub fn credits_used(stdout: &str) -> Option<u32> {
    let value: Value = serde_json::from_str(stdout.trim()).ok()?;
    let credits = [
        value.get("creditsUsed"),
        value.pointer("/data/creditsUsed"),
        value.pointer("/data/metadata/creditsUsed"),
    ]
    .into_iter()
    .flatten()
    .find_map(Value::as_u64)?;
    Some(credits as u32)
}

impl Default for FirecrawlCli {
    fn default() -> Self {
        Self::new()
    }
}

impl FetchAdapter for FirecrawlCli {
    fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        let stdout = self.run(&search_args(query, limit), query)?;
        parse_search(&stdout, query)
    }

    fn research(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        let stdout = self.run(&research_args(query, limit), query)?;
        parse_research(&stdout, query)
    }

    fn scrape(&self, url: &str) -> Result<Scrape, AdapterError> {
        Ok(self.scrape_inner(url)?.0)
    }

    fn parse_pdf(&self, path: &Path) -> Result<String, AdapterError> {
        let subject = path.display().to_string();
        let stdout = self.run(&parse_args(path), &subject)?;
        parse_markdown(&stdout, &subject)
    }

    fn spent(&self) -> Spent {
        self.meter.get()
    }
}

fn arg(value: &str) -> String {
    value.to_string()
}

/// Two formats or more make the CLI print one JSON object.
pub fn scrape_args(url: &str) -> Vec<String> {
    vec![
        arg("scrape"),
        url.to_string(),
        arg("-f"),
        arg("rawHtml,markdown,links"),
        arg("--json"),
    ]
}

pub fn search_args(query: &str, limit: usize) -> Vec<String> {
    vec![
        arg("search"),
        query.to_string(),
        arg("--limit"),
        limit.to_string(),
        arg("--json"),
    ]
}

pub fn research_args(query: &str, limit: usize) -> Vec<String> {
    vec![
        arg("research"),
        arg("search-papers"),
        query.to_string(),
        arg("--limit"),
        limit.to_string(),
        arg("--json"),
    ]
}

pub fn parse_args(path: &Path) -> Vec<String> {
    vec![
        arg("parse"),
        path.display().to_string(),
        arg("-f"),
        arg("markdown,links"),
        arg("--json"),
    ]
}

/// An HTTP status or message naming a rejected credential becomes
/// `Unauthenticated`; every other failure becomes `Failed`.
fn classify(subject: &str, message: &str) -> AdapterError {
    let lower = message.to_ascii_lowercase();
    let unauthenticated = ["unauthorized", "401", "unauthenticated", "invalid api key"]
        .iter()
        .any(|marker| lower.contains(marker));
    if unauthenticated {
        return AdapterError::Unauthenticated(format!("{subject}: {message}"));
    }
    AdapterError::Failed {
        url: subject.to_string(),
        error: message.to_string(),
    }
}

struct ScrapeDoc {
    final_url: String,
    content_type: String,
    raw_html: String,
    markdown: String,
}

fn as_json(stdout: &str, subject: &str) -> Result<Value, AdapterError> {
    serde_json::from_str(stdout.trim())
        .map_err(|err| AdapterError::Parse(format!("{subject}: {err}")))
}

fn text_at(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn parse_scrape(stdout: &str, url: &str) -> Result<ScrapeDoc, AdapterError> {
    let value = as_json(stdout, url)?;
    let body = value.get("data").unwrap_or(&value);
    let metadata = body.get("metadata").cloned().unwrap_or(Value::Null);
    // The metadata names the URL twice; the requested URL is the last resort.
    let final_url = [
        text_at(&metadata, "url"),
        text_at(&metadata, "sourceURL"),
        url.to_string(),
    ]
    .into_iter()
    .find(|candidate| !candidate.is_empty())
    .unwrap_or_default();
    let Some(status) = metadata.get("statusCode").and_then(Value::as_u64) else {
        return Err(classify(url, "the scrape states no status"));
    };
    if status >= 400 {
        return Err(classify(url, &format!("HTTP {status}")));
    }
    Ok(ScrapeDoc {
        final_url,
        content_type: text_at(&metadata, "contentType"),
        raw_html: text_at(body, "rawHtml"),
        markdown: text_at(body, "markdown"),
    })
}

fn parse_markdown(stdout: &str, subject: &str) -> Result<String, AdapterError> {
    let value = as_json(stdout, subject)?;
    let body = value.get("data").unwrap_or(&value);
    let markdown = text_at(body, "markdown");
    if markdown.is_empty() {
        return Err(AdapterError::Parse(format!(
            "{subject}: no markdown in output"
        )));
    }
    Ok(markdown)
}

fn parse_search(stdout: &str, query: &str) -> Result<Vec<SearchHit>, AdapterError> {
    let value = as_json(stdout, query)?;
    let web = value
        .get("data")
        .and_then(|data| data.get("web"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    Ok(web
        .iter()
        .map(|hit| SearchHit {
            url: text_at(hit, "url"),
            title: text_at(hit, "title"),
            snippet: text_at(hit, "description"),
        })
        .collect())
}

/// The paper index answers with source ids, not URLs, so the URL is built
/// from the strongest id: DOI, then PMC, then PubMed.
fn parse_research(stdout: &str, query: &str) -> Result<Vec<SearchHit>, AdapterError> {
    let value = as_json(stdout, query)?;
    let results = value
        .get("results")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    Ok(results
        .iter()
        .map(|paper| SearchHit {
            url: paper_url(paper),
            title: text_at(paper, "title"),
            snippet: text_at(paper, "abstract"),
        })
        .collect())
}

fn paper_url(paper: &Value) -> String {
    let first = |key: &str| {
        paper
            .get("ids")
            .and_then(|ids| ids.get(key))
            .and_then(Value::as_array)
            .and_then(|list| list.first())
            .and_then(Value::as_str)
            .map(str::to_string)
    };
    if let Some(doi) = first("doi") {
        return format!("https://doi.org/{doi}");
    }
    if let Some(pmcid) = first("pmcid") {
        return format!("https://pmc.ncbi.nlm.nih.gov/articles/{pmcid}/");
    }
    if let Some(pmid) = first("pmid") {
        return format!("https://pubmed.ncbi.nlm.nih.gov/{pmid}/");
    }
    text_at(paper, "primaryId")
}

fn write_cached(path: &Path, raw: &[u8]) -> Result<(), AdapterError> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)
            .map_err(|err| AdapterError::Command(format!("{}: {err}", dir.display())))?;
    }
    std::fs::write(path, raw)
        .map_err(|err| AdapterError::Command(format!("{}: {err}", path.display())))
}

/// One plain GET. The fetch stage takes its raw checksum from these bytes,
/// which are the response bytes.
pub fn fetch_raw(url: &str) -> Result<(String, String, Vec<u8>), AdapterError> {
    let response = match ureq::get(url).call() {
        Ok(response) => response,
        Err(ureq::Error::Status(status, _)) if status == 401 || status == 403 => {
            return Err(AdapterError::Unauthenticated(format!(
                "{url}: HTTP {status}"
            )))
        }
        Err(err) => {
            return Err(AdapterError::Failed {
                url: url.to_string(),
                error: err.to_string(),
            })
        }
    };
    let final_url = response.get_url().to_string();
    let content_type = response
        .header("content-type")
        .unwrap_or_default()
        .to_string();
    let mut body = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut body)
        .map_err(|err| AdapterError::Failed {
            url: url.to_string(),
            error: err.to_string(),
        })?;
    Ok((final_url, content_type, body))
}
