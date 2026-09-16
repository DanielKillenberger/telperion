---
satisfies: [R1, R2, R3, R4]
---
# fn-51-the-curtain-hangs-below-the-crown.1 Implement The curtain hangs below the crown

## Description
The birch's curtain ended on the shell's lower surface, so its hem was the
shell. The owner agreed the curtain should hang below the crown ("makes
sense"). This task adds `curtainDrop` (0 to 1, neutral 0) and
`curtainClearance` (0 to 5 m, never above the crown base): a hanging shoot
may fall past the shell's lower surface into a band down to the clearance,
through `Curtain::admits` in place of the shell check in advance.rs and
planner.rs. The birch states drop 1 to a 1.8 m clearance, its crown base;
a 1.0 m clearance hid the stems behind one wall of curtain.

## Acceptance
- [x] **R1** both rows railed and refused by name, on the wire, blended,
      on the harness, in the regenerated metadata; every other preset
      byte-identical, the birch re-pinned with the reason.
- [x] **R2** hanging shoots fall into the band and nothing else leaves the
      shell; the containment tests in species.rs, outline.rs, growth.rs and
      the monthly boundary test assert the new invariant, the old one at
      drop 0.
- [x] **R3** round 9 rendered and recorded in REPORT.md and
      `round9-fn51/stills.json`; 48/48 protocol, heaviest birch seed
      121,765 nodes.
- [x] **R4** `tests/drop.rs` (eight tests), a unit test in pendant.rs, the
      rails in pendulous.rs.

## NEEDS_HUMAN — the owner's verdict on the combined round

Commits c6f34bbf, b88818b6. Gates green: core (274), render (99), clippy,
typecheck, wasm, compare self-test, harness vitest. The host read S-WHOLE:
the curtain falls past the old round bottom into a skirt with the two stems
clear below, and the ball on a stick is gone. Still wrong: the silhouette is
symmetric, and the strands below the crown fill the width as one wall where
the photograph hangs in separate fringes per limb with gaps between; the
worker found no row that reaches the fringes. The white shoots show because
this branch predates fn-46's colour. The verdict comes on the merged round.

## Done summary
A curtain drop row lets hanging shoots fall below the shell to a clearance above the ground; the birch's skirt hangs below its crown. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.
## Evidence
- Commits: bb2889d4
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, npm run typecheck, npm run rust:test:wasm, npm test, uv run scripts/compare-references.py --self-test, node tests/species.mjs --measure-only (48 cases, measure/protocol-round25)
- PRs: