## Goal

A species' starting values should be what its literature agrees on, without a person checking each number. fn-149's per-field search took a 4.36 m trunk and a 25.9 m crown from one heritage-tree page, read a leaf length as its width and left the height unsourced. The owner asked to replace the stage: gather everything written about the species, read each document once, and compose each value in code as the confident aggregate across sources. On 2026-09-26 the spec also took the beech's first live run through Tune and Gaps to the owner's look, and a photograph search that finds the whole tree.

## Decisions

- owner (2026-09-25): "How can it be so hard to gather some data on trees and determine average values"
- owner (2026-09-25): "why are we not searching documentation regarding the species download it all that we can find and compose the highest confidence aggregate from those?"
- owner (2026-09-25): "ok go.."
- owner (2026-09-26), on carrying the beech's live run through Tune, Gaps and the owner's look, and a whole-tree photograph search: "yes"
- Shape: one broad gather per species; Jev labels each candidate span (field, unit, typical or record, age, growing condition, organ) and never picks a source or supplies a number; code aggregates (median of typical mature values from independent sources, spread, agreement count). A record value is the field's maximum only. Reading needs no licence; only open-licence copies go into the catalogue.
- Replaced and deleted: per-field discovery and search-again rounds, admission before reading, quality bars, per-field selection and their decision kinds. Kept: fetch pacing, Wikipedia as a lead, record and replay, preflight, photograph search, the profile's output shape.
- Host decisions (2026-09-26): documents are classed by kind; a value gates only when a flora, forestry or garden tier agrees on it (otherwise contextual); Start takes the median; appearance reads the best tier only; queries include the native range; photographs come from Commons single-tree categories; a found photograph gets a shot chosen from code's candidates; Tune is capped at three rounds; a gap is reachable only when a judged attempt passes the trait.

## Acceptance Criteria

- **R1:** Beech height, trunk diameter, crown width and leaf size from at least three independent sources each, inside flora ranges — partial: height 40 m from three agreeing forestry sources; leaf width 11.4 cm rests on one extension page, outside the flora's 2.5-7 cm (contextual).
- **R2:** A record value is the field's maximum, never its typical value — met (`tests/aggregate.rs`).
- **R3:** Unlicensed sources are read and cited; only open-licence sources are copied — met (`runner::folder`).
- **R4:** Each field records sources, spread and confidence; Start derives the tree with downstream code unchanged — met.
- **R5:** Recorded run replays offline; a shipped species lands within its catalogue ranges — met for the Oregon white oak, though every oak value rests on one source.
- **R6:** Replaced stages and decision kinds deleted, gates green — met.
- **R7:** Reference photographs include a mature open-grown beech in leaf — met ("Fagus sylvatica TK 2023-05-06 1", CC BY-SA 4.0); no bare tree kept.
- **R8:** The beech runs live through Tune and Gaps to the owner's look, replayed offline through Tune's first revision — met (the replay needs a hardware GPU; CI skips).

## Result

- Beech, native-range run: height 40.0 m (30-40, forestry, agreed); crown 12.9 m and leaf length 8.5 cm (extension, contextual); trunk 1.5 m (nursery, thin); crown base unsourced.
- Tune (three-round cap) kept round 2's half-strength bundle: crown 13.4 m to 17.7 m, envelope spread 0.161 to 0.211, habit, twig, bark and shading rows. Round 3 kept nothing.
- Gaps: 0 reachable, 7 identity (3 waiting on fn-61; single clear bole, fine layered twig sprays, light translucent foliage and leaf shape for the host under fn-62), 0 global, 5 unsourced. `gaps.md` is 3.7 KB.
- Owner verdict: the owner does not accept this beech; it waits on fn-61.
- Oak (R5): height 21.3 m, dbh 0.8 m, leaf length 0.10 m, all inside the catalogue; no photograph found.
- Fixtures: beech 447 files, 11 MB; oak 244 files, 1.4 MB; `tape_trim --check` clean. Gates on the rebased head: workspace 1048 passed, `npm test` 110 passed.

## Open

- Non-European whole-tree photographs (the oak kept none): not built.
- Tune's proposal asks about every dial (227): a dial set sized to the species or a question cap (`--init-tuning`, fn-165).
- A harness view of a run's kept tree (captured as fn-166).
- Proposal batches stable under a dial-table reorder.

## Friction

- A changed literature stage cannot replay the old tape -> re-record budgeted into the live run.
- Firecrawl refused past 22 plan credits, R5 blocked for a time -> account credits; `--status` could warn.
- A failed later inner stage reran the paid earlier one; HTTP 520 not retried -> follow-up (refresh per record, retry 5xx).
- An archived PDF served as text/html was read as binary -> fetch takes `%PDF-` bodies as PDF.
- Tune from a name had no profile manifest -> Start writes it (fixed).
- The measurer refused Start's draft profile -> Start writes a frozen ready copy (fixed, 87c38b48).
- A one-source value gated as a point -> host's tier and agreement rule.
- A found photograph had no shot -> host: shot chosen from code's candidates.
- `tape_trim --check` exited 0 on a finding -> exits nonzero (fixed).
- Tune asked Jev about 227 dials in one call (HTTP 400) -> `max_questions_per_call` 32; dial set to fn-165.
- Tune spent about 2.04 M Jev tokens on rounds that could keep nothing -> three-round cap; a same-gate stop proposed.
- `gaps.md` grew to 306 KB and 1.3 MB of repeated links -> one readable line per gap (fixed, 3.7 KB).
- A Tune recording did not replay from another directory (four run-specific request keys) -> each fixed with a test; round 1 re-recorded.
- The harness cannot show the kept tree -> fn-166.
- The rebase onto fn-152 reordered dial batches and missed the tape -> round 1 re-recorded; stable batch keys proposed.
