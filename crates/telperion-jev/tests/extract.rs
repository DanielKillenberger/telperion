use telperion_jev::extract::{
    candidate_sentences, candidate_spans, section_for_terms, split_for_state, visible_text,
    STATE_SENTENCE_LIMIT,
};
use telperion_jev::questions::selection_cases;
use telperion_jev::sha256_hex;

#[test]
fn recall_covers_foot_the_pilot_missed() {
    let sentence =
        "Growth during the first 10 years after field planting is relatively slow and 8 to 11 years are required to grow a 6-7 foot tree.";
    let spans = candidate_spans(sentence);
    assert!(
        spans.iter().any(|span| span.contains("6-7 foot")),
        "{spans:?}"
    );
    assert!(spans.iter().any(|span| span.contains("8 to 11 years")));
}

#[test]
fn selection_cases_all_have_their_expected_span_or_none() {
    for case in selection_cases() {
        if case.expect_span == "none" {
            continue;
        }
        let spans = candidate_spans(&case.document);
        assert!(
            spans.iter().any(|span| span == &case.expect_span),
            "{} missing {} in {spans:?}",
            case.id,
            case.expect_span
        );
    }
}

#[test]
fn empty_source_yields_no_sentence() {
    let bytes = b"No quantities live here, only adjectives.";
    let text = visible_text(bytes);
    assert!(candidate_sentences(&text).is_empty());
    assert_eq!(sha256_hex(bytes).len(), 64);
    assert_ne!(sha256_hex(bytes), sha256_hex(b""));
}

#[test]
fn long_sentence_splits_and_keeps_the_whole_as_context() {
    let mut sentence = String::from("Start. ");
    while sentence.len() <= STATE_SENTENCE_LIMIT {
        sentence.push_str("A measured 12 ft increment appears again. ");
    }
    let parts = split_for_state(&sentence);
    assert!(parts.len() > 1, "{}", parts.len());
    for (part, whole) in &parts {
        assert!(part.len() <= STATE_SENTENCE_LIMIT);
        assert_eq!(whole, &sentence);
    }
}

#[test]
fn key_terms_missing_section_is_none() {
    assert!(section_for_terms("only weather notes", &["taproot".into()], 80).is_none());
}

#[test]
fn html_tags_do_not_enter_candidate_text() {
    let html = b"<html><script>50 ft</script><p>Mature oaks are 50 to 90 ft tall.</p></html>";
    let text = visible_text(html);
    assert!(!text.contains("<p>"));
    let sentences = candidate_sentences(&text);
    assert_eq!(sentences.len(), 1);
    assert!(sentences[0].sentence.contains("50 to 90 ft"));
}
