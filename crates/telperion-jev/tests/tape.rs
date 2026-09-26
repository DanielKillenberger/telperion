//! Record and replay (fn-149, owner 2026-09-25): every external answer a run
//! receives is kept by a stable key and served back with no network; a
//! request the recording lacks fails, naming it.
use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::{
    AdapterError, FetchAdapter, FixtureAdapter, Scrape, SearchHit, Spent,
};
use telperion_jev::pipeline::canon::write_canonical;
use telperion_jev::pipeline::photos::Web;
use telperion_jev::tape::{self, Tape};

const PAGE: &str = "https://example.test/beech";

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-tape-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A fixture that answers one page and one search.
fn fixture(dir: &Path) -> FixtureAdapter {
    fs::write(dir.join("p.html"), [0u8, 159, 146, 150]).unwrap();
    fs::write(dir.join("p.md"), "Beech reaches 30 m.").unwrap();
    let hit = json!([{"url": PAGE, "title": "Beech", "snippet": "30 m"}]);
    write_canonical(
        &dir.join("index.json"),
        &json!({"scrape": {PAGE: {"final_url": PAGE, "content_type": "text/html", "raw": "p.html", "markdown": "p.md"}},
                "search": {"beech": hit}}),
    )
    .unwrap();
    FixtureAdapter::new(dir)
}

/// An adapter that must never be reached: a replay makes no call.
struct Offline;

impl FetchAdapter for Offline {
    fn search(&self, _: &str, _: usize) -> Result<Vec<SearchHit>, AdapterError> {
        panic!("a replay searched")
    }
    fn research(&self, _: &str, _: usize) -> Result<Vec<SearchHit>, AdapterError> {
        panic!("a replay searched")
    }
    fn scrape(&self, _: &str) -> Result<Scrape, AdapterError> {
        panic!("a replay scraped")
    }
    fn parse_pdf(&self, _: &Path) -> Result<String, AdapterError> {
        panic!("a replay parsed")
    }
    fn spent(&self) -> Spent {
        Spent::default()
    }
}

#[test]
fn a_recorded_fetch_replays_offline_and_a_missing_one_names_its_request() {
    let (fixtures, tape_dir) = (scratch("fixtures"), scratch("recording"));
    let recording = tape::Fetch {
        inner: Box::new(fixture(&fixtures)),
        tape: Tape::Record(tape_dir.clone()),
    };
    let live = recording.scrape(PAGE).unwrap();
    let hits = recording.search("beech", 5).unwrap();
    let missing = recording.scrape("https://example.test/none").unwrap_err();

    let replay = tape::Fetch {
        inner: Box::new(Offline),
        tape: Tape::Replay(tape_dir),
    };
    assert_eq!(replay.scrape(PAGE).unwrap(), live, "raw bytes and markdown");
    assert_eq!(replay.search("beech", 5).unwrap(), hits);
    // A recorded failure replays as the same failure.
    assert_eq!(
        replay.scrape("https://example.test/none").unwrap_err(),
        missing
    );
    let err = replay.search("oak", 5).unwrap_err().to_string();
    assert!(
        err.contains("replay:") && err.contains("\"query\":\"oak\""),
        "{err}"
    );
}

/// Answers a Jev call once; a second call would be a network call.
struct Once(Cell<u32>);

impl Transport for Once {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        self.0.set(self.0.get() + 1);
        assert!(self.0.get() == 1, "the replay reached the network");
        let asked: Value = serde_json::from_slice(request.body.as_deref().unwrap()).unwrap();
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"echo": asked["state"]})).unwrap(),
        })
    }
}

#[test]
fn a_jev_call_is_keyed_by_its_body_never_its_key_and_replays() {
    let dir = scratch("jev");
    let once = Once(Cell::new(0));
    let request = |key: &str| HttpRequest {
        method: "POST",
        url: "https://api.test/v1".into(),
        headers: vec![("Authorization".into(), format!("Bearer {key}"))],
        body: Some(serde_json::to_vec(&json!({"state": {"field": "height_m"}})).unwrap()),
    };
    let recording = tape::Jev {
        inner: &once,
        tape: Some(Tape::Record(dir.clone())),
    };
    let live = recording.send(&request("secret")).unwrap();
    let replay = tape::Jev {
        inner: &once,
        tape: Some(Tape::Replay(dir.clone())),
    };
    let served = replay.send(&request("replay")).unwrap();
    assert_eq!((served.status, served.body), (live.status, live.body));
    let recorded = fs::read_to_string(
        fs::read_dir(dir.join("jev"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path(),
    )
    .unwrap();
    assert!(
        !recorded.contains("secret"),
        "the key never reaches a recording"
    );
}

struct Images;

impl Web for Images {
    fn get(&self, url: &str) -> Result<(Vec<u8>, String), String> {
        Ok((url.as_bytes().to_vec(), "image/jpeg".into()))
    }
}

#[test]
fn a_photograph_replays_its_bytes() {
    let dir = scratch("web");
    let recording = tape::Photos {
        inner: &Images,
        tape: Some(Tape::Record(dir.clone())),
    };
    let live = recording.get("https://upload.test/a.jpg").unwrap();
    struct Down;
    impl Web for Down {
        fn get(&self, _: &str) -> Result<(Vec<u8>, String), String> {
            panic!("a replay downloaded")
        }
    }
    let replay = tape::Photos {
        inner: &Down,
        tape: Some(Tape::Replay(dir)),
    };
    assert_eq!(replay.get("https://upload.test/a.jpg").unwrap(), live);
    assert!(replay.get("https://upload.test/b.jpg").is_err());
}

#[test]
fn every_adapter_program_in_a_config_goes_through_the_tape_script() {
    let mut config = json!({"vision": {"program": "python3", "args": ["scripts/reference-first.py", "--model", "m"]},
        "sheet": {"adapter": {"program": "claude", "args": []}, "protocol": "p"}, "matched": {"headless": "h"}});
    tape::wrap(&mut config, &Tape::Replay("/rec".into()));
    assert_eq!(
        config["vision"]["args"],
        json!([
            tape::ADAPTER,
            "replay:/rec",
            "--",
            "python3",
            "scripts/reference-first.py",
            "--model",
            "m"
        ])
    );
    assert_eq!(config["sheet"]["adapter"]["program"], "python3");
    assert_eq!(config["matched"]["headless"], "h");
}
