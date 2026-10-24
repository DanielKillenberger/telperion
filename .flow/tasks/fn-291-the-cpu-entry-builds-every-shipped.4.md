---
satisfies: [R2, R3]
---
# fn-291-the-cpu-entry-builds-every-shipped.4 R2c and R3: all 24 cases under 3.5 GiB, stills of the entry's mesh, the probe in test:dist; gate and npm test

## Description
R2c and R3: the proof, and the check that keeps it. All 24 PROBE.md cases build through the rebuilt `dist/`, each at or under 3.5 GiB of Wasm memory. Telperion and Laurelin look unchanged in stills of the CPU entry's own mesh. Every other preset's surface decodes equal, or within task 6's bounds. `test:dist` carries the probe, and CI's package job runs it. The gate and `npm test` pass.

**Size:** M
**Files:** `scripts/test-dist.mjs`, `.github/workflows/tests.yml` (only for a timeout), `docs/principles.md`, `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`, a still tool under the evidence folder
**Touches:** [scripts/test-dist.mjs, .github/workflows/tests.yml, docs/principles.md, .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/**]

### Approach
- Stills. The headless example draws from the curve (`docs/pipeline.md:84,115`), so render the CPU entry's mesh itself from its decoded chunks: one fixed camera, flat shading.
  - Telperion and Laurelin at seed 1: the base dist (flat) against the new one (chunked).
  - Seeds 7 and 42: the new dist against the base native flat mesh.
  - View every image the verdict rests on.
- Byte check: decode each other preset's surface from the new dist and compare it with the base dist's flat surface, exact or within task 6's bounds.
- `test:dist`:
  - One child process per case, because memory only grows. Each asserts `memoryBytes` ≤ 3,758,096,384 (3.5 GiB).
  - A failure names the entry, preset and seed.
  - Keep the field entry's WebGPU refusal check.
  - Cite the probe's timings (about 4 minutes) and set the package job's timeout if needed.
- `docs/principles.md:19` names the dist check.
- Run the gate once at the end through the declared invocation `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` (AGENTS.md), with `CARGO_TARGET_DIR` set to the worktree's target. Then run `npm test`. Save full output to `raw/`.

### Investigation targets
**Required**:
- `scripts/test-dist.mjs`, `.github/workflows/tests.yml:270-330`
- `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/probe-dist.mjs`
## Acceptance
- [ ] All 24 cases build with Wasm memory at or under 3.5 GiB, tabled in PROBE.md
- [ ] Telperion and Laurelin stills of the CPU entry's mesh, before and after, viewed and recorded as unchanged to the eye
- [ ] Every other preset's surface byte-identical, or decoded within task 6's bounds
- [ ] `npm run test:dist` runs every case in its own process with the 3.5 GiB assertion, passes locally, and runs in CI's package job
- [ ] Gate and `npm test` green; outputs under raw/
## Done summary
`npm run test:dist` now builds every shipped preset at seeds 1, 7 and 42 through `dist/telperion.js`, one child process per case. Each case checks its wood chunks and fails if `memoryBytes` passes 3.5 GiB, naming the entry, preset and seed. The field entry's WebGPU refusal check stays. `docs/principles.md` names the check. CI's package job already runs it, and no job sets a timeout, so `tests.yml` is unchanged.

Measured (PROBE.md "Task 4"):
- All 24 cases build under the bar, in 4 min 3 s. The highest is laurelin seed 7 at 2,789 MiB; telperion at seeds 7 and 42 is 2,777 MiB.
- Surface check against the pre-chunk base dist covers the 6 other presets at 3 seeds plus telperion and laurelin at seed 1. Positions and absolute indices are byte-identical in all 20 cases. The worst decoded normal is 6.47e-5 rad, inside the 1e-4 rad bound.
- Stills of Telperion and Laurelin at seeds 1, 7 and 42 were drawn from the entry's own mesh by a software rasterizer (`still.mjs`). At seed 1 they are compared against the base dist; at seeds 7 and 42, against the base native flat mesh, whose positions and indices hash equal. No pixel differs by more than 1 grey level. All six sheets were viewed: unchanged to the eye.
- Gate: 1,412 passed, 0 failed, 103 ignored, after a 55-minute admission wait. `npm test`: 170 passed.
- Not run: `npm run rust:test:wasm`, the bindings suite (excluded by the host).

Friction: two admission waits and a refused pipe (FRICTION.md).

Tier: (conductor dispatch)

stage: impl-review - ran [codex, one reviewer (correctness): SHIP, no findings]

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 50ea727f27024d5754dae21decc4c93c66c2ca19, 51700113c26cc84046dc54b992ebab2c2478d34a
- Tests: telperion-heavy 6 --threads 2 -- npm run test:dist, node .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/surface-check.mjs <base-dist> dist <scratch> <ids> 1,7,42, .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/stills.sh, telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: