---
satisfies: [R4, R5]
---
# fn-23-fast-hero-the-oak-inside-the-frame.6 Every preset through one path, the stills, the owner's verdict, and the docs that say one draw

## Description
Close the spec: the preset and random-set sweep on the final path (R5), the species stills through the headless target, the oak still placed beside fn-22's for the owner's verdict (R4), the report completed, and the documentation that still describes one instanced draw brought to the truth.

**Size:** S
**Files:** `crates/telperion-render/tests/conformance.rs`, `tests/species.mjs`, `.flow/evidence/fn23/oak-hero.png`, `.flow/evidence/fn23/spruce-hero.png`, `.flow/evidence/fn23/REPORT.md`, `.flow/specs/fn-23-fast-hero-the-oak-inside-the-frame.md`, `README.md`, `package.json`, `crates/telperion-render/src/foliage.rs`
**Touches:** [crates/telperion-render/tests/conformance.rs, tests/species.mjs, .flow/evidence/fn23/**, .flow/specs/fn-23-fast-hero-the-oak-inside-the-frame.md, README.md, package.json, crates/telperion-render/src/foliage.rs]

### Approach
- Confirm the conformance sweep from task 3 covers every shipped preset and at least twenty seeded random parameter sets on the final path, with one exercised generator rejection surfacing the generator's message; add the seed list to the test if it is not already pinned.
- `npm run species:qa` renders every profile's still through the headless target; commit the oak and spruce hero stills at seed 7 to `.flow/evidence/fn23/`.
- Complete `REPORT.md`: the measured tables from tasks 4 and 5, the spruce numbers marked as recorded and not gated, and the owner verdict slot. The owner compares `oak-hero.png` with `.flow/evidence/fn22/oak-hero.png` and writes the verdict into the spec under a `## Owner verdict` heading; the task is done only on an accepting verdict.
- Documentation: `package.json` description and the module doc at `crates/telperion-render/src/foliage.rs:1` say one instanced draw; `README.md:58` describes the draw path, `:92-93` the headless example, `:100` the timing record. Update each in one sentence to the per-level indirect draws, the `--level` and `--orbit` flags, and the selection and orbit fields.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/tests/conformance.rs` — the sweep to confirm or extend
- `tests/species.mjs` — the stills rig
- `README.md:55-105` — the three statements to update

**Optional** (reference as needed):
- `.flow/evidence/fn22/REPORT.md` — the owner verdict section shape

### Key context
- The verdict is the owner's, recorded in the spec in their words; the task prepares the comparison and waits.
- Budget: two stills and the sweep; no further timing runs unless task 4 or 5 evidence was invalidated by a change here, in which case rerun that one session.

## Acceptance
- [ ] Every shipped preset and at least twenty pinned random parameter sets render a non-background still through the final path, and one generator rejection surfaces the generator's message, in `cargo test --release -p telperion-render`
- [ ] `npm run species:qa` passes and `.flow/evidence/fn23/oak-hero.png` and `spruce-hero.png` are committed at seed 7 from the headless target
- [ ] `.flow/evidence/fn23/REPORT.md` carries the native and browser tables, the orbit numbers, the spruce rows marked recorded and not gated, and the owner's verdict
- [ ] The spec carries the owner's verdict on the oak still in a `## Owner verdict` section; the task closes only on an accepting verdict
- [ ] `package.json`, `README.md` and the foliage module doc no longer say one instanced draw; the README names `--level` and `--orbit` and the selection and orbit record fields
- [ ] `npm run typecheck` and `cargo test --release --workspace` pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
