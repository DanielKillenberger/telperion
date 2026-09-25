# fn-150-the-slim-field-package-grows-every.1 Slim field grows every preset through the core pipeline

## Description
The spec's R1 to R5 as one task: the slim crate asks the core pipeline for the field, and the tests, npm test and 0.1.4 follow.

## Blocker (NEEDS_HUMAN, 2026-09-25, second)
The host's design is built and green, but its palm field fails the visual check, so R2, R3, R5, 0.1.4, the gate and the PR wait for a host decision. The first blocker, the +48% slim Wasm, is cleared. The work: `plan::supports` accepts a rosette. `foliage/plan/fronds.rs` walks every frond, living and dead, in 3 chords along its rachis, on the same spiral and frame as placement (`rosette::fronds`, shared, placed crown byte-identical at seeds 1, 7, 1407 and 4242). The pipeline compiles without `geometry`: `pipeline/drawn.rs` holds wood, placement and the placed field; `pipeline/planned.rs` refuses them by name. The slim crate is back on `default-features = false`. `telperion-field.wasm` is now 384,761 bytes raw, 137,950 gzip and 113,463 brotli, against 355,100 / 126,663 / 104,737 in 0.1.3 (+29,661 raw). The extra bytes are the skeleton steps (leaf bases 8.9 KB, the stable sort in `stem_apices` 8.5 KB, `clear_apical_twigs` 1.6 KB), the pipeline 4 KB, the frond plan 2.4 KB and function names 2.9 KB; there is no placement code. The plan is conservative: every placed leaflet vertex lies in a planned foliage cell at 0.1, 0.25 and 1 m at all four seeds (`tests/field_fronds.rs`). But on the quarter-metre grid only 81.2 to 81.8% of cells agree with the placed field, and the plan reports 2.82 to 2.87 times the placed foliage cells. The still (`raw/palm-field-seed1.png`, `examples/field_still.rs`) shows the planned palm as a round bush and the placed one as a palm. The cause is the capsule radius: a leaflet reaches 0.6197 × 2.35 × (1 + 0.35) ≈ 1.97 m from the rachis, and a round tube that wide around each rachis cannot be flat. More chords do not help. The main entry's palm field regresses the same way. The host chooses: accept the bush, give the field a flat primitive, or describe fronds per leaflet side or per leaflet.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
