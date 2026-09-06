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
FN-9.9 remains blocked after pass-2 architecture and inspected QA at production 3f3533c. All 48 retained numeric cases pass; spruce 4250668600 is 8.621604388 m <= unchanged 9.144 m. All 60 final image hashes are personally inspected. Oak upper bare sprays and every spruce crown/curtain mass still fail. Uncropped spruce seed-1 exterior detail now proves blunt terminal-cap taper failure; peg contact and complete socket fidelity remain unresolved. Upper-side forward bias passes only the seed-1 supplemental view. All nine shared-template geometry/PNG hashes match pass 1; software gates pass with isolated browser exit 0. No PR, merge, completion or owner approval.

See .flow/evidence/fn9/REPORT.md and pass2-* evidence. Next work: trace upper oak local handoffs/radius/shedding; reshape spruce pendant local systems; correct actual terminal caps and resolve peg/socket contact. Keep every seed and frozen range; do not substitute a global foliage count/shell increase.

## Evidence

- Commits: `3f3533c` architectural placement and regression; `54702f2` / `0d608f6` actual exterior twig/socket cameras and surface-radius needle selection; `972b3bd` consistent complete small browser fixtures. Numeric, cost, gate and final visual evidence commits remain in branch history.
- Tests: all 48 retained numeric cases, native workspace, 106 harness tests, fmt, Clippy `-D warnings`, typecheck, build, isolated Wasm/browser integration exit 0, final diff and Flow validation. Receipt replay exits expected 1 because automated visual approval is never granted.
- Visuals: `.flow/evidence/fn9/REPORT.md`, `pass2-visual.json`, `pass2-captures.json`, `pass2-shared.json`, and `qa-preview/pass2-*.png`. Bulk captures `/tmp/fn99-fix2-qa`; raw measurements `/tmp/fn99-fix2-replay`. Prior report preserved as `REPORT-pass1.md`.
- Next: trace upper oak handoffs/radius/shedding, reshape spruce local pendant systems into substantial curtains, correct actual terminal caps and resolve peg/socket contact. Preserve every seed and frozen target; no global foliage-count substitute.
- PRs: none; Flow remains blocked and ineligible for make-pr.
