use std::{cell::Cell, fs, io::Write, path::{Path, PathBuf}};
use serde_json::{json, Value};
use telperion_jev::{caller::{evaluate, load_key_from_env, EvaluateRequest, HttpRequest, HttpResponse, Transport, UreqTransport}, sha256_hex};

struct Once<T> { inner: T, used: Cell<bool>, raw: Option<PathBuf> }
impl<T: Transport> Transport for Once<T> {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        if self.used.replace(true) { return Err("single HTTP attempt exhausted; retry blocked before dispatch".into()); }
        let response = self.inner.send(request)?;
        if let Some(path) = &self.raw {
            let mut file = fs::OpenOptions::new().write(true).create_new(true).open(path).map_err(|e| e.to_string())?;
            file.write_all(&response.body).map_err(|e| e.to_string())?;
        }
        Ok(response)
    }
}
fn read(path: &Path) -> Value { serde_json::from_slice(&fs::read(path).unwrap()).unwrap() }
fn new_file(path: &Path, value: &Value) {
    fs::OpenOptions::new().write(true).create_new(true).open(path).unwrap().write_all(&serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn main() {
    let root = PathBuf::from(std::env::args().nth(1).expect("evidence directory"));
    let state = read(&root.join("owner-grading-state.json"));
    let questions = read(&root.join("owner-grading-questions.json"));
    let preflight = read(&root.join("owner-grading-preflight.json"));
    let sb = serde_json::to_vec(&state).unwrap(); let qb = serde_json::to_vec(&questions).unwrap();
    assert_eq!(sha256_hex(&sb), preflight["state_sha256"]);
    assert_eq!(sha256_hex(&qb), preflight["questions_sha256"]);
    let reserve = (sb.len() + qb.len() + 1024) as u64;
    assert_eq!(reserve, 42249); assert!(464835 + reserve <= 520000);
    let runtime = Path::new(".flow/tmp/fn68-pilot-run/run.json");
    let runtime_hash = sha256_hex(&fs::read(runtime).unwrap());
    assert_eq!(runtime_hash, "d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e");
    let key = load_key_from_env().expect("invoke with bash -ic; key never printed");
    new_file(&root.join("owner-grading-reservation.json"), &json!({"authority":"Owner asks Jev to compare every verdict to owner; host approves frozen24Score packet and one HTTP attempt", "prior_actual_tokens":464835,"reserved_tokens":reserve,"cumulative_cap":520000,"state_sha256":sha256_hex(&sb),"questions_sha256":sha256_hex(&qb),"runtime_sha256":runtime_hash,"status":"reserved_before_dispatch"}));
    let once = Once { inner: UreqTransport, used: Cell::new(false), raw: Some(root.join("local/owner-grading-raw-response.json")) };
    let result = evaluate(&once, &key, EvaluateRequest {tool:"fn68-owner-agreement",source:None,state:&state,questions:&questions,ledger_dir:Path::new(".flow/ledger/fn68-owner-agreement")});
    match result {
        Ok(entry) => {
            let usage = entry.usage.as_ref().expect("unknown usage: reservation remains");
            let used = usage.input_tokens.checked_add(usage.output_tokens).unwrap();
            new_file(&root.join("owner-grading-result.json"), &json!({"entry":entry,"actual_tokens":used,"cumulative_actual_tokens":464835+used,"reservation":reserve,"status":if used<=reserve {"settled"} else {"over_reservation_stop"}}));
            assert!(used <= reserve);
            assert_eq!(sha256_hex(&fs::read(runtime).unwrap()),runtime_hash);
            println!("actual_tokens={used} cumulative={}",464835+used);
        }
        Err(error) => { new_file(&root.join("owner-grading-error.json"), &json!({"error":error.to_string(),"reservation_retained":reserve,"no_retry":true})); panic!("grading stopped; reservation retained"); }
    }
}
#[cfg(test)] mod tests {
    use super::*;
    struct Mock(Cell<u32>);
    impl Transport for Mock { fn send(&self, _: &HttpRequest) -> Result<HttpResponse,String> {self.0.set(self.0.get()+1);Ok(HttpResponse{status:429,body:vec![]})} }
    #[test] fn retry_never_reaches_inner_transport() {
        let once=Once{inner:Mock(Cell::new(0)),used:Cell::new(false),raw:None};
        let req=HttpRequest{method:"POST",url:"mock".into(),headers:vec![],body:None};
        assert_eq!(once.send(&req).unwrap().status,429);assert!(once.send(&req).is_err());assert_eq!(once.inner.0.get(),1);
    }
}
