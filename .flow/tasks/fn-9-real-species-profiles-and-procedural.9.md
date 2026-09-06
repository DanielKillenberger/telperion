---
satisfies: [R2, R3, R4, R5, R6]
---
# fn-9-real-species-profiles-and-procedural.9 Run cross-seed visual QA and document the species workflow

## Description
Run cross-seed visual QA and document the species workflow. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `tests/browser/species.mjs`, `package.json`, `README.md`, `tests/migration/README.md`, `.flow/evidence/fn9/REPORT.md`
**Touches:** [tests/browser/species.mjs, package.json, README.md, tests/migration/README.md, .flow/evidence/fn9/REPORT.md]

### Approach
- Add a compact headless runner using the existing integration capture pattern. Render the parent protocol, record camera/browser/renderer metadata and output hashes, preserve per-case results on failure, and write bulk output outside the checkout by default.
- Draw and record the 12 fresh seeds per species after initial calibration; run fixed and fresh measurements, capture the required subsets plus all numerical failures, and personally inspect whole-tree/bare/foliage views against references.
- Report each trait and case, separating numeric pass, visual assessment, missing evidence and actual owner feedback. Fix observed mismatches and retain them as regressions; record unresolved failures rather than declaring fidelity complete.
- Recheck Ordinary, Telperion and Laurelin after shared-rule changes, with known historical defects distinguished from new regressions. Record native generation time/count/size costs on the same host; no software-renderer GPU speed claim.
- Document public species selection, seed semantics, profile provenance, CPU-only generation/capture prerequisites and replay commands. Keep source manifest and compact final report durable; wire relevant checks into package scripts and run final integrated gates once.

### Investigation targets
**Required:**
- `tests/browser/integration.mjs` — headless setup and image output
- `tests/browser/migration.mjs:32-110` — fixed capture environment
- `README.md:51-83` — public commands and cost claims
- `tests/migration/README.md` — evidence/replay conventions
- `.flow/evidence/fn8/REPORT.md` — compact evidence pattern
- `package.json` — check entrypoints

### Quick commands
```bash
npm run rust:test
npm run rust:test:wasm
npm test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm run typecheck
npm run build
node tests/browser/integration.mjs
node tests/browser/species.mjs --help
```

## Acceptance
- [x] All 24 recorded seeds per species have explicit numeric results; required fixed/fresh image sets are inspected against source references.
- [ ] Trait-level evidence supports R3–R6, or outstanding failures remain explicit and prevent task completion; no unearned owner approval is recorded.
- [x] Ordinary and both Two Trees are checked for shared-rule regressions, with same-host CPU/count/size results and renderer identity reported.
- [x] Documented CPU-only replay succeeds, or a concrete environment limitation remains unassessed; missing captures never silently pass.
- [x] Native workspace tests, Wasm checks, harness tests, formatting/clippy, typecheck, build and relevant headless integration all pass; evidence remains compact. Pass-5 clean isolated Wasm/browser and final mature recorder both have explicit exit 0; the failed reused-server attempt is retained separately.

## Done summary
Blocked after pass-5 local occupancy implementation and personal inspection. Final production `314efc4` passes all 48 retained numeric cases with explicit exit 0; spruce 4250668600 width is 8.688966688 m <= unchanged 9.144 m. Upper local descendants move in failed oak crowns, but 2, 3 and 2666899686 still have separated masses/windows. Protected oak 1/762807349/1444323199 retain exact pass-3 geometry and whole/bare/detail PNGs. Local-only transverse redistribution preserves every spruce structural fork and improves measured needle coverage, but all seven inspected spruce crowns still lack substantial S-BRANCH curtains. First-target peg-clear-front passes only for local projecting relief; full first-target peg/socket remains unresolved. All 84 final images are personally inspected. All nine shared geometry/PNG hashes exactly match pass 3. Final software gates pass, including a clean standard Wasm/browser run with explicit exit 0 after a separately retained reused-Vite failure. No PR, merge, completion or owner approval.

Blocked:
Pass 5 remains visually blocked after 84 personally inspected final images. Oak 2, 3 and 2666899686 retain separated upper masses/open windows; all seven required spruce crowns remain transparent and lack substantial S-BRANCH curtains. The first-target peg-clear-front has a narrow local projection pass, but other angles and the complete peg/socket remain unresolved. Numeric 48/48 pass with explicit exit 0 at production 314efc4, including spruce 4250668600 width 8.688966688 <= 9.144 m. Protected oak 1/762807349/1444323199 retain exact pass-3 geometry and whole/bare/detail PNGs; all nine shared views also match exactly. Final software gates, mature capture and isolated Wasm/browser exit 0. Evidence: .flow/evidence/fn9/REPORT.md and pass5-*. Next: measure feasible connected occupancy into actual oak windows; diagnose spruce longitudinal overlap and needle-versus-wood coverage while retaining every structural fork; obtain unobscured complete connected first-target peg/socket evidence. Preserve all seeds/ranges/count constraints. No PR, merge, completion or owner approval.

## Evidence

- Implementation: `314efc4`; pinned/batched capture runner `83f846c`; exact batch parity in `pass5-batch-parity.json`.
- Numeric: `.flow/evidence/fn9/pass5-numeric.json`; 48 retained cases pass with frozen seeds/ranges unchanged. Bulk `/tmp/fn99-pass5-verified-replay`.
- Gates: `.flow/evidence/fn9/pass5-gates.json`; native workspace, 106 harness tests, fmt, Clippy `-D warnings`, typecheck, build, clean Wasm/browser, diff and Flow validation. Failed pilot/boundary/fork/reused-server attempts remain explicit.
- Visuals: `.flow/evidence/fn9/REPORT.md`, `pass5-visual.json`, `pass5-captures.json`, `pass5-contacts.json`, `pass5-shared.json`, `pass5-protected-oaks.json`, `qa-preview/pass5-*.png`. Bulk `/tmp/fn99-pass5-qa`; previous authoritative report preserved as `REPORT-pass4.md`.
- Diagnosis: `pass5-oak-audit.json` traces retained upper descendants and occupied/vacated endpoint voxels. `pass5-curtain-audit.json` measures real triangle transverse coverage and longitudinal continuity on the same supporting shoots. Superseded structural compression stays separate in `pass5-*-pilot-audit.json`. `pass5-costs.json` records same-host native samples, sizes and peak RSS.
- Next: preserve accepted oak hashes and retained upper descendants; target actual empty inter-mass regions within feasible connected reach. Diagnose spruce longitudinal overlap and needle-versus-wood coverage without flattening structural forks or increasing global counts/lengths. Obtain complete unobscured first-target peg/socket evidence at the retained exact IDs. Preserve every seed and profile range.
- PRs: none; blocked and ineligible for make-pr.
