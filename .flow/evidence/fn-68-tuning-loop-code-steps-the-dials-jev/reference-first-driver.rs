use std::{fs,io::Write,path::{Path,PathBuf}};
use serde_json::{json,Value};
use telperion_jev::{sha256_hex,tuning::{reference_first::*,vision,state::ready}};
fn read(p:&Path)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:&Path,v:&Value){let bytes=serde_json::to_vec_pretty(v).unwrap();if p.exists(){assert_eq!(read(p),*v);return;}fs::OpenOptions::new().create_new(true).write(true).open(p).unwrap().write_all(&bytes).unwrap();}
fn main(){
 let args:Vec<String>=std::env::args().collect();let mode=&args[1];let root=PathBuf::from(&args[2]);
 let input=args.get(3).map(String::as_str).unwrap_or("joint-blind-request.json");let prefix=args.get(4).map(String::as_str).unwrap_or("reference-first");
 let original:vision::Request=serde_json::from_value(read(&root.join(input))).unwrap();
 let reference=ReferenceRequest::from_comparison(&original);reference.verify().unwrap();
 if mode=="prepare" {save(&root.join("reference-first-a-request.json"),&json!({"stage":"inventory","request":reference,"request_sha256":reference.hash(),"prompt":INVENTORY_PROMPT,"prompt_sha256":reference.prompt_hash()}));return;}
 let a_path=root.join("reference-first-a-receipt.json");let raw_a=read(&a_path);
 assert_eq!(raw_a["model"],"gpt-6-astra");assert_eq!(raw_a["effort"],"medium");
 assert_eq!(raw_a["request_sha256"],reference.hash());assert_eq!(raw_a["prompt_sha256"],reference.prompt_hash());
 for k in ["input_tokens","output_tokens"]{assert!(raw_a["usage"][k].as_u64().is_some());}
 let inventory=Inventory{request:reference.clone(),request_sha256:reference.hash(),prompt_sha256:reference.prompt_hash(),model:"gpt-6-astra".into(),effort:"medium".into(),ledger:format!("{}#sha256:{}",a_path.display(),sha256_hex(&fs::read(&a_path).unwrap())),traits:serde_json::from_value(raw_a["answer"]["traits"].clone()).unwrap(),observations:serde_json::from_value(raw_a["answer"]["observations"].clone()).unwrap()};
 inventory.verify().unwrap();assert!(inventory.traits.iter().any(|t|t.priority==Priority::Core&&!t.uncertain),"no useful certain core trait; stop");
 let comparison=ComparisonRequest::new(&original,inventory.clone());comparison.verify().unwrap();
 save(&root.join("reference-first-inventory.json"),&serde_json::to_value(&inventory).unwrap());
 save(&root.join(format!("{prefix}-b-request.json")),&json!({"stage":"comparison","request":comparison,"request_sha256":comparison.hash(),"prompt":COMPARISON_PROMPT,"prompt_sha256":sha256_hex(COMPARISON_PROMPT.as_bytes())}));
 if mode=="inventory" {return;}
 assert_eq!(mode,"comparison");let raw_b=read(&root.join(format!("{prefix}-b-receipt.json")));
 assert_eq!(raw_b["model"],"gpt-6-astra");assert_eq!(raw_b["effort"],"medium");assert_eq!(raw_b["request_sha256"],comparison.hash());assert_eq!(raw_b["prompt_sha256"],comparison.prompt_sha256);
 let answer=&raw_b["answer"];let passes=answer["passes"].as_array().unwrap();assert_eq!(passes.len(),comparison.comparison.required.len());
 let cells:Vec<Value>=comparison.comparison.required.iter().zip(passes).map(|(cell,status)|json!([cell,status])).collect();
 let visual:vision::Result=serde_json::from_value(json!({"request_sha256":comparison.comparison.hash(),"assessment":{"identity":comparison.comparison.identity,"model":"gpt-6-astra","ledger":format!("{prefix}-b-receipt.json"),"cells":cells,"defects":answer["defects"],"findings":answer["findings"]},"effort":"medium","usage":{"input_tokens":raw_b["usage"]["input_tokens"],"output_tokens":raw_b["usage"]["output_tokens"]},"observations":answer["observations"]})).unwrap();
 let mut result=ComparisonResult{request_sha256:comparison.hash(),visual,coverage:serde_json::from_value(answer["coverage"].clone()).unwrap()};result.bind(&comparison).unwrap();
 let required_ready=ready(&comparison.comparison.required,&comparison.comparison.identity,&result.visual.assessment);
 let output=args.get(5).cloned().unwrap_or_else(||format!("{prefix}-result.json"));
 save(&root.join(output),&json!({"result":result,"two_cell_ready":required_ready,"full_species_ready":false,"qualification":"unvalidated development experiment"}));println!("two_cell_ready={required_ready}; never full species readiness");
}
