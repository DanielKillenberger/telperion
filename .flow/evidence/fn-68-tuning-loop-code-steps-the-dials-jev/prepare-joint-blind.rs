use std::{fs,path::Path};
use telperion_jev::{sha256_hex,tuning::{vision::{Request,ReplayCase},joint::Packet,live::Config}};
fn main(){
 let root=Path::new(".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev");
 let mut request:Request=serde_json::from_slice(&fs::read(root.join("development-direct-reference-visual-request.json")).unwrap()).unwrap();
 let cfg:Config=serde_json::from_slice(&fs::read(root.join("pilot-config-final-diagnosed.json")).unwrap()).unwrap();
 request.joint=Some(Packet::from_request(&request).with_shots(&cfg.matched.references).unwrap());
 let fixture=ReplayCase{id:"known-beech-development".into(),provenance:"owner correction retained separately, not held-out generalization".into(),expected_ready:false,request};
 let blind=fixture.blind_request();blind.verify().unwrap();
 let encoded=serde_json::to_vec_pretty(&blind).unwrap();
 fs::write(root.join("joint-blind-request.json"),&encoded).unwrap();
 let audit=serde_json::json!({"request_sha256":blind.hash(),"file_sha256":sha256_hex(&encoded),"schema":blind.schema,"inputs":blind.images.len()+blind.references.len()+blind.quality_anchors.len(),"specimen_relation":blind.joint.as_ref().unwrap().reference_relation,"labels_in_request":false,"projection":"ReplayCase::blind_request; neutral checklist/cell IDs/anchor scope; factual image/shot provenance only; free-form reference relation provenance omitted, relation conservatively unknown","paid_calls":0,"status":"prepared_only","coverage":"known development specimen, not genuinely held-out biological generalization"});
 fs::write(root.join("joint-blind-audit.json"),serde_json::to_vec_pretty(&audit).unwrap()).unwrap();
 println!("{}",audit);
}
