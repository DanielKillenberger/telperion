use std::{fs,path::Path};
use telperion_jev::{sha256_hex,tuning::{live::Config,evaluation::Trial,engine::Run,vision::Request,state::Cell}};
fn main(){
 let root=Path::new(".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev");let jp=root.join("development-limbs-journal.json");let mut j:serde_json::Value=serde_json::from_slice(&fs::read(&jp).unwrap()).unwrap();assert!(j.get("direct_reference_visual").is_none(),"one visual attempt only");
 let original=fs::read(".flow/tmp/fn68-pilot-run/run.json").unwrap();assert_eq!(sha256_hex(&original),j["original_run_sha256"].as_str().unwrap());
 let cfg:Config=serde_json::from_slice(&fs::read(root.join("pilot-config-final-diagnosed.json")).unwrap()).unwrap();
 let trial:Trial=serde_json::from_slice(&fs::read(root.join("local/development-limbs-trial.json")).unwrap()).unwrap();assert!(trial.feasible);
 let mut images=Vec::new();for c in &trial.comparisons{images.push(c.images[0].clone());}
 let request=Request{schema:"tuning-vision-v2".into(),identity:trial.key.clone(),required:vec![Cell{item:"direct-reference-mature-beech-whole".into(),view:"B-WHOLE".into(),seed:1},Cell{item:"direct-reference-mature-beech-bare".into(),view:"B-BARE".into(),seed:1}],images,references:cfg.references.iter().filter(|i|i.view=="B-WHOLE"||i.view=="B-BARE").cloned().collect(),quality_anchors:cfg.quality_anchors.clone(),checklist:"Assess candidate whole and bare directly against the photographic references: is this recognizable mature European beech at established catalogue quality, not photorealism? Spruce is a style/finish floor only, never a species-morphology reference. For each view distinguish species-character blockers from plausible natural variation and optional refinement. Upright form or limited droop is neither grandfathered nor automatically disqualifying: explain whether the observed difference actually defeats mature-beech recognizability. Do not demand exact photographic shape matching. Pass when recognizable and meeting the finish floor, fail only for a grounded blocker, unknown when evidence cannot establish the distinction. Two view cells only; no six-cell machine-readiness claim.".into()};
 request.verify().unwrap();assert_eq!(request.images.len()+request.references.len()+request.quality_anchors.len(),5);
 fs::write(root.join("development-direct-reference-visual-request.json"),serde_json::to_vec_pretty(&request).unwrap()).unwrap();
 j["direct_reference_visual"]=serde_json::json!({"status":"reserved_before_dispatch","prior_passes":10,"attempted_passes":11,"prior_actual_tokens":277432,"reserved_tokens":35000,"cumulative_cap":320000,"request_sha256":request.hash(),"scope":"two direct-reference cells only; never six-cell readiness","authorization":"Owner explicitly approved320000 ceiling and ONE direct-reference assessment: ok go; visual10 to11; no retry","previous_request_sha256":j["visual"]["request_sha256"]});
 fs::write(&jp,serde_json::to_vec_pretty(&j).unwrap()).unwrap();
 let result=cfg.vision.assess(&request).expect("failed or unknown assessment: retain reservation and stop");
 fs::write(root.join("development-direct-reference-visual-result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();
 let usage=result.usage.as_ref().expect("unknown usage: retain reservation and stop");let used=usage.input_tokens.checked_add(usage.output_tokens).unwrap();
 j["direct_reference_visual"]["actual_tokens"]=serde_json::json!(used);j["direct_reference_visual"]["cumulative_actual_tokens"]=serde_json::json!(277432+used);j["direct_reference_visual"]["status"]=serde_json::json!(if used<=35000{"settled"}else{"over_reservation_stop"});
 assert_eq!(sha256_hex(&fs::read(".flow/tmp/fn68-pilot-run/run.json").unwrap()),j["original_run_sha256"].as_str().unwrap());
 fs::write(&jp,serde_json::to_vec_pretty(&j).unwrap()).unwrap();println!("{}",serde_json::to_string(&result).unwrap());assert!(used<=35000);
}
