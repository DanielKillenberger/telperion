use std::sync::Mutex;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::questions::{citation_cases, screen_cases, selection_cases, triage_cases};

pub struct CaseTransport;

impl Transport for CaseTransport {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        if request.method == "GET" {
            return Ok(HttpResponse {
                status: 200,
                body: b"Growth Rate: Moderate to fast. (may grow to 75 feet in 50 years).".to_vec(),
            });
        }
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}"))
            .map_err(|err| err.to_string())?;
        let answers = answers_for(&body);
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({
                "model": "jev-latest",
                "answers": answers,
                "usage": {"input_tokens": 8, "output_tokens": 3}
            }))
            .unwrap(),
        })
    }
}

fn answers_for(body: &Value) -> Value {
    let questions = &body["questions"];
    if questions.get("kind").is_some() {
        let sentence = body["state"]["candidate"]["sentence"]
            .as_str()
            .or_else(|| body["state"]["candidate"]["context"].as_str())
            .unwrap_or("");
        let kind = screen_cases()
            .into_iter()
            .find(|case| sentence.contains(&case.sentence) || case.sentence.contains(sentence))
            .map(|case| case.expect_kind)
            .or_else(|| {
                citation_cases()
                    .into_iter()
                    .find(|case| {
                        sentence.contains(&case.section) || case.section.contains(sentence)
                    })
                    .and_then(|case| case.expect_kind)
            })
            .unwrap_or_else(|| "not_about_tree_size".into());
        let anchor = if kind == "measured_size_at_age" {
            0.8
        } else {
            0.05
        };
        return json!({
            "kind": {
                "type": "choice",
                "choice": kind.clone(),
                "probabilities": { kind: 0.99 },
                "confidence": 0.98
            },
            "condition": {
                "type": "choice",
                "choice": "unstated",
                "probabilities": { "unstated": 0.9 },
                "confidence": 0.8
            },
            "anchor_usable": { "type": "noul", "noul": anchor }
        });
    }
    if questions.get("relation").is_some() {
        let claim = body["state"]["claim"].as_str().unwrap_or("");
        let case = citation_cases()
            .into_iter()
            .find(|case| case.claim == claim)
            .expect(claim);
        let confidence = case.expect_confidence.unwrap_or(0.99);
        return json!({
            "relation": {
                "type": "choice",
                "choice": case.expect_relation,
                "probabilities": { case.expect_relation: confidence },
                "confidence": confidence
            }
        });
    }
    if questions.get("span").is_some() {
        let question = body["state"]["question"].as_str().unwrap_or("");
        let chosen = selection_cases()
            .into_iter()
            .find(|case| case.question == question)
            .map(|case| case.expect_span)
            .unwrap_or_else(|| "none".into());
        return json!({
            "span": {
                "type": "choice",
                "choice": chosen,
                "probabilities": { chosen: 0.93 },
                "confidence": 0.9
            }
        });
    }
    if questions.get("spec").is_some() {
        let observation = body["state"]["owner_observation"].as_str().unwrap_or("");
        let cases = triage_cases();
        let choice = cases
            .routes
            .iter()
            .find(|case| case.observation == observation)
            .map(|route| route.expect[0].clone())
            .unwrap_or_else(|| "fn-31".into());
        let mut probabilities = serde_json::Map::new();
        probabilities.insert(choice.clone(), json!(0.86));
        probabilities.insert("new_spec".into(), json!(0.04));
        return json!({
            "spec": {
                "type": "choice",
                "choice": choice,
                "probabilities": probabilities,
                "confidence": 0.8
            }
        });
    }
    if questions.get("same_defect").is_some() {
        let prior = body["state"]["prior_finding"].as_str().unwrap_or("");
        let cases = triage_cases();
        let pair = cases
            .duplicates
            .iter()
            .find(|case| case.prior_finding == prior || prior.contains(&case.prior_finding));
        let p = if pair.map(|pair| pair.true_pair).unwrap_or(false) {
            0.8
        } else {
            0.12
        };
        return json!({ "same_defect": { "type": "noul", "noul": p } });
    }
    if questions.get("severity").is_some() {
        let observation = body["state"]["observation"].as_str().unwrap_or("");
        let cases = triage_cases();
        let level = cases
            .severity
            .iter()
            .find(|case| observation.contains(&case.id) || case.observation == observation)
            .map(|case| case.expect_level.as_str())
            .unwrap_or("noticeable");
        let score = match level {
            "cosmetic" => 0.01,
            "blocking" => 2.0,
            _ => 1.79,
        };
        return json!({
            "assessable": { "type": "noul", "noul": 0.95 },
            "severity": {
                "type": "score",
                "score": score,
                "confidence": 0.8,
                "probabilities": { "0": 0.05, "1": 0.1, "2": 0.85 }
            }
        });
    }
    json!({})
}

pub fn ledger_dir(tag: &str) -> std::path::PathBuf {
    static SEQ: Mutex<u64> = Mutex::new(0);
    let mut seq = SEQ.lock().unwrap();
    *seq += 1;
    let dir = std::env::temp_dir().join(format!("jev-tools-{tag}-{}-{}", std::process::id(), *seq));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
