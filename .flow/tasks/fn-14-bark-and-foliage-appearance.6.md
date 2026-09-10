---
satisfies: [R1, R2, R3, R11, R12]
---
# fn-14-bark-and-foliage-appearance.6 The evidence: references fetched, lit hero stills, the clock, the report, docs, and the owner's verdicts

## Description
Produce the evidence the spec closes on: fn-9's reference photographs re-fetched into the ignored references directory, the lit oak and spruce hero stills at seed 7, the oak's timing and browser orbit beside fn-24's, the clay view still for R3, the report, the docs that went stale, and the two owner verdict slots (R11, R12, the evidence halves of R1 to R3). Ends NEEDS_HUMAN by design until the owner writes the verdicts.

**Size:** M
**Files:** `.refs/fn9/` (ignored), `.flow/evidence/fn14/` (stills, timing JSON, REPORT.md), `.flow/specs/fn-14-bark-and-foliage-appearance.md` (owner verdict section), `README.md`, `STRATEGY.md` (one wording line), module headers in `crates/telperion-render/src/{lib,scene,wood,foliage,timing}.rs` and the three shader headers, `docs/species-onboarding.md`
**Touches:** [.refs/**, .flow/evidence/fn14/**, .flow/specs/fn-14-bark-and-foliage-appearance.md, README.md, STRATEGY.md, docs/species-onboarding.md, crates/telperion-render/src/lib.rs, crates/telperion-render/src/scene.rs, crates/telperion-render/src/wood.rs, crates/telperion-render/src/foliage.rs, crates/telperion-render/src/timing.rs, crates/telperion-render/src/shaders/**]

### Approach
- References: run fn-9's retrieval command from `.flow/evidence/fn9/REFERENCES.md` for O-WHOLE and S-WHOLE into `.refs/fn9/`; record source URL and SHA-256 in the report; never copy a pixel into the repo. Not a GPU capture.
- Stills: oak and spruce at seed 7, hero pose, 1600 by 1000, the default scene row, into `.flow/evidence/fn14/`; one clay-view oak still beside them for R3. View at most four images.
- Clock: the oak native timing and orbit, the browser orbit through `npm run test:render`, all with the shadow pass and 4x; the numbers beside fn-24's with the verdict vocabulary; R12's bound is 3.8 ms native p50 and the orbit's 16.7 and 33 ms.
- Report in fn-24's shape, plus a materials section: the shadow and resolve numbers, the sample count, and the memory the new buffers and the shadow map take.
- Docs: README lines that describe the clay room, the viewer, the headless flags and the timing record; the module and shader headers that say clay; the strategy's owner's-eye metric line drops "in clay"; species-onboarding gains one line on the appearance ranges the reference supplies.
- Verdicts: an `## Owner verdict` section in fn-23's shape with two slots, oak and spruce, each with the photograph's source and checksum beside it; leave the quoted lines empty for the conductor.

### Investigation targets
**Required** (read before coding):
- `.flow/evidence/fn9/REFERENCES.md` — the manifest and retrieval command
- `.flow/evidence/fn24/REPORT.md` — the report shape and verdict vocabulary
- `README.md:55-150` — the stale prose

**Optional** (reference as needed):
- `.flow/specs/fn-23-fast-hero-the-oak-inside-the-frame.md` — the owner verdict section shape

### Key context
- Budget rules bind: one timing re-run per target, no forest captures, four images viewed at most, never open receipts.
- If R12's number fails, write it into the report and return; no tuning.

## Acceptance
- [ ] O-WHOLE and S-WHOLE fetched under `.refs/fn9/` with source and checksum in the report; nothing from them committed
- [ ] Lit oak and spruce hero stills and one clay oak still under `.flow/evidence/fn14/`
- [ ] Oak native timing and browser orbit recorded beside fn-24's with verdicts; native p50 at or under 3.8 ms and the orbit inside its bounds, or the spec stops with the number
- [ ] `REPORT.md` in fn-24's shape with the materials section
- [ ] README, strategy wording, module and shader headers, and species-onboarding no longer describe the clay room as the only light
- [ ] Owner verdict slots for oak and spruce written into the spec, ready for the owner's words

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
