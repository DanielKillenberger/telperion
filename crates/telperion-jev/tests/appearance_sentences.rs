//! fn-128 R2 and R3, from the palm's second pass on fn-127. Select read an
//! appearance trait off a 600-character page chunk: `bark_roughness` cited
//! A1's navigation links, and `bark_colour` came out unstated although A1
//! says the trunk "is rough gray". Verify then asked those values whether
//! they were measurements. A1's text here is the live extract's, in page
//! order behind its navigation. No network.

mod common;

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use common::{ledger_dir, CaseTransport};
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::stage::{Context, Paths};
use telperion_jev::pipeline::stages::{fetch, inputs, select, verify};

const A1_URL: &str = "https://example.test/a1";
const A1_NAV: &str = "[Edible Landscapes Tour](https://arboretum.arizona.edu/tours/edible-landscapes-tour) [Trees Around the World Tour](https://arboretum.arizona.edu/tours/trees-around-world-tour) **Botanical Name:** Phoenix dactylifera **Sub Species:** **Variety:** **Forma:** **Cultivar:** **Characteristics:**";
const A1_SENTENCES: [&str; 5] = [
    "The date palm is an upright, evergreen tree growing up to 35 m (110 feet) tall (5).",
    "After the first 6 to 16 years, they may produce offshoots at the base of the trunk and grow shorter than 35 m, they typically have a single trunk in cultivation (3, 5).",
    A1_TRUNK,
    A1_FROND,
    "The leaflets are \u{bd} m (18 inches) long (1, 3, 5) and are induplicate, meaning they appear to have been folded in half lengthwise (5).",
];
const A1_TRUNK: &str = "The trunk has a characteristic diamond pattern and is rough gray and up to \u{bd} m (18 inches) in diameter (1, 3).";
const A1_FROND: &str = "Leaves resemble a \u{2018}feather-duster\u{2019} as fronds are pinnately compound, 5-7 m (15 to 20 feet) long, with armed petioles and stout midribs that hold the slender pinnae (1, 5).";

/// Jev as the test reads A1: the trunk sentence states a grey, rough bark;
/// the frond sentence is misread as a grey-green upper face, the kind of
/// placement the live run made on F1's history. Every other sentence
/// states no level. Verify's support question is answered from its
/// labelled cases; a measurement question on an appearance value panics.
#[derive(Default)]
struct Reader {
    asked: Mutex<Vec<Value>>,
}

impl Reader {
    fn level(trait_name: &str, sentence: &str) -> Option<&'static str> {
        match trait_name {
            "bark_colour" if sentence.contains("rough gray") => Some("grey"),
            "bark_roughness" if sentence.contains("rough gray") => Some("rough"),
            "leaf_front_colour" if sentence.contains("feather-duster") => Some("grey_green"),
            _ => None,
        }
    }
}

impl Transport for Reader {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let questions = &body["questions"];
        assert!(
            questions.get("measurement_not_invention").is_none(),
            "an appearance value is no measurement: {}",
            body["state"]
        );
        let answers = if questions.get("level").is_some() {
            self.asked.lock().unwrap().push(body["state"].clone());
            let name = body["state"]["trait"].as_str().unwrap();
            let sentences = body["state"]["sentences"].as_array().unwrap();
            let read = sentences
                .iter()
                .find_map(|s| Reader::level(name, s.as_str().unwrap()));
            let levels = table().levels(name).unwrap();
            let index = read
                .and_then(|key| levels.iter().position(|l| l.key == key))
                .unwrap_or(levels.len());
            json!({"level": {"type": "score", "score": index as f64, "confidence": 0.9, "probabilities": {}}})
        } else if questions.get("relation").is_some() {
            json!({"relation": {"type": "choice", "choice": "supports", "confidence": 0.95, "probabilities": {"supports": 0.95}}})
        } else {
            return CaseTransport.send(request);
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

/// The palm's manifest at version 2 with A1 its one source.
fn manifest() -> Value {
    let form = &table().growth_forms["palm"];
    let fields: Vec<Value> = form
        .fields
        .iter()
        .map(|(name, bar)| json!({"field": name, "condition": "open_grown", "required_ages_years": [20.0], "bar": bar.key(), "question": "Which span states this size?"}))
        .collect();
    let appearance: Vec<Value> = form
        .appearance
        .iter()
        .map(|name| json!({"trait_name": name, "sources": ["A1"]}))
        .collect();
    json!({
        "schema": "manifest", "schema_version": 2, "species": "date-palm",
        "taxon": {"scientific_name": "Phoenix dactylifera", "common_name": "Date palm", "rank": "species"},
        "context": "mature open-grown", "growth_form": "palm",
        "preset": "date-palm", "profile_id": "date-palm", "seed": 7,
        "sources": [{"id": "A1", "url": A1_URL, "title": "UA Campus Arboretum", "rights": "cited"}],
        "fields": fields, "appearance": appearance,
        "versions": {"question_sets": {"described": 1, "obligations": 1}, "tools": {}},
        "model": "jev-latest"
    })
}

/// A1 fetched through the fixture adapter, and empty screen and quality
/// artifacts: this select fills no measured field, only appearance.
fn fetched() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "jev-appearance-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let (dir, fixtures) = (root.join("pipeline"), root.join("fixtures"));
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    let page = format!("{A1_NAV} {}\n", A1_SENTENCES.join(" "));
    fs::write(fixtures.join("a1.md"), &page).unwrap();
    fs::write(
        fixtures.join("a1.html"),
        format!("<html><body>{page}</body></html>"),
    )
    .unwrap();
    write_canonical(
        &fixtures.join("index.json"),
        &json!({"scrape": {A1_URL: {"final_url": A1_URL, "content_type": "text/html", "raw": "a1.html", "markdown": "a1.md"}}}),
    )
    .unwrap();
    write_canonical(&dir.join("manifest.json"), &manifest()).unwrap();
    let paths = Paths::new(&dir);
    fetch::run(&paths, &FixtureAdapter::new(fixtures)).unwrap();
    for (stage, body) in [
        ("screen", json!({"rows": []})),
        ("quality", json!({"fields": {}})),
    ] {
        let (ctx, _) = Context::open(&paths, stage).unwrap();
        ctx.write(&ctx.header(stage, stage, inputs(&[]), vec![]), body)
            .unwrap();
    }
    dir
}

fn judge(reader: &Reader) -> Judge<'_> {
    Judge {
        transport: reader,
        key: "test-key",
        ledger_dir: ledger_dir("appearance"),
    }
}

/// R2: each sentence of A1 that names the bark is judged alone, and the
/// bark's colour is read from "rough gray" and cites that sentence.
#[test]
fn bark_colour_is_read_from_a1s_trunk_sentence_and_cites_it() {
    let dir = fetched();
    let reader = Reader::default();
    select::run(&Paths::new(&dir), &judge(&reader)).unwrap();

    let body = &read_json(&dir.join("select.json")).unwrap()["body"]["appearance"];
    for (name, level) in [("bark_colour", "grey"), ("bark_roughness", "rough")] {
        assert_eq!(body[name]["level"], level, "{name}");
        assert_eq!(body[name]["source"], "A1", "{name}");
        assert_eq!(body[name]["sentence"], A1_TRUNK, "{name}");
    }
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    assert_eq!(
        sidecar["entries"]["/profiles/0/appearance/bark_colour"]["span"],
        A1_TRUNK
    );
    for state in reader.asked.lock().unwrap().iter() {
        let sentences = state["sentences"].as_array().unwrap();
        assert_eq!(sentences.len(), 1, "one sentence per judgment: {state}");
        assert!(
            !sentences[0].as_str().unwrap().contains("](http"),
            "{state}"
        );
    }
}

/// R3: verify asks each appearance value whether its cited sentence
/// describes its level, never whether it is a measurement. The trunk
/// sentence holds; the frond sentence read as grey-green does not, and
/// files a claim decision.
#[test]
fn verify_holds_a_supported_appearance_value_and_files_a_claim_on_one_that_is_not() {
    let dir = fetched();
    let reader = Reader::default();
    let paths = Paths::new(&dir);
    select::run(&paths, &judge(&reader)).unwrap();
    let verify::Outcome::Ran { decisions } = verify::run(&paths, &judge(&reader)).unwrap() else {
        panic!("verify ran")
    };
    let front = "/profiles/0/appearance/leaf_front_colour";
    assert_eq!(
        decisions,
        [format!("date-palm/verify/claim-unsupported/{front}")]
    );
    let obligations = &read_json(&dir.join("verify.json")).unwrap()["body"]["obligations"];
    let held = |pointer: &str| {
        obligations
            .as_array()
            .unwrap()
            .iter()
            .find(|o| o["pointer"] == pointer)
            .map(|o| (o["obligation"].clone(), o["held"].clone()))
    };
    for pointer in [
        "/profiles/0/appearance/bark_colour",
        "/profiles/0/appearance/bark_roughness",
    ] {
        assert_eq!(
            held(pointer),
            Some((json!("appearance_supported"), json!(true)))
        );
    }
    assert_eq!(
        held(front),
        Some((json!("appearance_supported"), json!(false)))
    );
}
