use std::{fs,path::Path};
use telperion_jev::{sha256_hex,tuning::{vision::Request,live::Config}};
fn main(){
 let root=Path::new(".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev");let jp=root.join("joint-blind-astra-journal.json");assert!(!jp.exists(),"one approved attempt only");
 let bytes=fs::read(root.join("joint-blind-request.json")).unwrap();let request:Request=serde_json::from_slice(&bytes).unwrap();request.verify().unwrap();assert_eq!(request.hash(),"c82e452cf172c97481433192b34db4833d9213abca5499055d45c196e8275f87");assert_eq!(sha256_hex(&fs::read("scripts/tuning-vision-codex.py").unwrap()),"d3d17c0f98d48a9e2543807127bc414d3bf7fedd0c605325359a7be9f1c137cb");
 let mut cfg:Config=serde_json::from_slice(&fs::read(root.join("pilot-config-final-diagnosed.json")).unwrap()).unwrap();cfg.vision.model="gpt-6-astra".into();let index=cfg.vision.args.iter().position(|s|s=="--model").unwrap()+1;cfg.vision.args[index]="gpt-6-astra".into();assert_eq!(cfg.vision.effort,"medium");
 let original=sha256_hex(&fs::read(".flow/tmp/fn68-pilot-run/run.json").unwrap());
 let mut journal=serde_json::json!({"authority":"Owner yes to400k/up-to2stronger frozen-evidence experiments; host releases FIRST Astra medium only,35k reservation,visual12to13,no retries/renders","status":"reserved_before_dispatch","prior_actual_tokens":327203,"reserved_tokens":35000,"cumulative_cap":400000,"prior_visual_passes":12,"attempted_visual_passes":13,"model":"gpt-6-astra","effort":"medium","request_sha256":request.hash(),"request_file_sha256":sha256_hex(&bytes),"protocol_sha256":sha256_hex(&fs::read("scripts/tuning-vision-codex.py").unwrap()),"original_run_sha256":original,"evaluations":6,"image_reservations":24,"actual_images":20,"scope":"known development blind joint review,not general role qualification"});
 assert!(327203+35000<=400000);fs::write(&jp,serde_json::to_vec_pretty(&journal).unwrap()).unwrap();
 let result=cfg.vision.assess(&request).expect("failed/unknown retain reservation stop");
 fs::write(root.join("joint-blind-astra-result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();
 let u=result.usage.as_ref().expect("unknown usage retain reservation stop");let used=u.input_tokens.checked_add(u.output_tokens).unwrap();
 journal["actual_tokens"]=serde_json::json!(used);journal["cumulative_actual_tokens"]=serde_json::json!(327203+used);journal["status"]=serde_json::json!(if used<=35000{"settled"}else{"over_reservation_stop"});
 assert_eq!(sha256_hex(&fs::read(".flow/tmp/fn68-pilot-run/run.json").unwrap()),original);fs::write(&jp,serde_json::to_vec_pretty(&journal).unwrap()).unwrap();println!("{}",serde_json::to_string(&result).unwrap());assert!(used<=35000);
}
