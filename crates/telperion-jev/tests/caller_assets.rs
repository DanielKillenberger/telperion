//! A question about a picture: the image travels as an asset, the ledger
//! keeps its digest, and an image outside the documented limits never leaves.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use base64::Engine;
use serde_json::{json, Value};
use telperion_jev::assets::{load_asset, IMAGES_HEADER};
use telperion_jev::caller::{
    evaluate_with_assets, EvaluateRequest, HttpRequest, HttpResponse, Transport,
};
use telperion_jev::sha256_hex;

/// Each request's headers and parsed body.
type Seen = (Vec<(String, String)>, Value);

struct Recorder(Mutex<Vec<Seen>>);

impl Transport for Recorder {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body = serde_json::from_slice(request.body.as_ref().unwrap()).unwrap();
        self.0.lock().unwrap().push((request.headers.clone(), body));
        let reply = json!({
            "model": "jev-test",
            "answers": {"leaves": {"type": "choice", "choice": "in_leaf",
                "confidence": 0.9, "probabilities": {"in_leaf": 0.95, "neither": 0.05}}},
            "usage": {"input_tokens": 300, "output_tokens": 2}
        });
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&reply).unwrap(),
        })
    }
}

/// A PNG header of the given size; the caller reads no further than IHDR.
fn png(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = b"\x89PNG\r\n\x1a\n\0\0\0\x0dIHDR".to_vec();
    bytes.extend(width.to_be_bytes());
    bytes.extend(height.to_be_bytes());
    bytes.extend([8, 2, 0, 0, 0, 0, 0, 0, 0]);
    bytes
}

/// A baseline JPEG header: SOI, an APP0 segment, then SOF0.
fn jpeg(width: u16, height: u16) -> Vec<u8> {
    let mut bytes = vec![0xFF, 0xD8, 0xFF, 0xE0, 0, 16];
    bytes.extend(b"JFIF\0\x01\x01\0\0\x01\0\x01\0\0");
    bytes.extend([0xFF, 0xC0, 0, 17, 8]);
    bytes.extend(height.to_be_bytes());
    bytes.extend(width.to_be_bytes());
    bytes.extend([3, 1, 0x22, 0, 2, 0x11, 1, 3, 0x11, 1]);
    bytes
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-assets-{tag}-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &Path, name: &str, bytes: &[u8]) -> String {
    let path = dir.join(name);
    std::fs::write(&path, bytes).unwrap();
    path.to_string_lossy().into_owned()
}

#[test]
fn the_request_carries_the_image_and_the_ledger_its_digest() {
    let dir = scratch("send");
    let bytes = jpeg(1200, 974);
    let path = write(&dir, "still.jpg", &bytes);
    let image = load_asset("still-7f3a", &path).unwrap();
    let transport = Recorder(Mutex::new(Vec::new()));
    let state = json!({"still": "still-7f3a"});
    let questions = json!({"leaves": {"type": "choice", "instructions": "x",
        "criteria": {"in_leaf": "a", "neither": "b"}}});
    let ledger = dir.join("ledger");
    let entry = evaluate_with_assets(
        &transport,
        "k",
        EvaluateRequest {
            tool: "ask",
            source: None,
            state: &state,
            questions: &questions,
            ledger_dir: &ledger,
        },
        &[image],
    )
    .unwrap();

    let seen = transport.0.lock().unwrap();
    let (headers, body) = &seen[0];
    assert!(headers.contains(&(IMAGES_HEADER.into(), "true".into())));
    let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
    assert_eq!(
        body["assets"],
        json!({"still-7f3a": {"type": "image", "format": "jpg", "data_base64": encoded}})
    );
    assert_eq!(body["state"], state);

    let digest = sha256_hex(&bytes);
    assert_eq!(entry.assets.len(), 1);
    let file = std::fs::read_dir(&ledger).unwrap().next().unwrap().unwrap();
    let written: Value = serde_json::from_slice(&std::fs::read(file.path()).unwrap()).unwrap();
    assert_eq!(
        written["assets"],
        json!([{"name": "still-7f3a", "path": path, "sha256": digest}])
    );
    let text = std::fs::read_to_string(file.path()).unwrap();
    assert!(!text.contains(&encoded), "the ledger holds no image bytes");
}

#[test]
fn an_image_outside_the_documented_limits_is_refused_by_name() {
    let dir = scratch("limits");
    let cases: [(&str, Vec<u8>, &str); 5] = [
        (
            "wide",
            png(2440, 1980),
            "2440x1980, each side must be 280 to 1568",
        ),
        (
            "small",
            png(279, 600),
            "279x600, each side must be 280 to 1568",
        ),
        ("tall_jpeg", jpeg(1200, 1569), "1200x1569"),
        (
            "gif",
            b"GIF89a\x01\x00\x01\x00".to_vec(),
            "not a PNG or JPEG",
        ),
        (
            "heavy",
            vec![0u8; 16 * 1024 * 1024 + 1],
            "over the 16 MiB per-image limit",
        ),
    ];
    for (name, bytes, want) in cases {
        let path = write(&dir, name, &bytes);
        let err = load_asset(name, &path).err().expect(name);
        assert!(err.starts_with(&format!("asset {name} (")), "{err}");
        assert!(err.contains(want), "{name}: {err}");
    }
    let edge = write(&dir, "edge.png", &png(280, 1568));
    assert!(
        load_asset("edge", &edge).is_ok(),
        "both ends of the range are allowed"
    );
}
