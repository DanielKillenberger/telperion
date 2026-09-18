//! One caller: endpoint, interactive-shell key, backoff, ledger write.

use std::io::Read;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

use crate::ledger::{derived_identity, new_entry_id, write_entry, LedgerEntry, SourceRef, Usage};
use crate::sha256_hex;

/// TypeSafe evaluation endpoint. Named here so the isolation guard can find it.
pub const ENDPOINT: &str = "https://api.typesafe.ai/v1/systemone";
pub const MODEL: &str = "jev-latest";
pub const INTERACTIVE_SHELL: &str = "~/.bashrc";
const MAX_ATTEMPTS: u32 = 4;

#[derive(Debug)]
pub enum KeyError {
    Missing,
}

impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TYPESAFE_API_KEY is unset. Export it in the interactive shell ({INTERACTIVE_SHELL}, below the non-interactive guard) and call through bash -ic."
        )
    }
}

impl std::error::Error for KeyError {}

#[derive(Debug)]
pub enum CallerError {
    Key(KeyError),
    Transport(String),
    Http { status: u16, body: String },
    Json(String),
}

impl std::fmt::Display for CallerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(err) => write!(f, "{err}"),
            Self::Transport(msg) => write!(f, "transport: {msg}"),
            Self::Http { status, body } => write!(f, "HTTP {status}: {body}"),
            Self::Json(msg) => write!(f, "response json: {msg}"),
        }
    }
}

impl std::error::Error for CallerError {}

/// HTTP request the caller sends. The authorization header is attached by
/// `evaluate` and never written to the ledger.
pub struct HttpRequest {
    pub method: &'static str,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

pub trait Transport {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String>;
}

/// Live ureq transport. Used only from the `jev` binary.
pub struct UreqTransport;

impl Transport for UreqTransport {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let mut req = ureq::request(request.method, &request.url);
        for (key, value) in &request.headers {
            req = req.set(key, value);
        }
        let result = match &request.body {
            Some(body) => req.send_bytes(body),
            None => req.call(),
        };
        match result {
            Ok(resp) => read_response(resp),
            Err(ureq::Error::Status(_, resp)) => read_response(resp),
            Err(err) => Err(err.to_string()),
        }
    }
}

fn read_response(resp: ureq::Response) -> Result<HttpResponse, String> {
    let status = resp.status();
    let mut body = Vec::new();
    resp.into_reader()
        .read_to_end(&mut body)
        .map_err(|err| err.to_string())?;
    Ok(HttpResponse { status, body })
}

/// Reads the key from the process environment only. Tests use this path.
pub fn load_key_from_env() -> Result<String, KeyError> {
    match std::env::var("TYPESAFE_API_KEY") {
        Ok(key) if !key.is_empty() => Ok(key),
        _ => Err(KeyError::Missing),
    }
}

