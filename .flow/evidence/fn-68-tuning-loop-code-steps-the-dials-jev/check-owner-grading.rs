// Offline canonical-byte check using the same serde_json Value as the shared caller.
fn main() {
    for path in std::env::args().skip(1) {
        let raw = std::fs::read(&path).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
        let bytes = serde_json::to_vec(&value).unwrap();
        println!("{} {}", path, bytes.len());
    }
}
