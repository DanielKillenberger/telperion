# The literature step reads its sources whole

## Goal

The palm's literature step read a fraction of its sources: two admitted papers were a 196-byte cookie wall, a tag stripper deleted 53% of M1 and 73% of P6 after a "p <" value, and sentences split at decimal points put the frond's width into the leaflet's. After a fourth defect in one step tripped the host's convergence guard (fn-80), a whole-step audit produced this spec (text in) and fn-131 (judging and gating). Owner (2026-09-23): "yes pls fix the frictions if they're obvious." / "just to be sure we're not having runaway specs here that circle or smth?"

## Decisions

Host design (2026-09-23):
- Markdown is read as markdown; HTML is converted once where it enters (`html::source_text`).
- One shared sentence splitter: a terminator followed by a digit, or inside a number or unit, never ends a sentence.
- Fetch refuses empty, interstitial (cookie wall, bot check, login) or sliver scrapes against the raw body, falls back on the raw body's conversion, files `unavailable-source` per unreadable source (incl. PDF parse failure) and continues; a missing status is not success; a failed retry reopens.
- `screen` reads `extract.json`'s candidates; nothing re-extracts.

## Acceptance Criteria

- **R1:** A p-value's "<" deletes nothing; M1's leaflet sentence is a candidate — met.
- **R2:** P5's "(0.6 m) wide" stays in the frond's sentence — met.
- **R3:** P7/P8's cookie wall refused, raw conversion used, unusable sources file `unavailable-source` while others fetch — met.
- **R4:** `screen` judges exactly `extract.json`'s candidates — met.
- **R5:** Gate green — met after fixing an unrelated temp-path collision in the objectives test (986 passed, the one failure fixed in `4ea7a13c`).

## Result

Landed in `4ea7a13c`; carried by the fn-149 rewrite (#121). Probe of the live palm cache: P7 and P8 read from raw bodies (49 KB and 36 KB, 6 and 37 candidates); M1 7 candidates (was 6), P6 6 (was 4).

## Open

- Markdown paragraph breaks as sentence boundaries (navigation-heavy pages still merge link runs).
