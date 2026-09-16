use std::sync::Mutex;

use serde_json::json;
use telperion_jev::caller::{
    evaluate, CallerError, EvaluateRequest, HttpRequest, HttpResponse, KeyError, Transport,
    INTERACTIVE_SHELL,
};

struct Scripted {
    replies: Mutex<Vec<Result<HttpResponse, String>>>,
    seen: Mutex<Vec<HttpRequest>>,
}

impl Scripted {
    fn new(replies: Vec<Result<HttpResponse, String>>) -> Self {
        Self {
            replies: Mutex::new(replies),
            seen: Mutex::new(Vec::new()),
        }
    }
}

impl Transport for Scripted {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        self.seen.lock().unwrap().push(HttpRequest {
            method: request.method,
            url: request.url.clone(),
            headers: request.headers.clone(),
            body: request.body.clone(),
        });
        let mut replies = self.replies.lock().unwrap();
        if replies.is_empty() {
            return Err("no scripted reply".into());
        }
        replies.remove(0)
    }
}

fn ok_body() -> Vec<u8> {
    serde_json::to_vec(&json!({
        "model": "jev-latest",
        "answers": {
            "kind": {
                "type": "choice",
                "choice": "site_quality_criterion",
                "probabilities": {"site_quality_criterion": 0.99},
                "confidence": 0.98
            }
        },
        "usage": {"input_tokens": 12, "output_tokens": 4}
    }))
    .unwrap()
}

#[test]
fn missing_key_names_bashrc_and_never_prints_a_secret() {
    let text = KeyError::Missing.to_string();
    assert!(text.contains(INTERACTIVE_SHELL), "{text}");
    assert!(text.contains("non-interactive guard"), "{text}");
    assert!(!text.contains("Bearer"));
    assert!(!text.contains("export "));
}

#[test]
fn retries_429_then_records_success() {
    let transport = Scripted::new(vec![
        Ok(HttpResponse {
            status: 429,
            body: b"slow down".to_vec(),
        }),
        Ok(HttpResponse {
            status: 200,
            body: ok_body(),
        }),
    ]);
    let dir = tempfile();
    let state = json!({"n": 1});
    let questions =
        json!({"kind": {"type": "choice", "instructions": "x", "criteria": {"a": null}}});
    let entry = evaluate(
        &transport,
        "test-key-not-a-secret-for-display",
        EvaluateRequest {
            tool: "screen",
            source: None,
            state: &state,
            questions: &questions,
            ledger_dir: &dir,
        },
    )
    .expect("second attempt succeeds");
    assert_eq!(
        entry.choice("kind").as_deref(),
        Some("site_quality_criterion")
    );
    assert_eq!(transport.seen.lock().unwrap().len(), 2);
    let written = std::fs::read_dir(&dir).unwrap().count();
    assert_eq!(written, 1);
    let file = std::fs::read_dir(&dir).unwrap().next().unwrap().unwrap();
    let body = std::fs::read_to_string(file.path()).unwrap();
    assert!(!body.contains("test-key-not-a-secret-for-display"));
    assert!(!body.contains("Authorization"));
}

#[test]
fn exhausted_retries_record_failure_and_error() {
    let transport = Scripted::new(vec![
        Ok(HttpResponse {
            status: 529,
            body: b"overload".to_vec(),
        }),
        Ok(HttpResponse {
            status: 529,
            body: b"overload".to_vec(),
        }),
        Ok(HttpResponse {
            status: 529,
            body: b"overload".to_vec(),
        }),
        Ok(HttpResponse {
            status: 529,
            body: b"overload".to_vec(),
        }),
    ]);
    let dir = tempfile();
    let state = json!({"n": 1});
    let questions = json!({});
    let err = evaluate(
        &transport,
        "k",
        EvaluateRequest {
            tool: "screen",
            source: None,
            state: &state,
            questions: &questions,
            ledger_dir: &dir,
        },
    )
    .expect_err("gives up");
    match err {
        CallerError::Http { status, .. } => assert_eq!(status, 529),
        other => panic!("{other}"),
    }
    let file = std::fs::read_dir(&dir).unwrap().next().unwrap().unwrap();
    let body = std::fs::read_to_string(file.path()).unwrap();
    assert!(body.contains("HTTP 529"), "{body}");
}

#[test]
fn transport_error_writes_a_ledger_entry() {
    let transport = Scripted::new(vec![Err("connection refused".into())]);
    let dir = tempfile();
    let state = json!({"n": 1});
    let questions = json!({});
    let err = evaluate(
        &transport,
        "k",
        EvaluateRequest {
            tool: "screen",
            source: None,
            state: &state,
            questions: &questions,
            ledger_dir: &dir,
        },
    )
    .expect_err("transport fails");
    match err {
        CallerError::Transport(msg) => assert!(msg.contains("connection refused"), "{msg}"),
        other => panic!("{other}"),
    }
    let file = std::fs::read_dir(&dir).unwrap().next().unwrap().unwrap();
    let body = std::fs::read_to_string(file.path()).unwrap();
    assert!(body.contains("connection refused"), "{body}");
    assert!(body.contains("\"error\""), "{body}");
}

#[test]
fn malformed_json_records_a_failure_entry() {
    let transport = Scripted::new(vec![Ok(HttpResponse {
        status: 200,
        body: b"not-json".to_vec(),
    })]);
    let dir = tempfile();
    let state = json!({"n": 1});
    let questions = json!({});
    let err = evaluate(
        &transport,
        "k",
        EvaluateRequest {
            tool: "screen",
            source: None,
            state: &state,
            questions: &questions,
            ledger_dir: &dir,
        },
    )
    .expect_err("malformed json");
    match err {
        CallerError::Json(msg) => assert!(!msg.is_empty(), "{msg}"),
        other => panic!("{other}"),
    }
    let file = std::fs::read_dir(&dir).unwrap().next().unwrap().unwrap();
    let body = std::fs::read_to_string(file.path()).unwrap();
    assert!(body.contains("\"error\""), "{body}");
    assert!(body.contains("response json"), "{body}");
}

#[test]
fn same_state_writes_distinct_ids_and_files() {
    let transport = Scripted::new(vec![
        Ok(HttpResponse {
            status: 200,
            body: ok_body(),
        }),
        Ok(HttpResponse {
            status: 200,
            body: ok_body(),
        }),
    ]);
    let dir = tempfile();
    let state = json!({"n": 1});
    let questions = json!({});
    let first = evaluate(
        &transport,
        "k",
        EvaluateRequest {
            tool: "screen",
            source: None,
            state: &state,
            questions: &questions,
            ledger_dir: &dir,
        },
    )
    .unwrap();
    let second = evaluate(
        &transport,
        "k",
        EvaluateRequest {
            tool: "screen",
            source: None,
            state: &state,
            questions: &questions,
            ledger_dir: &dir,
        },
    )
    .unwrap();
    assert_ne!(first.id, second.id);
    assert_ne!(first.reference(), second.reference());
    assert!(first.reference().contains(&first.id));
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2);
}

fn tempfile() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-caller-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
