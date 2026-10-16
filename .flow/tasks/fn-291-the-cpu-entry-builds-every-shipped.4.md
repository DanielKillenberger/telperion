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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
