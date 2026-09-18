---
satisfies: [R1, R2, R3, R4]
---
# fn-55-matte-not-plastic-a-physical-highlight.1 Implement Matte, not plastic: a physical highlight and a pixel grain

## Description
The physical highlight and the grain rows, implemented in-host (the routed
gpt-6-astra bridge refused on quota). Report: `.flow/evidence/fn55/REPORT.md`.

## Resolution
Owner, 2026-09-18: the relief's drift at distance is a real defect seen
before fn-55 and is not fn-55's to fix. The 4x distance-series bound is
recalibrated from 3.0 to 3.25 with the reason beside it (the old sheen's
tone-curve compression; base 2.904, after 3.134, no highlight 3.167); the
2x and p95 bounds stand; fn-71 fixes the drift and restores 3.0. The bark
grain rows stay held off in the tables, their value set in fn-71.

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
One Fresnel-weighted, hemisphere-normalised highlight per material with its energy taken from the diffuse, and footprint-faded grain rows on bark and leaf; the frame cost recorded beside fn-52's and the beech and birch close-ups rendered again (`.flow/evidence/fn55/REPORT.md`). The owner recalibrated the 4x distance-series bound to 3.25 for the old sheen's tone-curve compression and moved the relief's drift and the bark grain's value to fn-71; the bark tables hold their grain off until then.

stage: impl-review - skipped(config: REVIEW_MODE=none)
stage: implement - skipped(reach: gpt-6-astra unreachable, codex usage limit until 2026-09-19 16:27; session model used)
## Evidence
- Commits: b143f7dd24d878dc4b7b6e0451d6ae223532fa72, 45b59852ecfc9424b1f35b4aae5ee61eadeb59d6
- Tests: baseline: green (cargo test --release --workspace; npm test; npm run typecheck, all exit 0 pre-edit), cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release --workspace (81 suites ok, exit 0; bark_distance 2x 2.463 <= 3.0, 4x 3.134 <= 3.25 recalibrated by the owner), GREEN_RECEIPT: .flow/tmp/green-receipts/45b59852-unittest.json, npm test (85 passed), npm run typecheck, headless --timing x9 base and x9 after under .flow/evidence/fn55/timing, node tests/species.mjs --profiles .flow/evidence/fn34/profiles.json --quick european-beech|silver-birch
- PRs: