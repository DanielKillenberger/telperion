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
- [x] Native workspace tests, Wasm checks, harness tests, formatting/clippy, typecheck, build and relevant headless integration all pass; evidence remains compact. Pass-4 isolated Wasm/browser and final mature recorder both have explicit exit 0.

## Done summary
Blocked after pass-4 implementation and personal inspection. Production `99112c4` passes all 48 retained numeric cases; spruce 4250668600 width is 8.685199801 m <= unchanged 9.144 m. Targeted scaffold direction/spacing preserves exact output for accepted oak crowns 1, 762807349 and 1444323199, but crowns 2, 3 and 2666899686 still fail. Stable spruce pendant planes and polygonal surface attachment improve architecture/contact, but all seven inspected spruce crowns still lack substantial S-BRANCH curtains. One alternate peg angle passes locally; complete peg/socket continuity remains unassessed. All 75 final images are personally inspected. All nine shared geometry/PNG hashes exactly match pass 3. Software gates pass with explicit isolated browser exit 0. No PR, merge, completion or owner approval.

Blocked:
Pass 4 remains visually blocked after 75 personally inspected images: oak 2, 3 and 2666899686 retain separated masses/open crown windows; all seven required spruce crowns lack substantial S-BRANCH curtains; one alternate peg angle is a local pass, but complete peg/socket continuity remains unresolved. Numeric 48/48 pass, retained spruce 4250668600 width 8.685199801 <= 9.144 m. Accepted oak 1/762807349/1444323199 and all nine shared views retain exact geometry/PNG hashes. Software gates and isolated browser/final mature recorder exit 0. Evidence: .flow/evidence/fn9/REPORT.md and pass4-*. Next target upper descendant occupancy, transverse needle coverage along secondary supports, and unobscured connected polygonal socket views. Preserve all seeds/ranges and no global count/length bump. No PR, merge, completion or owner approval.
## Evidence

- Implementation: `99112c4`; exterior cameras `5eb89c6`; conservative detail frustum capture `4af35df`, verified by three exact PNG/geometry comparisons.
- Numeric: `.flow/evidence/fn9/pass4-numeric.json`; all 48 retained cases pass with frozen seeds/ranges unchanged.
- Gates: `.flow/evidence/fn9/pass4-gates.json`; native workspace, 106 harness tests, fmt, Clippy `-D warnings`, typecheck, build, isolated Wasm/browser, diff and Flow validation.
- Visuals: `.flow/evidence/fn9/REPORT.md`, `pass4-visual.json`, `pass4-captures.json`, `pass4-contacts.json`, `pass4-shared.json`, `pass4-protected-oaks.json`, `qa-preview/pass4-*.png`. Bulk `/tmp/fn99-fix4-qa`; numeric `/tmp/fn99-fix4-final-replay`; prior report `REPORT-pass3.md`.
- Diagnosis: `pass4-oak-audit.json` and `pass4-curtain-audit.json` record scaffold redistribution and secondary bearing/bare length/overlap. `pass4-costs.json` records native samples and memory.
- Next: preserve accepted oak output and upper retention; trace upper local-descendant occupancy between remaining crown masses. Measure transverse needle coverage/continuity along S-BRANCH supports before revising hierarchy. Resolve unobscured actual peg/socket contact at retained exact targets. Preserve every seed/range; no global foliage-count or length substitute.
- PRs: none; blocked and ineligible for make-pr.
