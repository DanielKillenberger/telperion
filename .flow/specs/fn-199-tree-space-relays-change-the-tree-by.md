# Tree space: relays change the tree by degree

## Goal & Context

Phase B's claim is that every setting changes the tree by degree (`docs/tree-space.md`). Phase C added Troll's relays, and fn-193's Codex re-review found four places where a relay makes a walked setting jump. The four were accepted for C on the host's call, because the beech at its passed values does not depend on them. Production (fn-198) must not ship a continuity claim with known breaks, so this spec closes them first (host, 2026-10-04).

## Requirements

- **R1:** With relay probability 1, crossing the survival setting changes growth by degree, not a whole year at once (`crates/telperion-space/src/grow.rs:156`).
- **R2:** A relay's position along its parent is continuous in the parent's grown length; a node born at near-zero size does not move an existing relay by half a node (`geometry.rs:305`).
- **R3:** On a bent lateral, a relay that has barely left its parent keeps the parent's remaining straightening, blended by its presence (`geometry.rs:55`).
- **R4:** A relay's girth contribution is spread continuously over the parent's nodes it spans, so moving it past a node boundary leaves the parent's radius continuous (`girth.rs:49`).
- **R5:** Each fix has a walk test that is red on the base and green after, within the bound of 30. A's oracle and B's walks stay green.
- **R6:** The beech is re-rendered at its five seeds; the host views the sheet against `raw/final21/five-seeds.png` and records any visible change.

## Acceptance

- [ ] R1 to R4: four walk tests, each red on the base first.
- [ ] R5: crate tests and the workspace gate green.
- [ ] R6: the beech sheet viewed by the host, change recorded in RESULT.md.

## Boundaries

Engine continuity only. No new settings, no species values beyond what R6's re-render needs.

## Sources

Codex re-review findings for fn-193.1 (`.flow/review-fanout/33e47f112c61497c9b41dddd668e36c5/`), each with a worked example.
