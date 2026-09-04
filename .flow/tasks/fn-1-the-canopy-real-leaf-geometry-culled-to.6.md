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

- [ ] Canopy exports present in the barrel with the group banner-comment convention
- [ ] `src/index.ts` header names five stages and no longer claims the library stops at attachment frames
- [ ] README pipeline table carries a canopy row and the stage count is corrected
- [ ] README light section and Status paragraph no longer state that foliage is unbuilt
- [ ] The harness caption no longer says there is no foliage
- [ ] `package.json` description and keywords name foliage
- [ ] `npx vitest run` and `npx tsc --noEmit` green

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
