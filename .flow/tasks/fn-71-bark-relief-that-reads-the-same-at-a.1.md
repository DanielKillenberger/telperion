---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-71-bark-relief-that-reads-the-same-at-a.1 Implement Bark relief that reads the same at a distance

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Round 4, the cost of the picture the owner accepted, with no fade returning and nothing traded for it: the lenticel groove is read once a fragment at the pixel's footprint and shared by every cell whose footprint is over twice its depth, the dash window reads exactly the rows and arc cells a box can touch instead of three or five rows, the grain's box window is capped at three cells against round three's six, and the shading cells are sized per axis, two or three on each, so a grazing pixel takes its cells only along its length and four a side are gone. Frame cost on the three hero frames, seed 7, 1600 by 1000, three rounds each (`timing/round4-*`): oak 8.91, birch 7.00, beech 14.59 ms p50 against round three's 10.0, 15.0, 19.6 and round two's target of 5.9, 4.0, 11.4; what holds the remaining 3 ms per preset is named term by term in `.flow/evidence/fn71/REPORT.md`, "Round 4" - the oak's plate network read nine to sixteen times a fragment in exactly the band the owner asked to keep, the birch's dashes and grain box on every branch wider than two pixels - and each is left standing rather than bought back with a fade. The invariant holds on the real artifact: in `sweep/res8.md` the widest step between adjacent factors is 0.030, on the beech's 2px band from 1.5 to 2, which is round three's own widest step in the same cell, and the curve moves from round three in five of its seventy-two cells by 0.01 each. Every resolution receipt is green and reproduced by this session's test run to the same values (oak 1x 1.78, 2x 2.23, 4x 2.84, beech 1.06, birch 2.54, spruce and oak grazing 2.70 and 2.89), the redraw pins hold, and the five gates are green with only master's three inherited fmt diffs. The report's two unproven claims from the interrupted round were corrected against what this run measured: the sweep is round three's curve to 0.01, not to the second decimal everywhere, and the beech receipt reads 1.06. The harness on 5174 runs from this worktree with its browser modules rebuilt.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: d91434117d72ef85905d4b05689326c93b3e3cb6
- Tests: cargo fmt --all -- --check (green but for the three files master's fn-58 tooling commit left unformatted: crates/telperion-core/examples/species_measure.rs, crates/telperion-core/src/params/tests.rs, crates/telperion-jev/src/bin/jev.rs, untouched by this branch), cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm test, npm run typecheck
- PRs: