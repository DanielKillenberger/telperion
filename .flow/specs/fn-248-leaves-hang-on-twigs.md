# Leaves hang on twigs

## Goal

The owner (2026-10-09), looking at the release branch's trees: "I think major problem in terms of looks now is the lack of twigs. All the leaves seem to come from branches and they are directly attached." Then narrowed: "i was talking about telperion and laurelin". On a real tree most leaves sit on the current and last few years' thin shoots and spurs, each on its own petiole. Find why the engine's trees show no twig layer, fix it on every woody species, and prove it on stills and the owner's verdict. Host (2026-10-09): release blocker for fn-237. Scope: the fine wood layer and how leaves attach.

## Decisions

- R1 cause, checked against the code: (1) fn-217's S1 scale multiplied every node's position and radius, the finest order like the trunk, while leaves kept natural size, so the Two Trees' twigs were 2.5× as thick and long (current-year shoot 6.4 mm against the beech's 2.6 mm); (2) leaves stood every 6 mm along all slender wood whatever its age (the beech's `internodeLength` 0.006, set in fn-198 under a stale "5 cm" comment), a solid caterpillar fringe; (3) nothing dropped thin wood; (4) the shoot colour was lost at scale.
- Host decision (2026-10-09, after Astra's review, "Question, delete, then optimise"): fix the natural tree's foliage first: leaves by wood age (current-year wood and spur tips, retention reaching every count), spacing chosen on stills against reference beech shoots (25 mm).
- Host decision (2026-10-09), part 3: the Two Trees grow large through their own coarse ages (`/supernatural/internode`, `/supernatural/pipe` overrides, fn-217's X2 convention) at scale 1 on the beech's ordinary twigs; group off stays the ordinary beech.
- The Two Trees could not reach a beech's leaf area index within the browser's 4,000,000-phytomer ceiling (two buds a node: 5.1M nodes); host chose larger leaves on the Two Trees (`/supernatural/canopy/size`) and florets three times larger and fewer with coverage held.
- Host planning rule: a decision that touches the browser's phytomer bound is made from the preset's own seed, group on, never from `expected_kept`. Telperion set to 82 (3,773,761 kept at seed 42, 5 % under `WASM_KEPT`).

## Acceptance Criteria

- **R1:** for Laurelin and Telperion against the beech, then every woody shipped species at 40 and 80, leaves counted by wood radius and age, thin wood grown and drawn measured, close stills beside real shoots; cause named and checked against the code — met.
- **R2:** leaves read as borne on twigs and petioles at mid and close views on every woody species; by degree; both executors; frame rates held to fn-246's — met (Telperion close 25 fps, below fn-246's 34).
- **R3:** same stills before and after, viewed; the owner's verdict; gate and npm test green — met.

## Result

- Beech short shoots bear 4 leaves and live 4 years; leaves 2.5 cm apart on current-year wood and spur tips. Telperion at 82: seed 1 45.6 m tall, LAI 5.83; seed 42 LAI 6.94; 96.5 fps whole, 25.4 close.
- Owner verdicts (2026-10-09, seen in the harness): Telperion at 82 "sure fine"; flower shape "looks good"; flagged "flowers seem placed randomly and attached to the air" (went to fn-253).
- Gate: core, space, field, wasm, jev green; render green except the cold-compile timing test (25.7 s against 20 s at load 18-21; inconclusive, passed in the full gate). npm test 155. Impl-review SHIP after 2 P2 fixes in the measurement tool.

## Open

- Flower placement: fn-253.
- Review P2 declined: three ages merged across a fractional year (no shipped preset produces it; documented in code).

## Friction

- No public leaf-to-wood record and no wood age on `Tree` -> measured through a dedicated example; no repository remedy recorded.
- `cargo clippy -p telperion-core` fails on the base (26 errors) -> a clean lib clippy (proposed).
- dcg hook blocks `> $VAR` redirects and piped `perl -pe` -> local setup, reported, not specced.
- Cold-compile test fails when the render suite runs in parallel or in the gate -> run it serially (proposed).
- Two WGSL builtin names shadowed, reported only on the GPU -> a naga validation pass in CPU tests (proposed).
- A density trial ran 15 min and was killed -> trials outside the browser need the phytomer ceiling (proposed).
- The Wasm bound was set from a mean, not the preset's tree -> the host's planning rule above.

## Evidence

- `.flow/evidence/fn-248-leaves-hang-on-twigs/TWIGS-after.jpg` (before/after sheet sent to the owner)
- `.flow/evidence/fn-248-leaves-hang-on-twigs/TWIGS-part3.jpg` (the Two Trees grown through their coarse ages, close and mid)
- `.flow/evidence/fn-248-leaves-hang-on-twigs/TWIGS.md` (the full record, kept verbatim in line numbering because fn-237's species audit cites its lines)
- `.flow/evidence/fn-248-leaves-hang-on-twigs/astra/florets-r2.md` (Astra's florets round 2, DISAGREE; cited by fn-237)
- `.flow/evidence/fn-248-leaves-hang-on-twigs/FRICTION.md` (cited by `shader_tests.rs`)
