---
satisfies: [R8, R9]
---
# fn-190-one-growth-law-species-are-points-in-a.18 Pin today's baseline for every preset

## Description
Captures today's trees once, at a pinned revision, before anything replaces them: the comparison every later stage and R8/R9 use. Runs in wave 1, in parallel with the model specifications.

**Size:** S
**Files:** `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/BASELINE.md`, `raw/baseline/**`
**Touches:** [.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/BASELINE.md, .flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/baseline/**]

### Approach
- Revision: origin/master at `8e0141dc` (the probe's `today` command builds that pipeline; record the exact sha built, the build profile and the machine).
- Every shipped and in-work preset (beech, oak, spruce, birch, ash, date palm, telperion, laurelin, ordinary; the plane if it has a preset), seeds 1 and 7: bare and whole stills, skeleton warm-median time, full-build warm-median time, node count, fine wood, leaf count, peak memory.
- Later stages read these files; nobody re-runs `today` after production changes.

### Investigation targets
**Required:**
- `crates/telperion-render/examples/growth_law/main.rs` (the `today` command)
- `crates/telperion-core/src/presets.rs`

## Acceptance
- [ ] BASELINE.md names the revision, profile and machine
- [ ] Stills and every listed measure exist for every preset at seeds 1 and 7
- [ ] The host has viewed the stills

## Done summary
BASELINE.md pins eight presets at 8e0141dc (release, 21-run skeleton medians, full build, leaf count, peak memory via a scratch measuring crate); ash and plane have no preset there. Host viewed all eight sheets; the palm whole stills crop the crown (fixed in .19). Friction moved to FRICTION.md; cost mode and 11-run medians added to .19.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 551391c78e1eed09da5e9084784b326977274abe
- Tests:
- PRs: