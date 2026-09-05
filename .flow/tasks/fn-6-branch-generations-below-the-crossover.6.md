---
satisfies: [R3, R7]
---
# fn-6-branch-generations-below-the-crossover.6 The harness and the two trees: the dial retired, the read-out, no depth stated

## Description
The panel stops offering a depth and starts reporting one: the orders slider goes, a read-out shows the derived generation range and how many handoffs hit the cap, the ceiling notice stops telling the owner to lower the orders, and both presets are stated without a depth so the owner can judge R7 in clay. Frontend task: no DESIGN.md exists, follow the panel's own conventions.

**Size:** M
**Files:** `harness/params.ts`, `harness/skeleton-view.ts`, `harness/skeleton-view.test.ts`, `harness/GrowerDev.tsx`, `harness/params.test.ts`, `src/presets/two-trees.ts` (comments and any rest values the clay pass settles)
**Touches:** [.flow/specs/fn-6-branch-generations-below-the-crossover.md, harness/params.ts, harness/skeleton-view.ts, harness/skeleton-view.test.ts, harness/GrowerDev.tsx, harness/params.test.ts, src/presets/two-trees.ts]

### Approach
- Remove the `twigLevels` slider (`harness/params.ts:271`) and its block comment (`:258-270`); add dials for the new terms with rails and measured-comment justification in the same style (`lengthRatio`, `ratioPower`, `internodes`, `laterals`, `limbRadius`; the twig anatomy is a preset term with no dial unless the clay pass wants one).
- `TreeStats` (`harness/skeleton-view.ts:196`) gains generations min/median/max, handoffs, level-capped handoffs, twig count; `GrowerDev.tsx` renders them beside nodes/triangles; the capped notice at `GrowerDev.tsx:472` is reworded to the dials that now exist (step, `limbRadius`).
- Round-trip tests (`harness/skeleton-view.test.ts:549-568`) follow the new param set term for term; slider-range test follows.
- Presets: comments on the twigs blocks rewritten to the new model (no R8 depth judgement; the read-out is the number); the R7 clay pass with the owner happens on this task's build, after task 5 has put the foliage on the twig, and any rest value the owner moves is recorded in the preset comment with the reason and re-measured against task 5's R4 thresholds before it ships.
- Build-time honesty: keep the settle-after-last-notch mechanism; the build timer already in `TreeStats.buildMs` is shown.

### Investigation targets
**Required:**
- `harness/params.ts:100-140, 223-301, 360-372` — field docs, sliders, defaults
- `harness/skeleton-view.ts:66-130, 190-215, 273-318, 377-470` — mappings, stats, build
- `harness/GrowerDev.tsx:440-480` — stats and notices rendering

**Optional:**
- `harness/skeleton-view.test.ts:540-600, 705-730` — the tests that build both presets in full (60 s timeouts)

### Key context
- Memory `a-second-subject-on-stage-makes-every`: every "what is on screen" query must handle the comparison view with two presets.
- R7 is a manual gate: this task ends with the owner's verdict recorded in the spec, not with a test.
## Acceptance
- [ ] No orders slider; a read-out shows generation range, handoffs and level-capped handoffs; ceiling notice names existing dials
- [ ] New dials with rails and measured comments; round-trip and slider-range tests green
- [ ] Both presets state the twig and the ratios, no depth; comments rewritten
- [ ] Any preset value moved by the verdict still holds task 5's R4 thresholds
- [ ] Owner's R7 verdict recorded in the spec (fills the volume; reads as its size; Telperion and Laurelin read as themselves)
- [ ] `npx vitest run harness src/presets` green; `npx tsc --noEmit` green; `npm run dev` builds both presets
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
