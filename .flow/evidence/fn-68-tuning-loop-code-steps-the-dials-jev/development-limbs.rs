use std::{fs,path::Path};
use telperion_jev::{sha256_hex,tuning::{live::{Config,Live},engine::Services}};
fn main(){
 let root=Path::new(".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev");let journal=root.join("development-limbs-journal.json");assert!(!journal.exists(),"one attempt only");
 let config_path=root.join("pilot-config-final-diagnosed.json");let bytes=fs::read(&config_path).unwrap();let cfg:Config=serde_json::from_slice(&bytes).unwrap();
 cfg.verify().unwrap();let identity=cfg.identity().unwrap();let original=fs::read(".flow/tmp/fn68-pilot-run/run.json").unwrap();let hash=sha256_hex(&original);
 let protocol=fs::read(root.join("development-limbs-protocol.json")).unwrap();let p:serde_json::Value=serde_json::from_slice(&protocol).unwrap();
 assert_eq!(cfg.seed,1);assert_eq!(cfg.matched.numeric_references.len(),2);
 let mut j=serde_json::json!({"status":"reserved_before_evaluation","protocol_sha256":sha256_hex(&protocol),"config_sha256":sha256_hex(&bytes),"identity":identity,"original_run_sha256":hash,"prior_actual_tokens":221183,"reserved_evaluations":1,"reserved_images":4,"actual_model_tokens":0});
 fs::write(&journal,serde_json::to_vec_pretty(&j).unwrap()).unwrap();
 let t=telperion_jev::caller::UreqTransport;let mut live=Live{config:&cfg,transport:&t,key:"offline-no-model-call"};
 let trial=live.evaluate(p["candidate"].clone(),3,"owner-development-limbs2to1",None);
 let image_count=trial.comparisons.iter().map(|c|c.images.len()).sum::<usize>();assert!(image_count<=4);
 let trial_path=root.join("local/development-limbs-trial.json");fs::write(&trial_path,serde_json::to_vec_pretty(&trial).unwrap()).unwrap();
 assert_eq!(sha256_hex(&fs::read(".flow/tmp/fn68-pilot-run/run.json").unwrap()),hash);
 j["status"]=serde_json::json!("evaluation_complete_waiting_host_visual");j["actual_images"]=serde_json::json!(image_count);
 j["trial"]=serde_json::json!({"key":trial.key,"identity":trial.identity,"overrides":trial.overrides,"feasible":trial.feasible,"reason":trial.reason,"score":trial.score,"seconds":trial.seconds,"comparisons":trial.comparisons,"nodes":trial.measurement["metrics"]["nodes"],"growth":trial.measurement["metrics"]["growth"],"numeric_status":trial.measurement["numeric_status"],"raw_local_receipt":trial_path});
 fs::write(&journal,serde_json::to_vec_pretty(&j).unwrap()).unwrap();println!("{}",j);
}
