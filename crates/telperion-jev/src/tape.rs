//! Record and replay (owner, 2026-09-25): a run's every external answer,
//! kept so the run can be served again offline.
//!
//! `species <id> --record <dir>` stores each response a run receives, keyed
//! by a stable hash of its request: Firecrawl searches and scrapes, Jev
//! calls, the Wikimedia Commons API and image bytes, and every vision
//! adapter's reply (`scripts/tape-adapter.py`, which wraps the adapter
//! program). `--replay <dir>` serves them back with no network and fails,
//! naming the request, on any the recording lacks. This is one layer at the
//! boundary: each stage takes its adapter, transport, web client and adapter
//! programs through the wrappers here, so there is no second path through
//! the stages. The mode is the process's (`TELPERION_TAPE`), set once by the
//! binary and inherited by the adapter programs it starts.
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};

use crate::caller::{HttpRequest, HttpResponse, Transport};
use crate::pipeline::adapter::{AdapterError, FetchAdapter, Scrape, SearchHit, Spent};
use crate::pipeline::canon::{canonical_sha256, read_json, write_atomic, write_canonical};
use crate::pipeline::photos::Web;

/// The environment variable that carries the mode: `record:<dir>` or
/// `replay:<dir>`.
pub const VAR: &str = "TELPERION_TAPE";
/// The adapter program's wrapper.
pub const ADAPTER: &str = "scripts/tape-adapter.py";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tape {
    Record(PathBuf),
    Replay(PathBuf),
}

impl Tape {
    /// The process's tape, if the run records or replays.
    pub fn from_env() -> Option<Self> {
        Self::parse(&std::env::var(VAR).ok()?)
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.split_once(':')? {
            ("record", dir) => Some(Self::Record(dir.into())),
            ("replay", dir) => Some(Self::Replay(dir.into())),
            _ => None,
        }
    }

    pub fn value(&self) -> String {
        match self {
            Self::Record(dir) => format!("record:{}", dir.display()),
            Self::Replay(dir) => format!("replay:{}", dir.display()),
        }
    }

    fn dir(&self) -> &Path {
        match self {
            Self::Record(dir) | Self::Replay(dir) => dir,
        }
    }

    /// Where the answer to `request` of `kind` lives. The key ignores every
    /// `path`, so a replay from another directory finds the same answer.
    fn entry(&self, kind: &str, request: &Value) -> (String, PathBuf) {
        let key = canonical_sha256(&json!({"kind": kind, "request": stable(request)}));
        let path = self.dir().join(kind).join(format!("{}.json", &key[..32]));
        (key, path)
    }

    /// Serves `request` from the recording, or runs `live` and records its
    /// answer. A response's bytes are kept beside it, never inside the JSON.
    pub fn serve(
        &self,
        kind: &str,
        request: &Value,
        live: impl FnOnce() -> Result<(Value, Vec<u8>), String>,
    ) -> Result<(Value, Vec<u8>), String> {
        let (key, path) = self.entry(kind, request);
        let blob = path.with_extension("bin");
        if let Self::Replay(dir) = self {
            let recorded = read_json(&path).map_err(|_| {
                format!(
                    "replay: {} holds no {kind} answer for {request} (key {key})",
                    dir.display()
                )
            })?;
            let bytes = std::fs::read(&blob).unwrap_or_default();
            return Ok((recorded["response"].clone(), bytes));
        }
        let (response, bytes) = live()?;
        let entry = json!({"kind": kind, "key": key, "request": request, "response": response});
        write_canonical(&path, &entry).map_err(|e| e.to_string())?;
        if !bytes.is_empty() {
            write_atomic(&blob, &bytes).map_err(|e| e.to_string())?;
        }
        Ok((response, bytes))
    }
}

/// `value` without any `path` key, at any depth.
fn stable(value: &Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .filter(|(k, _)| k.as_str() != "path")
                .map(|(k, v)| (k.clone(), stable(v)))
                .collect::<Map<_, _>>(),
        ),
        Value::Array(list) => Value::Array(list.iter().map(stable).collect()),
        other => other.clone(),
    }
}

/// The fetch adapter through the tape, when the run has one.
pub fn fetch(inner: Box<dyn FetchAdapter>) -> Box<dyn FetchAdapter> {
    match Tape::from_env() {
        Some(tape) => Box::new(Fetch { inner, tape }),
        None => inner,
    }
}

pub struct Fetch {
    pub inner: Box<dyn FetchAdapter>,
    pub tape: Tape,
}

fn adapter_error(err: String) -> AdapterError {
    AdapterError::Command(err)
}

impl Fetch {
    fn hits(&self, op: &str, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        let request = json!({"op": op, "query": query, "limit": limit});
        let (response, _) = self
            .tape
            .serve("firecrawl", &request, || {
                let hits = match op {
                    "search" => self.inner.search(query, limit),
                    _ => self.inner.research(query, limit),
                };
                Ok((answer(hits.map(|h| json!(h))), Vec::new()))
            })
            .map_err(adapter_error)?;
        settled(&response)
            .and_then(|v| serde_json::from_value(v).map_err(|e| AdapterError::Parse(e.to_string())))
    }
}

/// A call's outcome as the tape keeps it: its value or its error.
fn answer(result: Result<Value, AdapterError>) -> Value {
    match result {
        Ok(value) => json!({"ok": value}),
        Err(err) => json!({"err": err}),
    }
}

fn settled(response: &Value) -> Result<Value, AdapterError> {
    match response.get("ok") {
        Some(value) => Ok(value.clone()),
        None => Err(serde_json::from_value(response["err"].clone())
            .unwrap_or_else(|_| AdapterError::Command(response["err"].to_string()))),
    }
}

impl FetchAdapter for Fetch {
    fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        self.hits("search", query, limit)
    }
    fn research(&self, query: &str, limit: usize) -> Result<Vec<SearchHit>, AdapterError> {
        self.hits("research", query, limit)
    }
    fn scrape(&self, url: &str) -> Result<Scrape, AdapterError> {
        let request = json!({"op": "scrape", "url": url});
        let (response, raw) = self
            .tape
            .serve("firecrawl", &request, || match self.inner.scrape(url) {
                Ok(s) => Ok((
                    json!({"ok": {"final_url": s.final_url, "content_type": s.content_type, "markdown": s.markdown}}),
                    s.raw,
                )),
                Err(err) => Ok((answer(Err(err)), Vec::new())),
            })
            .map_err(adapter_error)?;
        let page = settled(&response)?;
        let text = |key: &str| page[key].as_str().unwrap_or_default().to_string();
        Ok(Scrape {
            final_url: text("final_url"),
            content_type: text("content_type"),
            raw,
            markdown: text("markdown"),
        })
    }
    fn parse_pdf(&self, path: &Path) -> Result<String, AdapterError> {
        // A cached PDF is named by its bytes, so its name is the request.
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned());
        let request = json!({"op": "parse", "file": name});
        let (response, _) = self
            .tape
            .serve("firecrawl", &request, || {
                Ok((
                    answer(self.inner.parse_pdf(path).map(|m| json!(m))),
                    Vec::new(),
                ))
            })
            .map_err(adapter_error)?;
        Ok(settled(&response)?.as_str().unwrap_or_default().to_string())
    }
    fn spent(&self) -> Spent {
        self.inner.spent()
    }
}

/// Jev's transport through the tape. The key is never part of a request.
pub struct Jev<'a> {
    pub inner: &'a dyn Transport,
    pub tape: Option<Tape>,
}

impl<'a> Jev<'a> {
    pub fn new(inner: &'a dyn Transport) -> Self {
        Self {
            inner,
            tape: Tape::from_env(),
        }
    }
}

impl Transport for Jev<'_> {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let Some(tape) = &self.tape else {
            return self.inner.send(request);
        };
        let body: Value = request
            .body
            .as_deref()
            .and_then(|b| serde_json::from_slice(b).ok())
            .unwrap_or(Value::Null);
        let asked = json!({"method": request.method, "url": request.url, "body": body});
        let (response, bytes) = tape.serve("jev", &asked, || {
            let sent = self.inner.send(request)?;
            Ok((json!({"status": sent.status}), sent.body))
        })?;
        Ok(HttpResponse {
            status: response["status"].as_u64().unwrap_or(0) as u16,
            body: bytes,
        })
    }
}

/// The key a Jev call carries: none is needed to replay.
pub fn key(live: impl FnOnce() -> Result<String, String>) -> Result<String, String> {
    match Tape::from_env() {
        Some(Tape::Replay(_)) => Ok("replay".into()),
        _ => live(),
    }
}

/// The web client for photographs through the tape.
pub struct Photos<'a> {
    pub inner: &'a dyn Web,
    pub tape: Option<Tape>,
}

impl Web for Photos<'_> {
    fn get(&self, url: &str) -> Result<(Vec<u8>, String), String> {
        let Some(tape) = &self.tape else {
            return self.inner.get(url);
        };
        let (response, bytes) = tape.serve("web", &json!({"url": url}), || {
            let result = self.inner.get(url);
            Ok(match result {
                Ok((bytes, kind)) => (json!({"ok": kind}), bytes),
                Err(err) => (json!({"err": err}), Vec::new()),
            })
        })?;
        match response.get("ok") {
            Some(kind) => Ok((bytes, kind.as_str().unwrap_or_default().to_string())),
            None => Err(response["err"]
                .as_str()
                .unwrap_or("recorded error")
                .to_string()),
        }
    }
}

/// `config` with every adapter program (an object holding `program` and
/// `args`) wrapped by the tape's adapter script, when the run has a tape.
pub fn adapters(config: &mut Value) {
    let Some(tape) = Tape::from_env() else {
        return;
    };
    wrap(config, &tape);
}

pub fn wrap(value: &mut Value, tape: &Tape) {
    match value {
        Value::Object(map) if map.contains_key("program") && map.contains_key("args") => {
            let mut args = vec![
                json!(ADAPTER),
                json!(tape.value()),
                json!("--"),
                map["program"].clone(),
            ];
            args.extend(map["args"].as_array().cloned().unwrap_or_default());
            map.insert("program".into(), json!("python3"));
            map.insert("args".into(), json!(args));
        }
        Value::Object(map) => map.values_mut().for_each(|v| wrap(v, tape)),
        Value::Array(list) => list.iter_mut().for_each(|v| wrap(v, tape)),
        _ => {}
    }
}
