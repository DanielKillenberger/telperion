# The literature step reads its sources whole

## Conversation Evidence

> host audit of the literature step against the palm's live run, 2026-09-23, after a fourth defect in one step tripped the host's convergence guard (fn-80 ECONOMICS.md).
> user (2026-09-23): "yes pls fix the frictions if they're obvious." / "just to be sure we're not having runaway specs here that circle or smth?"

## Goal & Context
<!-- scope: business -->

The palm's literature step has been reading a fraction of what its sources say. Two of the admitted scientific papers were a 196-byte cookie wall; half and more of two others were deleted by a tag stripper; and sentences were cut at decimal points, which put the frond's width into the leaflet's. Every later stage judged what was left. This spec makes the text the stages read the text the source holds; fn-131 fixes how it is judged and gated. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked by the host's audit on 2026-09-23 on the fn-80 branch (file:line and artifacts under `.flow/evidence/date-palm/pipeline/`).** [checked]
  - `extract.rs:68-83` `visible_text` strips HTML tags from markdown; called from `stages/extract.rs:35`, `screen.rs:43`, `stages/appearance.rs:114`, `cite.rs:222`. `cache/M1.md` loses 42 KB (53%) after "(_p_ < 0.001)"; `cache/P6.md` loses 115 KB (73%) after "(_p_ < 0.05)"; M1's leaflet sentence (line 214, length 34.5–62.33 cm, width 2.87–4.9 cm) never reaches `extract.json`.
  - `extract.rs:177-184` splits sentences at every `.`, `!`, `?`: P5's "18-20 ft (5.5-6.1 m) long by 2 ft (0.6 m) wide" became "1 m) long by 2 ft (0.6 m) wide.", filed as `leaflet_width_m` 0.6 m.
  - `adapter/firecrawl.rs:92-109`: P7 and P8's raw GET holds the whole article (196 KB) while the scrape's markdown is the cookie wall; the two are never compared. No empty or interstitial check on the scrape path; a missing `statusCode` reads as 200 (`:276-279`); a PDF parse failure stops the run with no decision (`fetch.rs:150-155`); `unavailable-source` is filed only on an adapter error or a checksum mismatch (`fetch.rs:85`, `:100`); the first failing source stops the loop; a failed `retry` re-files with the same inputs and is skipped (`decision.rs:162-166`).
  - `screen.rs:50` re-extracts instead of reading `extract.json`, so the artifact can disagree with what screen judged.
- **Shape.** [host design]
  - *Markdown is not HTML.* Stages read markdown as markdown: no tag stripping on it; HTML is converted once, where it enters.
  - *Sentences.* A period followed by a digit, or inside a number or unit, never ends a sentence; one splitter, shared.
  - *Fetch refuses what is not the source.* The scrape's markdown is checked against the raw body (size ratio) and a list of known interstitials (cookie walls, bot checks, login pages); on a failure the fetch tries the raw body's own conversion, then files `unavailable-source` for that source and continues with the rest; a PDF parse failure files `unavailable-source` too; a missing status is not success; a retry that fails again reopens.
  - *One reading.* `screen` reads `extract.json`'s candidates; nothing re-extracts.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test on M1's and P6's cached text: after the fix the "<" in a p-value deletes nothing and M1's leaflet sentence is a candidate. [inferred]
- **R2:** A red test on P5's sentence: the frond's "(0.6 m) wide" stays in the frond's sentence. [inferred]
- **R3:** A red test on P7 and P8's cached pages: the cookie wall is refused, the raw body's conversion is used, and a source with no usable content files `unavailable-source` while the others are still fetched. [inferred]
- **R4:** `screen` judges exactly `extract.json`'s candidates. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- Judgment, selection and gating are fn-131's. [inferred]
- No live run here; the host reruns the palm. [inferred]

## Decision Context

- The host's convergence guard (fn-80) required one whole-step review before more patches; this spec and fn-131 are its outcome. [paraphrase]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
