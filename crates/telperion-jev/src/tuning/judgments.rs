use super::{calibration::Manifest, engine::Run, live::Validation};
use serde_json::{json, Value};
pub fn summary(state: &Run) -> Value {
    let recent=state.trials.iter().rev().take(5).map(|t|json!({"label":t.label,"feasible":t.feasible,
        "score":t.score,"reason":t.reason,"views":t.comparisons.iter().map(|c|json!({"reference":c.reference,
        "target":c.target,"observed":c.observed})).collect::<Vec<_>>()})).collect::<Vec<_>>();
    json!({"owner_notes":state.owner_notes,"visual":state.visual,"recent_attempts":recent,
        "dials":state.dials.iter().map(|d|json!({"id":d.id,"meaning":d.meaning,"current":state.effective.pointer(&d.path)})).collect::<Vec<_>>()})
}
pub fn proposals(state: &Run) -> Result<Value, String> {
    let mut questions = serde_json::Map::new();
    for dial in &state.dials {
        let current = state
            .effective
            .pointer(&dial.path)
            .and_then(Value::as_f64)
            .ok_or("missing dial")?;
        questions.insert(dial.id.clone(), dial.question(current)?);
    }
    Ok(Value::Object(questions))
}
pub fn routes(gaps: &std::collections::BTreeMap<String, String>) -> Value {
    let mut q = json!({"route":{"type":"choice","instructions":"Route remaining visible defects using authored dial meanings and prior outcomes. Numeric stall alone proves no generator gap. Choose an existing spec only when its stated scope matches the defect.",
        "criteria":{"tuning":"Existing authored dials can address the defect with a supported adjustment.",
            "new_capability":"A new capability investigation is required.","appearance":"An appearance issue outside these dials needs investigation.",
            "insufficient_evidence":"No supported diagnosis yet."}}});
    for (id, scope) in gaps {
        q["route"]["criteria"][format!("existing:{id}")] = json!(scope);
    }
    q
}
pub fn threshold(v: &Validation) -> Result<f64, String> {
    let m: Manifest =
        serde_json::from_slice(&std::fs::read(&v.manifest).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    Ok(m.min_confidence)
}
pub fn allowance(state: &Value, questions: &Value) -> u64 {
    (serde_json::to_vec(state).unwrap().len() + serde_json::to_vec(questions).unwrap().len()) as u64
        + 1024
}
