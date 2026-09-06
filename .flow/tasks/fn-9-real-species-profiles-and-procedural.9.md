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
Blocked:
FN-9.9 QA rejects completion. All 48 recorded seeds measured and all 50 mature PNGs captured/inspected, but fresh spruce 4250668600 exceeds crown width (9.225869594 m > 9.144 m). Both species fail healthy leaf-on crown mass; oak lobes and spruce needle taper fail reference shape. Terminal/socket fidelity remains unassessed; contextual spruce DBH remains ambiguous. See .flow/evidence/fn9/REPORT.md and retained per-case evidence. Resume corrective generation/anatomy work using the same seeds and references, then rerun QA. No owner approval; no PR or merge until acceptance passes.
## Evidence
- Commits: `fe1eefa` (fresh-seed manifest before generation); final QA/evidence commit in branch history.
- Tests: full native workspace, Wasm/browser integration, 106 harness tests, typecheck/build; Rust 1.98 Clippy findings corrected and focused 19-test species/foliage/measurement suite passed. Detailed gates in `.flow/evidence/fn9/REPORT.md`.
- PRs: none; task blocked, not eligible for make-pr.
