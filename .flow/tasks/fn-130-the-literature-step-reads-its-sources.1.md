---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-130-the-literature-step-reads-its-sources.1 Implement fn-130-the-literature-step-reads-its-sources

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The literature step now reads its sources whole. Stages read markdown as markdown, and HTML is converted once where it enters (`html::source_text`). One sentence splitter serves every split: a terminator a digit follows, or one inside a unit quantity, never ends a sentence. Fetch refuses an empty, interstitial or sliver scrape and falls back on the raw body's conversion. It files unavailable-source for each unreadable source (adapter error, checksum mismatch, PDF parse failure, no usable content) and fetches the rest. A missing scrape status counts as a failure, and a retry or replacement that fails again reopens its decision. Screen judges exactly extract.json's candidates.

- R1: `tests/literature_text.rs::a_p_value_deletes_nothing_and_m1s_leaflet_sentence_is_a_candidate`
- R2: `::the_fronds_width_stays_in_the_fronds_sentence`
- R3: `::a_cookie_wall_is_refused_and_a_source_with_no_usable_content_files_unavailable`, `::a_pdf_parse_failure_files_unavailable_source_and_the_rest_are_fetched`, `::a_retry_that_fails_again_reopens_the_decision`, `::a_scrape_with_no_status_is_not_a_success`
- R4: `::screen_judges_exactly_the_candidates_extract_json_holds`
- R5: gate 986 passed, 1 failed, 21 ignored. The failing test is `objectives::the_palm_s_proposed_approval_is_the_owner_s_priorities_then_every_drawable_trait`. It uses a fixed `/tmp/fn119-objectives` path, this diff does not touch it, and it passed when rerun alone. It is logged in FRICTION.md. The gate is not green on a single run.

Declared behaviour changes: fetch no longer stops at the first unreadable source, so two `pipeline_fixes` tests now assert the continue-and-reopen behaviour. `SourceRef.bytes` in screen now comes from the recorded `markdown_bytes`.

Read-only probe of the live palm cache: P7 and P8 are refused as cookie walls and read from their raw bodies (49 KB and 36 KB of text, 6 and 37 candidates). M1 gives 7 candidates, up from 6, and P6 gives 6, up from 4.

Follow-up (not built): markdown paragraph breaks as sentence boundaries. Navigation-heavy pages still merge link runs that carry no period.

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: session (host-settled design)
## Evidence
- Commits: 4ea7a13c
- Tests: cargo test --profile ci --workspace --no-fail-fast (986 passed, 1 failed: objectives temp-path collision, fixed in 4ea7a13c)
- PRs: