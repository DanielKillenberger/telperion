---
satisfies: [R3, R4, R6]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.6 The evidence: stills, the transition, the oak's frame, and the owner's verdicts

## Description
Produce the evidence the spec closes on: one still per preset at the hero pose, the oak-to-spruce transition, fn-23's oak timing and orbit re-run, a report, and the owner's recorded verdicts on the oak still, the spruce still and the video (R3, R4, R6). Fold the doc updates the enums leave stale into the same task.

**Size:** M
**Files:** `.flow/evidence/fn24/` (stills, timing JSON, transition record, `REPORT.md`), `.flow/specs/fn-24-continuous-tree-space-habit-and-leaf-as.md` (owner verdict section), `docs/species-onboarding.md`, `tests/migration/README.md`
**Touches:** [.flow/evidence/fn24/**, .flow/specs/fn-24-continuous-tree-space-habit-and-leaf-as.md, docs/species-onboarding.md, tests/migration/README.md]

### Approach
- Stills: `npm run species:qa` through the headless target for all five presets; oak and spruce at seed 7, hero pose, 1600 by 1000, the same framing as `.flow/evidence/fn23/oak-hero.png` and `.flow/evidence/fn22/spruce-hero.png`. Ordinary, Telperion and Laurelin are recorded, not gated.
- Transition: one run of the headless command from oak to spruce at seed 7, 240 frames, into `.flow/evidence/fn24/transition/`; the record and the notice go in the report. Do not open the frames or the video.
- Timing: re-run fn-23's oak native timing and browser orbit exactly as fn-23's quick commands do, into `.flow/evidence/fn24/`; the report puts the numbers beside fn-23's with the same verdict vocabulary. An unavailable, disjoint or contended session is recorded and does not count.
- Report: follow `.flow/evidence/fn23/REPORT.md` in structure; protocol first, then per-preset results, then the transition record.
- Verdicts: add an `## Owner verdict` section to the spec in fn-23's shape with three slots, oak still, spruce still, transition video, each answered in the owner's words; the task ends NEEDS_HUMAN until the owner writes them, and a rejecting verdict stops the spec with a one-paragraph blocker.
- Docs: reword the capability column at `docs/species-onboarding.md:13` away from enum-gated anatomy, and re-check the walkthrough at `tests/migration/README.md:91` against the renamed tests.

### Investigation targets
**Required** (read before coding):
- `.flow/evidence/fn23/REPORT.md` — the report shape and verdict vocabulary
- `.flow/specs/fn-23-fast-hero-the-oak-inside-the-frame.md` — the Quick commands and the Owner verdict section shape
- `tests/species.mjs` — the QA still protocol

**Optional** (reference as needed):
- `.flow/evidence/fn23/oak-native-timing.json` — the numbers to sit beside

### Key context
- Budget: at most four images viewed per capture; one transition render; one timing re-run. No forest captures.

## Acceptance
- [ ] Five stills at the hero pose under `.flow/evidence/fn24/`; oak and spruce match fn-23's and fn-22's framing
- [ ] Transition frame sequence and record under `.flow/evidence/fn24/transition/`, video present or a recorded notice
- [ ] Oak native timing and browser orbit recorded beside fn-23's numbers with verdicts; oak p50 at or under 2 ms native and the orbit inside fn-23's R2 bounds, or the spec stops with the number
- [ ] `REPORT.md` in fn-23's shape
- [ ] Owner verdicts on the oak still, the spruce still and the transition video recorded in the spec in the owner's words; the spec closes only on three accepting verdicts
- [ ] Species-onboarding and migration docs no longer describe enum-gated anatomy or stale test names

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
