---
satisfies: [R2, R3]
---
# fn-291-the-cpu-entry-builds-every-shipped.4 R2c and R3: all 24 cases under 3.5 GiB, stills of the entry's mesh, the probe in test:dist; gate and npm test

## Description
R2c and R3: the proof, and the check that keeps it. Run all 24 PROBE.md cases through the rebuilt `dist/`, each at or under 3.5 GiB of Wasm memory. Look at stills of the CPU entry's own mesh for Telperion and Laurelin against the pre-change mesh. Check every other preset's surface is byte-identical or within task 3's bounds. Then `test:dist` carries the probe, CI's package job runs it, and the gate and `npm test` pass.

**Size:** M
**Files:** `scripts/test-dist.mjs`, `.github/workflows/tests.yml` (only for a timeout), `docs/principles.md`, `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`, a still tool under the evidence folder
**Touches:** [scripts/test-dist.mjs, .github/workflows/tests.yml, docs/principles.md, .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/**]

### Approach
- Stills: the headless example draws from the curve (`docs/pipeline.md:84,115`), so render the CPU entry's mesh itself: positions, decoded normals, indices, one fixed camera, flat shading. Do this for Telperion and Laurelin at seed 1, before (base dist) and after; view them (at most four images). Seeds 7 and 42 built after the change are compared against the native full-precision mesh.
- Byte check: hash each other preset's surface from the base dist and the new one, or compare decoded values within task 3's bounds where the encoding changed.
- `test:dist`: one child process per case (memory only grows), each asserting `memoryBytes` ≤ 3,758,096,384 bytes (3.5 GiB). A failure names the entry, preset and seed. Keep the field-entry WebGPU refusal check.
- Cost: cite the probe's timings (about 4 minutes for the 24 cases). Set the package job's timeout if needed.
- `docs/principles.md:19`: name the dist check beside the entry tests.
- Gate once at the end: `CARGO_TARGET_DIR=<worktree>/target telperion-heavy 12 --threads 6 -- cargo test --profile ci --workspace --no-fail-fast`, then `npm test`; full output to `raw/`.

### Investigation targets
**Required**:
- `scripts/test-dist.mjs`, `.github/workflows/tests.yml:270-330`
- `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/probe-dist.mjs`

## Acceptance
- [ ] All 24 cases build with Wasm memory at or under 3.5 GiB, tabled in PROBE.md
- [ ] Telperion and Laurelin stills of the CPU entry's mesh, before and after, viewed and recorded as unchanged to the eye
- [ ] Every other preset's surface byte-identical, or decoded within task 3's bounds
- [ ] `npm run test:dist` runs every case in its own process with the 3.5 GiB assertion, passes locally, and runs in CI's package job
- [ ] Gate and `npm test` green; outputs under raw/


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
