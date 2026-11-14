# The silver birch as a real species

## Goal

Onboard the silver birch (Betula pendula) as a catalogue species: frozen botanical profile with cited ranges, catalogued references, a value-table preset, the fixed and fresh seed protocol, numeric gates, and the owner's accepting verdict in the harness on the mature path. The owner chose the birch, with the beech and the ash, as natural bases of fn-10's legendary trees (the White Tree of Gondor grows from the birch). The spec began with all three species; the ash moved to fn-56 and the beech to fn-62, and this spec shipped the birch alone.

## Decisions

- > user (2026-09-14, on the natural base of each legendary tree): "Onboard the base species"
- > user (2026-09-14, on where that work lives): "Separate species spec"
- > user (2026-09-15): "yes move ash to its own spec"
- > user (2026-09-16, on moving the beech to its own spec): "ok let's do that."
- One species per spec: three species grew fifteen capability dependencies (fn-36 to fn-40, fn-44 to fn-52, fn-54) and twenty-six rounds; the lesson is the one-species-per-spec rule in AGENTS.md.
- Judged where it is seen: the round-25 acceptance on the judging page was withdrawn on 2026-09-17 when the harness showed a different tree (growth path, 590,410 nodes at seed 1, versus the direct build, 73,337). fn-65 made the direct build the product; the round-26 verdict was retaken in the harness with base and in-crown views.
- Measure before render: round 26 measured nine candidates and rendered four.
- Growth traits copied from the oak with a height-by-age entry; calibration is fn-30's and growth is hidden.

## Acceptance Criteria

- **R1:** Frozen profile, catalogued references and species record in the fn-19 schema, ranges cited, estimates labelled — met.
- **R2:** One value-table preset `silver_birch`, selectable natively, via wasm and in the browser, every identity site updated, no generator or renderer change — met.
- **R3:** Fixed and fresh seed protocol 48 of 48, none capped; owner accepts in the harness on the mature path — met.
- **R4:** Moved: the ash to fn-56, the beech to fn-62, with their packets — moved.
- **R5:** Identity pins, sweep leaf-count bands, growth-reference entry; byte-identical by seed — met.

## Result

- The rows that make it a birch: two stems leaning 28 degrees from a fork half a metre up; two limbs a station at 45 degrees, stations 1.2 m apart, drooping shoots (`rise_secondary` -0.85); seven twig laterals at 0.55; a curtain of 3 m strands with a ragged hem clearing the stems by 1.8 m; 18 m envelope, 0.36 spread; small rhombic serrated leaf; white bark row from fn-40.
- Round 26 (two limbs at 45 instead of four at 62, seven laterals at 0.55 instead of eight at 0.6): lower crown 27 to 13 primaries, 54,512 to 33,526 branches at seed 1, whole-tree centre on the photograph's 83.
- Owner verdict, round 26 (2026-09-18), the silver birch in the harness on the mature path, seed 1: Accepting. "alright i will accept this. It looks great now".

## Open

Three defects the owner named stay outside: the ring at the fork (fn-66), the flat-ended limbs (fn-4), the veins' tone and relief (fn-60).

## Evidence

- `.flow/evidence/fn34/REPORT.md` (index), `rounds.tsv` (every round's numbers), `round26-trials.tsv`, `numeric-summary.json`, `seeds.json`, `profiles.json`, `stills/` and `stills.json` (protocol stills, referenced by the catalogue), `GAPS.md` (the round-3 gap analysis that spawned the capability specs)
