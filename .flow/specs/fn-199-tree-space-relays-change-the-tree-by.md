# Tree space: relays change the tree by degree

## Goal

Phase B claims every setting changes the tree by degree (`docs/tree-space.md`). fn-193's Codex re-review found four places where Troll's relays make a walked setting jump; they were accepted for phase C because the beech at its passed values did not depend on them. Production (fn-198) must not ship a continuity claim with known breaks, so this spec closed them first (host, 2026-10-04).

## Decisions

- Host, 2026-10-07, R1: option (b), continuous time. A failing unit grows the share of its year the apex lived (`ln u / ln viability`, fixed-point `ln` on the contract's `log2`, mirrored in WGSL); its relay grows the rest of the cycle.
- Host, 2026-10-07: R4's matching sag-load fix stays, one rule for girth and load.
- Host, second round: the closed form models the lived share by quadrature with the fewest points that pass; the predictor must stay in milliseconds, so one point (A1) plus the nesting cut (A3), adopted. B2: a relay's first-unit thresholds read the unrounded lived share (Q32, in the contract). Epitony checked two ways (relays after a whole unit rise >= 0.05; failed units' relays do not fall) rather than loosening the threshold.
- Host, "one rule for carried-on axes": Rule 1, a continuation is a relay at blend 0 (one function lays, ages, frames and straightens both). Rule 2 (R3 weight on the carried-on axis) withdrawn for Rule 2': `relay_point` blends from the last unit back by its grown extent along the whole line of carried-on axes. Moving relays on partial last units is accepted.
- C1: the lived part takes the unit's abortion draw.

## Acceptance Criteria

- **R1:** With relay probability 1, crossing the survival setting changes growth by degree — met (decision (b), A1/A3, B2, C1).
- **R2:** A relay's position is continuous in the parent's grown length — met by fn-206 (`b172c2d4`).
- **R3:** A relay barely off a bent lateral keeps the parent's straightening, blended by presence — met (Rules 1 and 2').
- **R4:** A relay's girth (and sag load) is spread over the nodes it spans — met (`56e431d8`).
- **R5:** Each fix has a walk test red on the base, green after, bound 30; A's oracle and B's walks green — met.
- **R6:** The beech re-rendered at five seeds and viewed against the base — met (host viewed the first sheets, worker the final one).

## Result

- Walk tests in `crates/telperion-space/tests/relays.rs`; census over seeds 1 to 50 shows no R1 or R3 jump. Every rule mirrored on the GPU (`stand.wgsl`); realized GPU tests pass.
- Two defects found on the way and fixed: the phyllotaxis phase was added whole at any blend (now `(1 − blend) × (parent phase + divergence × rank)`, `e314a066`), which had pushed the light walk to 32.6 > 30 (now 26.82 / 29.07); and Rule 2' blended frames were not square (`stand::square`), exposed after merging F4's tip (`78cc3fbe`).
- Rows re-pinned once (beech, oak); spruce and palm unchanged to the bit.
- R6 (80 years, seeds 1, 2, 3, 4, 7): every tree reads as an open-grown beech, no defect; base's long near-level low limbs shorter; heights 17.8 to 19.0 m against 17.8 to 18.8 m on the base. Sheets were in ignored `raw/`.
- Gate green on `d3bb6617`: render 163, rest 994.

## Open

`generation_limit_guard` failed on the base `e9bb5b08` too; it passed after F4's merge. None left in scope.

## Friction

- Whole workspace gate held the shared GPU lock, queuing GPU checks 15-20 min (twice) -> proposal: lock only GPU test binaries.
- Cited Codex findings lived in an ignored fan-out dir -> specs citing reviews should copy the worked examples.
- `/tmp` 16 GB tmpfs filled by base checkouts and other sessions -> local setup, reported only; base checkouts belong under ignored `raw/`.
- `tests/walks.rs` takes 17 min with no single-walk filter -> split or filter the walks binary.
- Quadrature timing grew without bound past two points -> time one point count first.
- `git archive | tar -x` kept old mtimes so cargo did not rebuild during a bisect -> use `tar -x -m`.
- Inherited red `generation_limit_guard` on the base cost a base run -> fix on master.