/// Reads the key from `~/.bashrc` via an interactive bash. Never prints it.
pub fn load_key_from_interactive_shell() -> Result<String, KeyError> {
    let output = std::process::Command::new("bash")
        .args(["-ic", r#"printf %s "$TYPESAFE_API_KEY""#])
        .output()
        .map_err(|_| KeyError::Missing)?;
    let key = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if key.is_empty() {
        Err(KeyError::Missing)
    } else {
        Ok(key)
    }
}

pub fn load_key() -> Result<String, KeyError> {
    load_key_from_env().or_else(|_| load_key_from_interactive_shell())
}

pub struct EvaluateRequest<'a> {
    pub tool: &'a str,
    pub source: Option<&'a SourceRef>,
    pub state: &'a Value,
    pub questions: &'a Value,
    pub ledger_dir: &'a Path,
}

/// Posts one evaluation. Retries 429 and 529 with exponential delay. Writes a
/// ledger entry on success and after a exhausted retry sequence.
pub fn evaluate(
    transport: &dyn Transport,
    key: &str,
    request: EvaluateRequest<'_>,
) -> Result<LedgerEntry, CallerError> {
    let payload = json!({
        "model": MODEL,
        "state": request.state,
        "questions": request.questions,
    });
    let body = serde_json::to_vec(&payload).expect("state serializes");
    let state_sha256 = sha256_hex(&serde_json::to_vec(request.state).expect("state serializes"));
    let started = Instant::now();
    let mut last_status = 0u16;
    let mut last_body = String::new();

    for attempt in 0..MAX_ATTEMPTS {
        let http = HttpRequest {
            method: "POST",
            url: ENDPOINT.to_string(),
            headers: vec![
                ("Authorization".into(), format!("Bearer {key}")),
                ("Content-Type".into(), "application/json".into()),
            ],
            body: Some(body.clone()),
        };
        let response = match transport.send(&http) {
            Ok(response) => response,
            Err(err) => {
                let entry = failure_entry(
                    &request,
                    &state_sha256,
                    started,
                    format!("transport: {err}"),
                );
                let _ = write_entry(request.ledger_dir, &entry);
                return Err(CallerError::Transport(err));
            }
        };
        last_status = response.status;
        last_body = String::from_utf8_lossy(&response.body)
            .chars()
            .take(800)
            .collect();
        if response.status == 429 || response.status == 529 {
            if attempt + 1 < MAX_ATTEMPTS {
                thread::sleep(Duration::from_secs(1 << attempt));
                continue;
            }
            break;
        }
        if response.status >= 400 {
            let entry = failure_entry(
                &request,
                &state_sha256,
                started,
                format!("HTTP {last_status}: {last_body}"),
            );
            let _ = write_entry(request.ledger_dir, &entry);
            return Err(CallerError::Http {
                status: response.status,
                body: last_body,
            });
        }
        let parsed: Value = match serde_json::from_slice(&response.body) {
            Ok(value) => value,
            Err(err) => {
                let entry = failure_entry(
                    &request,
                    &state_sha256,
                    started,
                    format!("response json: {err}"),
                );
                let _ = write_entry(request.ledger_dir, &entry);
                return Err(CallerError::Json(err.to_string()));
            }
        };
        let entry = success_entry(&request, &state_sha256, started, parsed);
        write_entry(request.ledger_dir, &entry).map_err(CallerError::Transport)?;
        return Ok(entry);
    }

    let entry = failure_entry(
        &request,
        &state_sha256,
        started,
        format!("HTTP {last_status}: {last_body}"),
    );
    let _ = write_entry(request.ledger_dir, &entry);
    Err(CallerError::Http {
        status: last_status,
        body: last_body,
    })
}

fn success_entry(
    request: &EvaluateRequest<'_>,
    state_sha256: &str,
    started: Instant,
    parsed: Value,
) -> LedgerEntry {
    let usage = parsed.get("usage").and_then(|value| {
        Some(Usage {
            input_tokens: value.get("input_tokens")?.as_u64()?,
            output_tokens: value.get("output_tokens")?.as_u64()?,
        })
    });
    let identity = derived_identity(state_sha256, request.questions, request_model(&parsed));
    LedgerEntry {
        id: new_entry_id(),
        tool: request.tool.to_string(),
        state_sha256: state_sha256.to_string(),
        source: request.source.cloned(),
        model: parsed
            .get("model")
            .and_then(Value::as_str)
            .unwrap_or(MODEL)
            .to_string(),
        questions: request.questions.clone(),
        answers: parsed.get("answers").cloned().unwrap_or(Value::Null),
        usage,
        elapsed_ms: started.elapsed().as_millis() as u64,
        recorded_at: now_rfc3339(),
        error: None,
        identity,
    }
}

fn failure_entry(
    request: &EvaluateRequest<'_>,
    state_sha256: &str,
    started: Instant,
    error: String,
) -> LedgerEntry {
    LedgerEntry {
        id: new_entry_id(),
        tool: request.tool.to_string(),
        state_sha256: state_sha256.to_string(),
        source: request.source.cloned(),
        model: MODEL.to_string(),
        questions: request.questions.clone(),
        answers: Value::Null,
        usage: None,
        elapsed_ms: started.elapsed().as_millis() as u64,
        recorded_at: now_rfc3339(),
        error: Some(error),
        identity: derived_identity(state_sha256, request.questions, MODEL),
    }
}

fn request_model(parsed: &Value) -> &str {
    parsed.get("model").and_then(Value::as_str).unwrap_or(MODEL)
}

pub(crate) fn now_rfc3339() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = secs / 86400;
    let tod = secs % 86400;
    let (year, month, day) = civil_from_days(days as i64);
    let hour = tod / 3600;
    let minute = (tod % 3600) / 60;
    let second = tod % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Howard Hinnant's days-from-civil inverse, UTC.
fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    (year as i32, m as u32, d as u32)
}

#[cfg(test)]
mod date_tests {
    use super::civil_from_days;

    #[test]
    fn epoch_is_1970_01_01() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }

    #[test]
    fn known_day() {
        // 2026-09-16 is 20_712 days after 1970-01-01.
        assert_eq!(civil_from_days(20_712), (2026, 9, 16));
    }
}
