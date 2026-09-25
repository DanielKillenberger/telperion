//! What a resume does to a run's caps (fn-117).
//!
//! Only the caps the config sets are compared. A cap the config leaves out is
//! no cap, and the resumed run carries none; a cap it sets must equal the
//! run's, or be raised by an extension naming the exact old and new values.
use super::continuation::{HumanDecision, TokenCapExtension};
use super::state::Budget;
use serde_json::Value;
use std::fs;

/// One cap's resume: the extension that may raise it, then the config's word.
fn resume_cap(
    label: &str,
    extension: Option<&TokenCapExtension>,
    run: &mut Option<u64>,
    config: Option<u64>,
) -> Result<(), String> {
    if let Some(extension) = extension {
        if Some(extension.previous) != *run
            || Some(extension.next) != config
            || extension.next <= extension.previous
        {
            return Err(format!(
                "{label} extension must name exact previous and increased cap"
            ));
        }
        *run = config;
    }
    if config.is_none() {
        *run = None;
    }
    Ok(())
}

/// Historical runs lacking a visual counter name every paid visual attempt.
fn reconcile_visual(
    run: &mut Budget,
    reconciliation: &super::continuation::VisualReconciliation,
) -> Result<(), String> {
    if run.visual_passes.is_some()
        || run.max_visual_passes.is_some()
        || reconciliation.reason.trim().is_empty()
        || reconciliation.paid_ledgers.is_empty()
    {
        return Err("visual accounting reconciliation is initial and evidence-backed only".into());
    }
    let mut unique = std::collections::HashSet::new();
    for path in &reconciliation.paid_ledgers {
        if !unique.insert(fs::canonicalize(path).map_err(|e| e.to_string())?) {
            return Err("duplicate visual ledger".into());
        }
        let record: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if record.get("status").is_none() {
            return Err("not a visual attempt ledger".into());
        }
    }
    let spent = unique.len() as u64;
    if spent > reconciliation.previous_cap {
        return Err("prior visual cap exceeded".into());
    }
    run.visual_passes = Some(spent);
    run.max_visual_passes = Some(reconciliation.previous_cap);
    Ok(())
}

/// Applies the decision's extensions and the config's caps to a resumed run,
/// refusing any cap the config sets that the run does not already carry.
pub fn resume(run: &mut Budget, config: &Budget, decision: &HumanDecision) -> Result<(), String> {
    resume_cap(
        "token",
        decision.token_cap_extension.as_ref(),
        &mut run.max_tokens,
        config.max_tokens,
    )?;
    if let Some(reconciliation) = &decision.visual_reconciliation {
        reconcile_visual(run, reconciliation)?;
    }
    resume_cap(
        "round",
        decision.round_cap_extension.as_ref(),
        &mut run.max_rounds,
        config.max_rounds,
    )?;
    resume_cap(
        "visual",
        decision.visual_cap_extension.as_ref(),
        &mut run.max_visual_passes,
        config.max_visual_passes,
    )?;
    resume_cap(
        "image",
        decision.image_cap_extension.as_ref(),
        &mut run.max_images,
        config.max_images,
    )?;
    resume_cap(
        "evaluation",
        decision.evaluation_cap_extension.as_ref(),
        &mut run.max_evaluations,
        config.max_evaluations,
    )?;
    if [
        run.max_tokens,
        run.max_images,
        run.max_rounds,
        run.max_evaluations,
    ] != [
        config.max_tokens,
        config.max_images,
        config.max_rounds,
        config.max_evaluations,
    ] {
        return Err("resume cannot silently change species or budget caps".into());
    }
    if run.max_visual_passes != config.max_visual_passes {
        return Err("resume cannot silently change visual cap".into());
    }
    Ok(())
}

/// The config as it stood before this resume's caps, for the evidence-reuse
/// identity check: the new caps normalised back to the run's old ones.
pub fn restore(config: &mut Budget, previous: &Budget) {
    config.max_tokens = previous.max_tokens;
    config.max_rounds = previous.max_rounds;
    config.max_visual_passes = previous.max_visual_passes;
    config.max_images = previous.max_images;
    config.max_evaluations = previous.max_evaluations;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuning::continuation::PilotAuthority;
    use serde_json::json;

    fn capped(tokens: u64) -> Budget {
        Budget {
            max_tokens: Some(tokens),
            max_rounds: Some(3),
            max_images: Some(52),
            max_evaluations: Some(13),
            max_visual_passes: Some(26),
            visual_passes: Some(0),
            ..Budget::default()
        }
    }

    fn decision(extension: Option<(u64, u64)>) -> HumanDecision {
        let mut d = json!({"pause_id": "p", "identity": "i", "action": "a", "by": "owner",
            "rationale": "r"});
        if let Some((previous, next)) = extension {
            d["token_cap_extension"] = json!({"previous": previous, "next": next});
        }
        serde_json::from_value(d).unwrap()
    }

    /// fn-117: a config without caps drops the run's; a set cap is compared as
    /// before, and only an exact extension raises it.
    #[test]
    fn only_the_caps_a_config_sets_are_compared_on_resume() {
        let uncapped = Budget::default();
        for (config, extension, ok) in [
            (uncapped.clone(), None, true),
            (capped(100), None, true),
            (capped(200), None, false),
            (capped(200), Some((100, 200)), true),
            (capped(200), Some((90, 200)), false),
        ] {
            let mut run = capped(100);
            let got = resume(&mut run, &config, &decision(extension));
            assert_eq!(got.is_ok(), ok, "{config:?} {extension:?}: {got:?}");
            if ok {
                assert_eq!(run.max_tokens, config.max_tokens);
                assert_eq!(run.max_visual_passes, config.max_visual_passes);
            }
        }
    }

    /// With no cap set the pilot authority restates none; a number it names
    /// that the run does not carry is still a mismatch.
    #[test]
    fn a_pilot_authority_restates_only_the_caps_the_run_carries() {
        let authority = |caps: serde_json::Value| -> PilotAuthority {
            let mut a = json!({"purpose": "p", "reason": "r", "next_identity": "i"});
            a.as_object_mut()
                .unwrap()
                .extend(caps.as_object().unwrap().clone());
            serde_json::from_value(a).unwrap()
        };
        let uncapped = Budget::default();
        assert!(authority(json!({})).verify("i", &uncapped).is_ok());
        assert!(authority(json!({"max_tokens": 5}))
            .verify("i", &uncapped)
            .is_err());
        let run = capped(100);
        let exact = json!({"max_tokens": 100, "max_rounds": 3, "max_images": 52,
            "max_evaluations": 13, "max_visual_passes": 26});
        assert!(authority(exact).verify("i", &run).is_ok());
        assert!(authority(json!({})).verify("i", &run).is_err());
    }
}
