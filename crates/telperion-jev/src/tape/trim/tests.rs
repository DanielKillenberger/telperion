use std::collections::BTreeSet;

use super::page;
use crate::extract::candidate_sentences;

const URL: &str = "https://example.test/beech";

/// fn-157: a fetched page keeps the sentences the read stage asked, each
/// with the context a label asked it in, and drops the candidate
/// sentences it never asked; a sentence only a document's kind was asked
/// over is kept alone.
#[test]
fn a_page_keeps_the_sentences_read_asked_and_nothing_else() {
    let filler = "Beech woods are shaded and quiet in every season. ".repeat(12);
    let text = format!(
        "The nuts are 2 cm across and fall in autumn. {filler}\
         The tree reaches 30 m tall in parks and gardens across Europe."
    );
    let text = text.as_str();
    let asked = candidate_sentences(text)
        .into_iter()
        .find(|c| c.sentence.contains("30 m"))
        .unwrap();
    let quotes: BTreeSet<String> = [asked.sentence.clone(), asked.context.clone()].into();
    let (trimmed, _) = page(URL, "text/markdown", text, &[], &quotes, true).unwrap();
    assert!(trimmed.contains("reaches 30 m tall"), "{trimmed}");
    assert!(!trimmed.contains("nuts"), "{trimmed}");
    let bare: BTreeSet<String> = [asked.sentence].into();
    let (alone, _) = page(URL, "text/markdown", text, &[], &bare, true).unwrap();
    assert!(alone.contains("reaches 30 m tall"), "{alone}");
    assert!(!alone.contains("shaded"), "{alone}");
}

/// fn-157, the oak's replay: a document's kind and an appearance level
/// are asked over sentences that may be a word or two (an address cut at
/// its full stop). Each is quoted whatever its length, or the trimmed
/// page asks over other sentences.
#[test]
fn a_short_sentence_a_request_asked_over_is_kept() {
    let tape = std::env::temp_dir().join(format!("trim-kind-{}", std::process::id()));
    std::fs::create_dir_all(tape.join("jev")).unwrap();
    let entry = serde_json::json!({"request": {"body": {"state": {
        "passages": ["org%2Fportal%2Ftaxa%2Findex."],
        "sentences": ["cad=4) [bark](https://books."]}}}});
    std::fs::write(tape.join("jev/a.json"), entry.to_string()).unwrap();
    let quoted = super::quoted(&tape).unwrap();
    assert!(
        quoted.contains("org%2Fportal%2Ftaxa%2Findex."),
        "{quoted:?}"
    );
    assert!(
        quoted.contains("cad=4) [bark](https://books."),
        "{quoted:?}"
    );
}

/// fn-157, the oak's replay: a page fetch dropped live was never read,
/// and keeps nothing, not even its licence tags; kept, the tags alone
/// read as a page and the replay asks its kind. A page read where it
/// landed after a redirect keeps its tags.
#[test]
fn a_page_the_run_never_read_keeps_nothing() {
    let tape = std::env::temp_dir().join(format!("trim-unread-{}", std::process::id()));
    std::fs::create_dir_all(tape.join("firecrawl")).unwrap();
    let key = "a".repeat(64);
    let entry = serde_json::json!({"key": key,
        "request": {"op": "scrape", "url": "https://video.test/watch"},
        "response": {"ok": {"content_type": "text/html", "markdown": "A video page of 3 m trees."}}});
    let path = tape.join(format!("firecrawl/{}.json", &key[..32]));
    std::fs::write(&path, entry.to_string()).unwrap();
    let raw =
        r#"<html><link rel="license" href="https://creativecommons.org/licenses/by/3.0/"></html>"#;
    std::fs::write(path.with_extension("bin"), raw).unwrap();
    super::tape(&tape, &[]).unwrap();
    let after: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(after["response"]["ok"]["markdown"], "");
    assert!(!path.with_extension("bin").exists());

    let mut landed = entry.clone();
    landed["response"]["ok"]["final_url"] = "https://video.test/landed".into();
    std::fs::write(&path, landed.to_string()).unwrap();
    std::fs::write(path.with_extension("bin"), raw).unwrap();
    let kind = serde_json::json!({"request": {"body": {"state": {
        "source": {"url": "https://video.test/landed"}, "passages": []}}}});
    std::fs::create_dir_all(tape.join("jev")).unwrap();
    std::fs::write(tape.join("jev/k.json"), kind.to_string()).unwrap();
    super::tape(&tape, &[]).unwrap();
    assert!(path.with_extension("bin").exists());
}

