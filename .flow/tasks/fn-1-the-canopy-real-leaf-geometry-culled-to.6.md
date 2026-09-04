# fn-1-the-canopy-real-leaf-geometry-culled-to.6 Public surface and the docs that still say there is no foliage

## Description
Exports the canopy stage and retires every claim in the repo that the library stops at branches. Folded into one task because these are all the same edit landing in five places, and this repo keeps two synchronized descriptions of the pipeline that go stale together.

**Size:** S
**Files:** `src/index.ts`, `README.md`, `package.json`, `harness/GrowerDev.tsx`
**Touches:** [src/index.ts, README.md, package.json, harness/GrowerDev.tsx]

### Approach

- Add the canopy export block to the barrel at `src/index.ts`, after the surface exports and before presets, with the one-line comment banner each existing group carries (see the banner above the space-colonization group).
- `src/index.ts:1-21` header states the pipeline is four stages and that the library emits geometry and attachment frames. Both claims become wrong.
- `README.md:32-41` is the pipeline table and needs a canopy row; the "Four stages" lead-in at `README.md:34` needs its count corrected.
- `README.md:51-55` says the library emits geometry and attachment frames, full stop. `README.md:63-65` is the Status paragraph saying foliage is not built yet. Both go stale on landing.
- `harness/GrowerDev.tsx:300` carries the same claim in the harness caption.
- `package.json` description enumerates the four stages' signature techniques and the keywords omit foliage.

### Investigation targets

**Required** (read before coding):
- `src/index.ts:1-88` - the header block and the whole barrel with its group banners
- `README.md:30-70` - pipeline table, the light section, and Status
- `harness/GrowerDev.tsx:295-305` - the caption claiming no foliage

**Optional** (reference as needed):
- `package.json` - description and keywords

### Key context

- This repo has no CHANGELOG and no changelog convention. Release notes live in commit bodies: a Conventional Commits subject, then a prose paragraph explaining the reasoning, then a `Task:` trailer. The landing commit body carries the weight a changelog entry would, so write it accordingly. There is no separate changelog file to touch.
- README and `src/index.ts` deliberately hold the same pipeline description in two formats. Update both in the same commit or they drift.
- No linter or formatter is configured. Match the surrounding file style by eye.

### Acceptance
## Acceptance
- [ ] Canopy exports present in the barrel with the group banner-comment convention
- [ ] `src/index.ts` header names five stages and no longer claims the library stops at attachment frames
- [ ] README pipeline table carries a canopy row and the stage count is corrected
- [ ] README light section and Status paragraph no longer state that foliage is unbuilt
- [ ] The harness caption no longer says there is no foliage
- [ ] `package.json` description and keywords name foliage
- [ ] `npx vitest run` and `npx tsc --noEmit` green
## Done summary
Exports the canopy stage from the barrel with the group banner-comment
convention - `buildElement`/`buildCanopy`/`cullCanopy`/`shoots` with their
parameter types and defaults - and retires the claim that this library grows
branches and stops, in all five places it was still made: the `src/index.ts`
header (four stages, "emits geometry and attachment frames"), the README
pipeline table and its "Four stages" lead-in, the README light section and
Status paragraph, `package.json`'s description and keywords, and the harness
caption.

**The silhouette judgement .4 left open: `src/canopy/silhouette.ts` is not
exported, and `src/index.ts` says why in three lines.** It is pure library code
with no test-only dependency, so it could ship - but it is a screen-space
instrument for holding the culler to account, not a way to grow a tree.
Exporting it puts eleven names into the public surface for something that is
not a stage, and semver-pins a raster resolution and six fixed view directions
whose only job is to make one test's tolerance honest. A consumer that wants an
outline has a camera and wants its own. The comment exists so the omission does
not read as an oversight to whoever reads the barrel next.

**The README quickstart was already broken before this task.**
`solveRadii(skeleton, envelope, params)` and `buildSurface(skeleton, field,
envelope, params)` both take the envelope; the documented snippet passed
neither, so the only executable thing in the README had never compiled. Fixed
here because the block was being rewritten anyway, and the replacement was
compiled against the real barrel as a standalone file before being written into
the README - not read for plausibility.

**Two stale strings outside this task's Touches, left for the conductor:**
`src/presets/index.ts:8-9` ("hands its three members") and
`src/presets/preset.ts:11-12` ("the three argument objects") are both wrong
since `TreePreset` gained a required `canopy` in .3 - four now. Same staleness
class this task existed to clear, in files it was forbidden to touch.

The harness caption is written about the library rather than about the frame,
so it is true whether or not .5's instanced draw lands in the same integration:
"foliage grows on the young wood at the end of every shoot and is culled to a
shell, so what you are looking at is the outside of the canopy and not its
filling."

Acceptance, all met: canopy exports present with the banner convention; the
header names five stages and no longer stops at attachment frames; README table
carries a canopy row and the count is corrected; README light section and
Status no longer say foliage is unbuilt; the harness caption no longer says it;
`package.json` description and keywords name foliage; `npx vitest run` (18
files / 251 tests) and `npx tsc --noEmit` green.

baseline: green via handoff (18 files / 251 tests at a69a0cb, verified by the
conductor); no linter or formatter is configured in this repo.

Sibling note at
`.git/flow-notes/fn-1-the-canopy-real-leaf-geometry-culled-to-20260904T174358Z-527372/t6-public-surface.md`.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after it integrates)
## Evidence
- Commits: 6eab840, HEAD
- Tests: npx vitest run (18 files / 251 tests passed on the integrated target), npx tsc --noEmit (clean on the integrated target), README quickstart extracted and typechecked standalone against the real barrel: clean
- PRs: