---
satisfies: [R3]
---
# fn-291-the-cpu-entry-builds-every-shipped.3 test:dist holds the package entry to the R1 matrix in CI; gate and npm test green

## Description
Extend `scripts/test-dist.mjs` so the package job checks every shipped preset through the dist CPU entry: each builds, or is refused by the name R2 settled, and any other outcome fails naming entry, preset and seed. Keep CI's cost bounded (runner memory, job time): the full three-seed matrix only if task 1's timings fit the job, otherwise one seed per preset in CI and the full matrix as the probe. Then run the gate once and `npm test`.

**Size:** S
**Files:** `scripts/test-dist.mjs`, `.github/workflows/tests.yml` (only if a timeout or step change is needed), `docs/principles.md`
**Touches:** [scripts/test-dist.mjs, .github/workflows/tests.yml, docs/principles.md]

### Approach
- One fresh `TreeEngine` per case (memory only grows).
- The package job's step at `.github/workflows/tests.yml:311-322` already runs `npm run test:dist` on the unpacked tarball; keep it there.
- `docs/principles.md:19` names the entry tests; add the dist check beside them.
- Gate: `CARGO_TARGET_DIR=<worktree>/target telperion-heavy 12 --threads 6 -- cargo test --profile ci --workspace --no-fail-fast`, once, at the end; full output to `raw/`.

### Investigation targets
**Required**:
- `scripts/test-dist.mjs` — the script to extend
- `.github/workflows/tests.yml:270-330` — the package job

## Acceptance
- [ ] `npm run test:dist` covers every shipped preset and passes locally on the built dist, naming entry, preset and seed on failure
- [ ] The package job runs it within the job's time and memory (task 1 timings cited)
- [ ] Gate and `npm test` green, outputs saved under raw/

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