/// fn-157: a PDF's text is the parse Firecrawl returned, recorded under
/// its own entry. A page not openly licensed keeps only the parse's
/// quoted passages, and its scrape only the PDF's magic, so a replay
/// still takes the body for a PDF and asks for the parse.
#[test]
fn a_pdf_keeps_its_magic_and_its_parse_only_the_quoted_passages() {
    let tape = std::env::temp_dir().join(format!("trim-pdf-{}", std::process::id()));
    std::fs::create_dir_all(tape.join("firecrawl")).unwrap();
    std::fs::create_dir_all(tape.join("jev")).unwrap();
    let url = "https://archive.test/guideline.pdf";
    let (scrape, parse) = ("b".repeat(64), "c".repeat(64));
    let at = |key: &str| tape.join(format!("firecrawl/{}.json", &key[..32]));
    let entry = serde_json::json!({"key": scrape,
        "request": {"op": "scrape", "url": url},
        "response": {"ok": {"content_type": "text/html", "markdown": ""}}});
    std::fs::write(at(&scrape), entry.to_string()).unwrap();
    std::fs::write(at(&scrape).with_extension("bin"), b"%PDF-1.4 binary body").unwrap();
    let parsed = "Beech grows to 30 m tall in its native woods. The guideline is not open.";
    let entry = serde_json::json!({"key": parse,
        "request": {"op": "parse", "file": "P6.pdf"}, "response": {"ok": parsed}});
    std::fs::write(at(&parse), entry.to_string()).unwrap();
    let kind = serde_json::json!({"request": {"body": {"state": {
        "source": {"url": url},
        "passages": ["Beech grows to 30 m tall in its native woods."]}}}});
    std::fs::write(tape.join("jev/k.json"), kind.to_string()).unwrap();
    super::tape(&tape, &[]).unwrap();
    let bytes = std::fs::read(at(&scrape).with_extension("bin")).unwrap();
    assert_eq!(bytes, b"%PDF-");
    let after: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(at(&parse)).unwrap()).unwrap();
    let text = after["response"]["ok"].as_str().unwrap();
    assert!(text.contains("30 m tall"), "{text}");
    assert!(!text.contains("not open"), "{text}");
    assert!(super::only_quoted(&tape, &[]).unwrap().is_empty());
}

/// fn-157: a stale answer that rekeys onto a question the recording already
/// answers under its right key is dropped; the answer recorded under the
/// right key is the run's.
#[test]
fn rekey_never_overwrites_an_answer_already_under_its_key() {
    let tape = std::env::temp_dir().join(format!("trim-rekey-{}", std::process::id()));
    std::fs::create_dir_all(tape.join("jev")).unwrap();
    let request = serde_json::json!({"body": {"state": {"q": 1}}});
    let right = crate::tape::entry_key("jev", &request);
    let at = |key: &str| tape.join(format!("jev/{}.json", &key[..32]));
    let current = serde_json::json!({"key": right, "request": request, "response": {"ok": "new"}});
    std::fs::write(at(&right), current.to_string()).unwrap();
    let stale_key = "e".repeat(64);
    let stale =
        serde_json::json!({"key": stale_key, "request": request, "response": {"ok": "old"}});
    std::fs::write(at(&stale_key), stale.to_string()).unwrap();
    super::rekey(&tape).unwrap();
    let kept: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(at(&right)).unwrap()).unwrap();
    assert_eq!(kept["response"]["ok"], "new");
    assert!(!at(&stale_key).exists());
}
