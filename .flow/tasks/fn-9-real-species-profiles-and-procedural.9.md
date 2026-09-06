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
- [x] Native workspace tests, Wasm checks, harness tests, formatting/clippy, typecheck, build and relevant headless integration all pass; evidence remains compact.

## Done summary
Blocked after corrective QA:
All 48 retained numeric cases pass at generation `1f2ebf4`; spruce `4250668600` now measures 8.813982965 m against the unchanged 9.144 m maximum. All 52 final required/supplemental/shared-template PNGs were captured and personally inspected. Oak lobes, spruce needle shaft and foliage-bearing distribution improved, but every inspected species crown still fails healthy mass/gaps; spruce curtains, peg/orientation and complete terminal/socket fidelity remain unmet or unassessed. Ordinary, Telperion and Laurelin geometry and PNG hashes exactly match previous QA. No owner approval, PR, merge or completion. See the corrective REPORT and retained before-fix evidence.

## Evidence
- Commits: `fe1eefa` freezes seeds; `3a50326` and `1f2ebf4` implement corrective generation and regressions; final browser/evidence receipt in branch history.
- Tests: 48/48 numeric replay; full native workspace, isolated Wasm/browser integration (exit 0), 106 harness tests, fmt, Clippy with `-D warnings`, typecheck and build. Browser process result is recorded separately in `corrective-gates.json`; no image-existence fidelity pass.
- Visuals: 52 inspected PNGs; unchanged nine shared-template geometry/PNG hashes; same-host native counts/costs. `.flow/evidence/fn9/REPORT.md`, `corrective-*.json` and `qa-preview/fix-*.png`.
- PRs: none; Flow remains blocked and not eligible for make-pr.
