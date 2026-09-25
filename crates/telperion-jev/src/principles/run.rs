//! One reviewer run: at most two batched calls through the shared caller,
//! each answer cached per candidate under its full input and every version
//! it depends on, inside a deadline. Any failure makes the run incomplete;
//! nothing here ever reports clean on a call that did not answer.

use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::{json, Map, Value};

use crate::caller::{evaluate, EvaluateRequest, HttpRequest, HttpResponse, Transport, MODEL};

use super::ask::{self, QUESTIONS_VERSION};
use super::candidates::{Candidate, Extraction, EXTRACTOR_VERSION, MAX_INPUT_TOKENS};
use super::policy::{Exception, Policy, EXCEPTIONS_JSON};
use super::review::{decide, select, Outcome};

pub const DEADLINE: Duration = Duration::from_secs(10);

#[derive(Debug, Default)]
pub struct Run {
    pub outcome: Outcome,
    /// The answers, keyed as the questions were: recorded for replay.
    pub phase_one: Value,
    pub phase_two: Value,
    pub calls: u32,
    pub cached: usize,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub elapsed_ms: u64,
    pub ledger: Vec<String>,
}

pub struct Settings<'a> {
    /// Loads the key only when a call is needed: a fully cached run never
    /// starts the interactive shell.
    pub key: &'a dyn Fn() -> Option<String>,
    pub ledger: &'a Path,
    pub cache: Option<&'a Path>,
    /// Ask phase two about every non-`none` mechanism, whatever its cut,
    /// so an evaluation can choose cuts from the recorded answers.
    pub record_all: bool,
    pub deadline: Instant,
    /// Phase-one answers already recorded for this extraction: an
    /// evaluation that changes only phase two re-asks only phase two.
    pub phase_one: Option<Value>,
}

pub fn review(transport: &dyn Transport, s: &Settings, x: &Extraction, policy: &Policy, exceptions: &[Exception]) -> Run {
    let started = Instant::now();
    let mut run = Run { phase_one: json!({}), phase_two: json!({}), ..Run::default() };
    let all: Vec<&Candidate> = x.candidates.iter().collect();
    let misses = match &s.phase_one {
        Some(recorded) => {
            run.phase_one = recorded.clone();
            Vec::new()
        }
        None => fill(&mut run.phase_one, s.cache, policy, &all, "p1"),
    };
    run.cached += all.len() - misses.len();
    if !misses.is_empty() {
        let (state, questions) = ask::phase_one(policy, &misses);
        if let Err(e) = call(transport, s, &mut run, "principles-p1", &state, &questions, &misses, policy, "p1") {
            return incomplete(run, started, e);
        }
    }
    let (picked, mut abstained) = select(x, &run.phase_one, policy, !s.record_all);
    let chosen: Vec<&Candidate> = picked.iter().map(|p| p.candidate).collect();
    let misses: Vec<&Candidate> = fill(&mut run.phase_two, s.cache, policy, &chosen, "p2");
    let mut short = None;
    if !misses.is_empty() {
        let mut pairs: Vec<(&Candidate, &str, &str, f64)> = picked
            .iter()
            .filter(|p| misses.iter().any(|m| m.id == p.candidate.id))
            .map(|p| (p.candidate, p.mechanism.as_str(), p.evidence.as_str(), p.probability))
            .collect();
        pairs.sort_by(|a, b| b.3.total_cmp(&a.3));
        // The surest selections are confirmed first; those past the token
        // budget are left unasked and the run cannot say clean.
        let mut asked: Vec<(&Candidate, &str, &str)> = Vec::new();
        for (c, m, e, _) in &pairs {
            asked.push((c, m, e));
            let (state, questions) = ask::phase_two(policy, exceptions, &asked);
            // Three bytes a token: measured runs tokenise denser than four.
            let estimate = (state.to_string().len() + questions.to_string().len()) as u64 / 3;
            if !s.record_all && run.input_tokens + estimate > MAX_INPUT_TOKENS as u64 {
                asked.pop();
                short = Some(format!("token budget: {} of {} selections left unconfirmed", pairs.len() - asked.len(), pairs.len()));
                break;
            }
        }
        if !asked.is_empty() {
            let (state, questions) = ask::phase_two(policy, exceptions, &asked);
            let sent: Vec<&Candidate> = asked.iter().map(|a| a.0).collect();
            if let Err(e) = call(transport, s, &mut run, "principles-p2", &state, &questions, &sent, policy, "p2") {
                return incomplete(run, started, e);
            }
        }
    }
    let mut outcome = decide(x, &picked, &run.phase_two, policy);
    abstained.append(&mut outcome.abstained);
    outcome.abstained = abstained;
    outcome.incomplete = outcome.incomplete.or(short);
    run.outcome = outcome;
    run.elapsed_ms = started.elapsed().as_millis() as u64;
    run
}

fn incomplete(mut run: Run, started: Instant, reason: String) -> Run {
    run.outcome.incomplete = Some(reason);
    run.elapsed_ms = started.elapsed().as_millis() as u64;
    run
}

#[allow(clippy::too_many_arguments)]
fn call(
    transport: &dyn Transport,
    s: &Settings,
    run: &mut Run,
    tool: &str,
    state: &Value,
    questions: &Value,
    asked: &[&Candidate],
    policy: &Policy,
    phase: &str,
) -> Result<(), String> {
    if Instant::now() >= s.deadline {
        return Err("deadline reached before the call".into());
    }
    let key = (s.key)().ok_or("no TypeSafe key in the interactive shell (~/.bashrc)")?;
    run.calls += 1;
    let entry = evaluate(
        transport,
        &key,
        EvaluateRequest { tool, source: None, state, questions, ledger_dir: s.ledger },
    )
    .map_err(|e| format!("{tool}: {e}"))?;
    run.ledger.push(entry.reference());
    if let Some(u) = &entry.usage {
        run.input_tokens += u.input_tokens;
        run.output_tokens += u.output_tokens;
    }
    let target = if phase == "p1" { &mut run.phase_one } else { &mut run.phase_two };
    if let (Some(map), Some(answers)) = (target.as_object_mut(), entry.answers.as_object()) {
        for (k, v) in answers {
            map.insert(k.clone(), v.clone());
        }
    }
    if let Some(dir) = s.cache {
        for c in asked {
            store(dir, &cache_key(c, policy, phase), &own_answers(target, &c.id));
        }
    }
    Ok(())
}

/// Copies cached answers into `into` and returns the candidates missing.
fn fill<'a>(into: &mut Value, cache: Option<&Path>, policy: &Policy, cs: &[&'a Candidate], phase: &str) -> Vec<&'a Candidate> {
    let mut misses = Vec::new();
    for c in cs {
        let hit = cache.and_then(|dir| std::fs::read(dir.join(cache_key(c, policy, phase))).ok());
        let hit = hit.and_then(|b| serde_json::from_slice::<Map<String, Value>>(&b).ok());
        match (hit, into.as_object_mut()) {
            (Some(answers), Some(map)) => {
                for (suffix, v) in answers {
                    map.insert(format!("{}_{suffix}", c.id), v);
                }
            }
            _ => misses.push(*c),
        }
    }
    misses
}

fn own_answers(all: &Value, id: &str) -> Value {
    let prefix = format!("{id}_");
    let mut out = Map::new();
    for (k, v) in all.as_object().into_iter().flatten() {
        if let Some(suffix) = k.strip_prefix(&prefix) {
            out.insert(suffix.to_string(), v.clone());
        }
    }
    Value::Object(out)
}

fn store(dir: &Path, name: &str, answers: &Value) {
    let _ = std::fs::create_dir_all(dir);
    let _ = std::fs::write(dir.join(name), serde_json::to_vec(answers).unwrap_or_default());
}

/// The candidate's full input with its id cleared, and every version the
/// answer depends on: extractor, questions, model, policy (which holds the
/// cuts and modes) and the exception registry.
pub fn cache_key(c: &Candidate, policy: &Policy, phase: &str) -> String {
    let mut c = c.clone();
    c.id.clear();
    let input = serde_json::to_string(&c).unwrap_or_default();
    let versions = format!(
        "{phase}\n{EXTRACTOR_VERSION}\n{QUESTIONS_VERSION}\n{MODEL}\n{}\n{}",
        policy.version,
        crate::sha256_hex(EXCEPTIONS_JSON.as_bytes())
    );
    format!("{}.json", crate::sha256_hex(format!("{versions}\n{input}").as_bytes()))
}

/// The live transport under a deadline: each request may take only the
/// time left, and a rate limit with too little time left fails at once
/// instead of sleeping past the deadline.
pub struct Deadline {
    pub until: Instant,
}

impl Transport for Deadline {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let left = self.until.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err("deadline".into());
        }
        let agent = ureq::AgentBuilder::new().timeout(left).build();
        let mut req = agent.request(request.method, &request.url);
        for (k, v) in &request.headers {
            req = req.set(k, v);
        }
        let result = match &request.body {
            Some(body) => req.send_bytes(body),
            None => req.call(),
        };
        let response = match result {
            Ok(r) | Err(ureq::Error::Status(_, r)) => r,
            Err(e) => return Err(e.to_string()),
        };
        let status = response.status();
        if (status == 429 || status == 529) && self.until.saturating_duration_since(Instant::now()) < Duration::from_secs(4) {
            return Err(format!("HTTP {status} with too little time left to retry"));
        }
        let mut body = Vec::new();
        std::io::Read::read_to_end(&mut response.into_reader(), &mut body).map_err(|e| e.to_string())?;
        Ok(HttpResponse { status, body })
    }
}
