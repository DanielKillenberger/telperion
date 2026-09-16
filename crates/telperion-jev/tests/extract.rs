use telperion_jev::extract::{
    candidate_sentences, candidate_spans, section_for_terms, split_for_state, visible_text,
    STATE_SENTENCE_LIMIT,
};
use telperion_jev::questions::{screen_cases, selection_cases};
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
fn long_sentence_splits_with_a_bounded_neighbourhood() {
    let mut sentence = String::from("Start. ");
    while sentence.len() <= STATE_SENTENCE_LIMIT {
        sentence.push_str("A measured 12 ft increment appears again. ");
    }
    let parts = split_for_state(&sentence);
    assert!(parts.len() > 1, "{}", parts.len());
    for (part, context) in &parts {
        assert!(part.len() <= STATE_SENTENCE_LIMIT);
        assert!(context.len() <= STATE_SENTENCE_LIMIT);
        assert!(
            context.contains(part.trim()) || part.chars().take(24).all(|ch| context.contains(ch)),
            "context should be a neighbourhood of the part"
        );
        assert!(context.len() < sentence.len());
    }
}

#[test]
fn unicode_offsets_stay_on_char_boundaries() {
    let mut text = String::from("°");
    text.push_str(&"x".repeat(319));
    text.push_str("50 to 90 ft tall at the café — 12 m.");
    let sentences = candidate_sentences(&text);
    assert!(
        sentences
            .iter()
            .any(|row| row.sentence.contains("50 to 90 ft")),
        "{sentences:?}"
    );
    let spans = candidate_spans(&text);
    assert!(
        spans.iter().any(|span| span.contains("50 to 90 ft")),
        "{spans:?}"
    );
    let section = section_for_terms(&text, &["café".into(), "tall".into()], 17);
    assert!(section.is_some());

    let o1 = screen_cases()
        .into_iter()
        .find(|case| case.id == "o1")
        .expect("o1");
    let o1_context = o1.context.as_deref().unwrap_or("");
    assert!(
        o1_context.contains('°') || o1_context.contains('F'),
        "labelled OWIC context should carry the degree-Fahrenheit run"
    );
    let _ = candidate_sentences(o1_context);
    let _ = section_for_terms(o1_context, &["height".into(), "growth".into()], 17);

    let mut long = String::from("a");
    while long.len() <= STATE_SENTENCE_LIMIT {
        long.push('é');
    }
    long.push_str(" A measured 12 ft increment.");
    let parts = split_for_state(&long);
    assert!(!parts.is_empty());
    for (part, context) in &parts {
        assert!(part.len() <= STATE_SENTENCE_LIMIT);
        assert!(context.len() <= STATE_SENTENCE_LIMIT);
        assert!(part.is_char_boundary(part.len()));
        assert!(context.is_char_boundary(context.len()));
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
