//! fn-149, owner 2026-09-25 (option A): the Profile stage finds its own
//! reference photographs. Commons candidates pass the rights question, one
//! look keeps a mature open-grown whole tree and a bark close-up, and the
//! kept ones join `packet/references.json` beside what was recorded, never
//! over it. A fake web, a mock transport and a fake look: no network.
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::photos::{self, commons, Screen, Verdict, Web};
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::tuning::evaluation::Image;

const TAXON: &str = "Fagus sylvatica";

/// Commons answers each of the three queries with its own files; a file
/// named `restricted` carries an all-rights-reserved licence.
struct FakeWeb(BTreeMap<String, Vec<u8>>);

impl Web for FakeWeb {
    fn get(&self, url: &str) -> Result<(Vec<u8>, String), String> {
        self.0
            .get(url)
            .cloned()
            .map(|b| (b, "image/jpeg".into()))
            .ok_or(url.into())
    }
}

fn web() -> FakeWeb {
    let mut pages = BTreeMap::new();
    let file = |name: &str, index: u64, licence: &str| {
        json!({"index": index, "title": format!("File:{name}.jpg"), "imageinfo": [{"mime": "image/jpeg",
            "descriptionurl": format!("https://commons.wikimedia.org/wiki/File:{name}.jpg"),
            "thumburl": format!("https://upload.wikimedia.org/{name}.jpg"),
            "extmetadata": {"LicenseShortName": {"value": licence}, "Artist": {"value": "Ann"}}}]})
    };
    let answers = [
        vec![
            ("whole", "CC BY-SA 4.0"),
            ("stand", "CC BY 4.0"),
            ("restricted", "All rights reserved"),
        ],
        vec![("bark", "CC0")],
        vec![("winter", "CC BY-SA 4.0")],
    ];
    for ((query, limit), files) in commons::queries(TAXON).into_iter().zip(answers) {
        let listed: serde_json::Map<String, Value> = files
            .iter()
            .enumerate()
            .map(|(i, (name, licence))| (name.to_string(), file(name, i as u64 + 1, licence)))
            .collect();
        let body = json!({"query": {"pages": listed}});
        pages.insert(
            commons::url(&query, limit),
            serde_json::to_vec(&body).unwrap(),
        );
        for (name, _) in files {
            let mut jpeg = vec![0xFF, 0xD8, 0xFF, 0xE0];
            jpeg.extend(name.as_bytes());
            pages.insert(format!("https://upload.wikimedia.org/{name}.jpg"), jpeg);
        }
    }
    FakeWeb(pages)
}

/// Classes a licence with "reserved" in it restricted, any other open.
struct Rights;

impl Transport for Rights {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let reserved = body["state"]["lines"].to_string().contains("reserved");
        let class = if reserved {
            "restricted"
        } else {
            "open-licence"
        };
        let answers = json!({"rights": {"type": "choice", "choice": class, "confidence": 0.9, "probabilities": {class: 0.9}}});
        let body = json!({"model": "jev-latest", "answers": answers});
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&body).unwrap(),
        })
    }
}

/// Keeps the whole tree and the bark; the stand-grown tree is not open-grown
/// and the winter one is framed short. Records what it was shown.
#[derive(Default)]
struct Look(Mutex<Vec<usize>>);

impl Screen for Look {
    fn screen(&self, _: &str, images: &[Image]) -> Result<(Vec<Verdict>, Value), String> {
        self.0.lock().unwrap().push(images.len());
        let verdict = |i: usize, image: &Image| {
            let name = String::from_utf8_lossy(&fs::read(&image.path).unwrap()[4..]).into_owned();
            let (grown, whole, view) = match name.as_str() {
                "whole" => (true, true, "leaf-on"),
                "stand" => (false, true, "leaf-on"),
                "bark" => (false, false, "bark"),
                _ => (true, false, "bare"),
            };
            Verdict {
                id: format!("candidate-{i}"),
                species: "yes".into(),
                mature_open_grown: grown,
                whole_tree: whole,
                view: view.into(),
            }
        };
        let verdicts = images
            .iter()
            .enumerate()
            .map(|(i, im)| verdict(i, im))
            .collect();
        Ok((
            verdicts,
            json!({"input_tokens": 1000, "output_tokens": 100}),
        ))
    }
}

fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-photos-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(dir.join("packet")).unwrap();
    let manifest = json!({
        "schema": "manifest", "schema_version": 1, "species": "european-beech",
        "taxon": {"scientific_name": TAXON, "common_name": "European beech", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "european-beech", "profile_id": "european-beech", "seed": 7,
        "sources": [], "fields": [], "versions": {"question_sets": {}, "tools": {}}, "model": "jev-latest"
    });
    write_canonical(&dir.join("manifest.json"), &manifest).unwrap();
    let recorded = json!({"reference_version": "fn19-references-v1", "sources": [],
        "references": [{"id": "P-OLD", "source_id": "R1", "asset_sha256": "a", "kept": false}]});
    write_canonical(&dir.join("packet/references.json"), &recorded).unwrap();
    dir
}

#[test]
fn a_run_from_a_name_finds_checks_and_records_its_own_photographs() {
    let dir = scratch();
    let paths = Paths::new(&dir);
    let judge = Judge {
        transport: &Rights,
        key: "test-key",
        ledger_dir: dir.join("ledger"),
    };
    let look = Look::default();
    let word = photos::find(&paths, &web(), &judge, &look).unwrap();
    assert!(
        word.ends_with("5 candidates, 4 open-licence, 2 kept"),
        "{word}"
    );
    assert_eq!(
        *look.0.lock().unwrap(),
        [4],
        "one look over the open-licence batch"
    );
    let doc = read_json(&dir.join("packet/references.json")).unwrap();
    let list = doc["references"].as_array().unwrap();
    assert_eq!(list[0]["id"], "P-OLD", "a recorded reference is kept");
    let kept: Vec<(&str, &str, &str)> = list[1..]
        .iter()
        .map(|r| {
            (
                r["view"].as_str().unwrap(),
                r["source_id"].as_str().unwrap(),
                r["url"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        kept,
        [
            (
                "leaf-on",
                "R2",
                "https://commons.wikimedia.org/wiki/File:whole.jpg"
            ),
            (
                "bark",
                "R3",
                "https://commons.wikimedia.org/wiki/File:bark.jpg"
            ),
        ]
    );
    let bark = &list[2];
    assert_eq!(bark["attribution"], "Ann, Wikimedia Commons, CC0");
    let copy = photos::copy(&paths, bark["asset_sha256"].as_str().unwrap()).unwrap();
    assert_eq!(
        telperion_jev::sha256_hex(&fs::read(copy).unwrap()),
        bark["asset_sha256"]
    );
    let report = read_json(&photos::dir(&paths).join("find.json")).unwrap();
    assert_eq!(report["rights_calls"], 5);
    assert_eq!(report["vision_calls"], 1);

    // Recorded enough: a rerun looks at nothing and changes nothing.
    let before = fs::read(dir.join("packet/references.json")).unwrap();
    let word = photos::find(&paths, &web(), &judge, &look).unwrap();
    assert_eq!(word, "3 reference photographs recorded");
    assert_eq!(
        fs::read(dir.join("packet/references.json")).unwrap(),
        before
    );
    assert_eq!(look.0.lock().unwrap().len(), 1);
}
