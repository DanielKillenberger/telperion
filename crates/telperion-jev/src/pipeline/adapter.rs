//! Fetch adapter contract (fn-58 R2).
//!
//! One contract for discovery and fetch. Every returned value is plain data:
//! a hit list, the bytes and the markdown of one response, or the markdown of
//! one local file. No adapter option that asks a model to answer or extract is
//! reachable from here, so no model writes a number into the pipeline.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::sha256_hex;

pub mod firecrawl;
pub mod tables;

pub use firecrawl::{fetch_raw, FirecrawlCli, RawSource};
pub use tables::{age_indexed_rows, coverage, markdown_tables, table_rows_for, AgeRow, Coverage};

/// One candidate source from discovery.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SearchHit {
    pub url: String,
    pub title: String,
    pub snippet: String,
}

/// One fetched response. `raw` is the response body the fetch stage
/// checksums; `markdown` is the transformation the stage judges.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Scrape {
    pub final_url: String,
    pub content_type: String,
    pub raw: Vec<u8>,
    pub markdown: String,
}

/// Every way a fetch fails. `Failed.url` carries the query for `search` and
/// `research`, which have no URL of their own.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub enum AdapterError {
    Unauthenticated(String),
    Failed { url: String, error: String },
    Command(String),
    Parse(String),
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthenticated(msg) => write!(f, "unauthenticated: {msg}"),
            Self::Failed { url, error } => write!(f, "fetch failed for {url}: {error}"),
            Self::Command(msg) => write!(f, "command: {msg}"),
            Self::Parse(msg) => write!(f, "adapter output: {msg}"),
        }
    }
}

impl std::error::Error for AdapterError {}

/// The one contract the discovery and fetch stages call.
pub trait FetchAdapter {
    fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError>;
    /// The research paper index, separate from web search.
    fn research(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError>;
    fn scrape(&self, url: &str) -> Result<Scrape, AdapterError>;
    /// Markdown of a local PDF already on disk.
    fn parse_pdf(&self, path: &Path) -> Result<String, AdapterError>;
}

/// What the fetch stage records for one admitted source. Raw and markdown
/// carry separate checksums because the markdown is a transformation that can
/// differ between runs. Fields are declared in sorted order so the serialized
/// record is canonical.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FetchRecord {
    pub content_type: String,
    pub final_url: String,
    pub markdown_bytes: u64,
    pub markdown_sha256: String,
    pub raw_bytes: u64,
    pub raw_sha256: String,
}

pub fn checksums(scrape: &Scrape) -> FetchRecord {
    FetchRecord {
        content_type: scrape.content_type.clone(),
        final_url: scrape.final_url.clone(),
        markdown_bytes: scrape.markdown.len() as u64,
        markdown_sha256: sha256_hex(scrape.markdown.as_bytes()),
        raw_bytes: scrape.raw.len() as u64,
        raw_sha256: sha256_hex(&scrape.raw),
    }
}

/// True when the response is a PDF. The content type decides; a URL path
/// ending in `.pdf` decides when the content type is absent or generic.
pub fn is_pdf(content_type: &str, url: &str) -> bool {
    let kind = content_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    if kind == "application/pdf" || kind == "application/x-pdf" {
        return true;
    }
    if !kind.is_empty() && kind != "application/octet-stream" {
        return false;
    }
    let path = url
        .split(['?', '#'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    path.ends_with(".pdf")
}

/// An adapter over files on disk. The model-swap test and every workspace
/// test use it, so those runs reach no network.
pub struct FixtureAdapter {
    pub dir: PathBuf,
}

#[derive(Deserialize, Default)]
struct FixtureIndex {
    #[serde(default)]
    scrape: BTreeMap<String, FixtureScrape>,
    #[serde(default)]
    search: BTreeMap<String, Vec<SearchHit>>,
    #[serde(default)]
    research: BTreeMap<String, Vec<SearchHit>>,
    #[serde(default)]
    parse: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct FixtureScrape {
    final_url: String,
    content_type: String,
    raw: String,
    markdown: String,
}

impl FixtureAdapter {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    fn index(&self) -> Result<FixtureIndex, AdapterError> {
        let path = self.dir.join("index.json");
        let text = fs::read_to_string(&path)
            .map_err(|err| AdapterError::Command(format!("{}: {err}", path.display())))?;
        serde_json::from_str(&text)
            .map_err(|err| AdapterError::Parse(format!("{}: {err}", path.display())))
    }

    fn file(&self, name: &str) -> Result<Vec<u8>, AdapterError> {
        let path = self.dir.join(name);
        fs::read(&path).map_err(|err| AdapterError::Failed {
            url: name.to_string(),
            error: format!("{}: {err}", path.display()),
        })
    }
}

fn unknown(key: &str) -> AdapterError {
    AdapterError::Failed {
        url: key.to_string(),
        error: "not in the fixture index".into(),
    }
}

impl FetchAdapter for FixtureAdapter {
    fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        let mut hits = self
            .index()?
            .search
            .remove(query)
            .ok_or_else(|| unknown(query))?;
        hits.truncate(limit);
        Ok(hits)
    }

    fn research(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        let mut hits = self
            .index()?
            .research
            .remove(query)
            .ok_or_else(|| unknown(query))?;
        hits.truncate(limit);
        Ok(hits)
    }

    fn scrape(&self, url: &str) -> Result<Scrape, AdapterError> {
        let entry = self
            .index()?
            .scrape
            .remove(url)
            .ok_or_else(|| unknown(url))?;
        let raw = self.file(&entry.raw)?;
        let markdown = self.file(&entry.markdown)?;
        Ok(Scrape {
            final_url: entry.final_url,
            content_type: entry.content_type,
            raw,
            markdown: String::from_utf8_lossy(&markdown).into_owned(),
        })
    }

    fn parse_pdf(&self, path: &Path) -> Result<String, AdapterError> {
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        let file = self
            .index()?
            .parse
            .remove(&name)
            .ok_or_else(|| unknown(&name))?;
        let markdown = self.file(&file)?;
        Ok(String::from_utf8_lossy(&markdown).into_owned())
    }
}
