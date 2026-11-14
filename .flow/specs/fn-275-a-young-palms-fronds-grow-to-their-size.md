# A young palm's fronds grow to their size over its establishment

## Goal

The owner (2026-10-14), watching the harness grow the date palm in: "also the date palm starts with full grown leaves". The rosette drew every frond at the mature length from year 1. A palm establishes: its first leaves are small, each new leaf larger, and the stem rises only once the crown is mature. Make frond size, stem girth and rise, and the seedling's entire leaves follow sourced establishment, by degree, on both executors, with the mature palm unchanged. Release item (fn-237), after fn-270.

## Decisions

- Owner (2026-10-14): "also the date palm starts with full grown leaves".
- Host (2026-10-14, R2a): the stem establishes by the same law and rows: girth follows the crown's establishment share and visible elongation waits on it (Tomlinson and Huggett 2012; Broschat, UF/IFAS EP473; FAO 156 ch. I). Rulings: (1) applied on the grown tree in the one conversion (CPU `space.rs`, device `dress.wgsl`), growth engine untouched; (2) option A: each stem phytomer lowered by a closed form of the rise it waited; (3) `/space/ages/2/internode` retuned (0.0339) so the palm at 40 stands as today (21.59 m); (4) no change to the stem's 0.297 m radius in this spec (noted for the species audit).
- Owner (2026-10-14): "ok", to seedling leaves as a release item (R2b: entire leaves for "the first three years after seed germination", FAO 156 ch. I). Host: leaflet count = leafletCount ^ f, no new row; strips of one blade whose tips converge on the blade's tip; the 3-year (FAO) against 6.75-year clock accepted as a known simplification.
- Owner (2026-10-15): "it looks nice other than having the leaves not being connected in a sensible way"; "yea i can see them floating" (R2c, a drawn rachis).
- Host (2026-10-15): a drawn rachis needs the device's frond code moved into the leaf-base stage (about 1,200 lines), too large for the release; instead a young frond's segments crowd toward its base by R2b's course (segment base = f × its station). The drawn rachis moves to fn-283. The remaining spine gap is accepted, closed by fn-283.
- Juvenile leaf form in other species (heteroblasty) is fn-278.

## Acceptance Criteria

- **R1:** sourced establishment, each value checked against its text; stand-ins declared — met (`frondEstablishment` 6.75 years from a 2025 Frontiers tissue-culture table; `frondEarlyShare` 0.1 a declared stand-in from FAO plantlets).
- **R2:** one law: each frond sized by the crown's age at its making along the sourced curve, kept for life; dormant without a rosette; no species branch — met. Includes R2a (stem girth and rise) and R2c (segments crowd toward the base; drawn rachis deferred).
- **R2b:** a seedling's fronds entire, becoming pinnate by the crown's age at making, through fn-277's fractional counts; the palm at 40 byte-equal — met.
- **R3:** the mature palm at its shipped age byte-equal — met (oldest drawn frond made at about year 33, past 6.75).
- **R4:** CPU and device agree; no frond changes size at the year boundary — met (exact at 3 and 4).
- **R5:** stills at 1, 3, 6 and the shipped age and the whole-year grow-in viewed; owner's verdict — met (owner's 2026-10-15 remarks led to R2c).

## Result

Fronds at year 1 are 0.10-0.12 of a mature one, a knee-high rosette at 3, near-mature fronds on a ground rosette at 6, the shipped palm at 40 byte-identical. Seedling leaves: leaflet count from 1 (an entire strap) to 113, by fn-277's keyed draw; leaves at 1/2/3/4 years 10/20/45/151 (were 1,017/1,921/2,938/3,842); reach 0.79-1.21 blades with no jump from one strip to two; a leaflet with no spine within 0.75 widths of another segment; next to a basal spine and between spines up to 3.2 widths (1.39 on the mature palm), accepted. Final gate 1,407 passed, 0 failed.

## Open

- A drawn rachis, continuous with the retained leaf base, and closing the spine gaps -> fn-283.
- The stem's radius against FAO's circumference -> species audit.

## Friction

- No source gave a young palm's leaf length at a stated age; four PMC papers scraped by hand (about 25 minutes, 60k tokens) -> source-screening tool returning candidate sentences.
- `telperion-heavy` admission queues of 20-70 minutes (gate, focused runs, single-test probes) -> queue that reports expected wait; smaller admission class for incremental single-crate runs; measured gate peak.
- Generation-limit inventory guard failed only at the end of a 30-minute gate -> add `cargo test -p telperion-core --test generation_limit_guard` to the focused-run recipe for `src/pipeline` changes.
- The R2c brief assumed a rachis primitive that did not exist; two escalations -> briefs name the device stage order (`resident/build.rs`: hang, folds, sweep, stations).

## Evidence

- `.flow/evidence/fn-275-a-young-palms-fronds-grow-to-their-size/SOURCES.md` (R1 sources, R2a measurements and host rulings)
- `.flow/evidence/fn-275-a-young-palms-fronds-grow-to-their-size/SEEDLING.md` (R2b and R2c: design, measurements, host rulings)
