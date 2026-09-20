use super::{calibration::Manifest, engine::Run, live::Validation};
use serde_json::{json, Value};
pub fn summary(state: &Run) -> Value {
    let recent=state.trials.iter().rev().take(5).map(|t|json!({"label":t.label,"identity":t.identity,"current_revision":t.identity==state.identity,"feasible":t.feasible,
        "resource_feasibility":{"nodes":t.measurement["metrics"]["nodes"],"growth":t.measurement["metrics"]["growth"]},
        "score":t.score,"reason":t.reason,"views":t.comparisons.iter().map(|c|json!({"reference":c.reference,
        "target":c.target,"observed":c.observed})).collect::<Vec<_>>()})).collect::<Vec<_>>();
    let nodes = state
        .current
        .and_then(|i| state.trials.get(i))
        .and_then(|t| t.measurement["metrics"]["nodes"]["value"].as_u64());
    let cap = state
        .effective
        .pointer("/skeleton/growth/maxNodes")
        .and_then(Value::as_u64);
    let amendments = state.authorizations.iter().rev().filter_map(|a|a.baseline_amendment.as_ref().map(|m|json!({
        "previous_identity":a.identity,"next_identity":a.next_identity,"previous_overlay":m.previous,"next_overlay":m.next,
        "prior_effective_cap":"not recorded; omitted overlay values are not historical effective values",
        "authorization_rationale":a.rationale,"experimental_reason":a.experimental_pilot.as_ref().map(|p| &p.reason)
    }))).take(3).collect::<Vec<_>>();
    json!({"current_identity":state.identity,"resource_amendments":amendments,"resource_limit":{"meaning":"computational feasibility, not botanical character","max_nodes":cap,"current_nodes":nodes,"remaining_nodes":cap.zip(nodes).map(|(c,n)|c.saturating_sub(n))},"owner_notes":state.owner_notes,"visual":state.visual,"recent_attempts":recent,
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
