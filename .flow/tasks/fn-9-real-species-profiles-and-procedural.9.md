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
- [ ] Native workspace tests, Wasm checks, harness tests, formatting/clippy, typecheck, build and relevant headless integration all pass; evidence remains compact. Latest isolated browser assertions pass, but recorder exit 143 leaves its child exit unassessed; earlier explicit exit 0 has a mixed-log limitation.

## Done summary

Blocked after pass-3 implementation and personal inspection. Production `d797508` passes all 48 retained numeric cases; spruce 4250668600 width is 8.685860226 m <= unchanged 9.144 m. Upper-support shedding demonstrably stripped oak foliage-bearing descendants; targeted retention repairs the upper sprays without changing lower shedding. Three required oak crown cases still fail (2, 3, 2666899686). Spruce descending hierarchy and terminal radius improve, but all seven inspected spruce crowns still lack substantial curtains. Seed-1 actual twig taper passes only supplemental inspected endpoints; complete socket/peg contact remains unassessed. All 70 final images are inspected, and all nine shared geometry/PNG hashes exactly match pass 2. No PR, merge, completion or owner approval.

Blocked:
Pass 3 remains visually blocked after 70 personally inspected images: oak 2, 3, 2666899686 crown gaps/mass fail; all seven required spruce crowns lack substantial hanging curtains; complete socket/peg contact unresolved. Numeric 48/48 pass, retained spruce 4250668600 width 8.685860226 <= 9.144 m, nine shared geometry/PNG hashes unchanged. Evidence: .flow/evidence/fn9/REPORT.md and pass3-*. No PR, merge, completion or owner approval. Next target remaining scaffold spacing, foliage-bearing secondary hierarchy, and actual rendered surface contact; preserve every seed/range.

## Evidence

- Implementation: `d797508`; exterior runner correction `2dfa2d2`; numeric/source/cost evidence `0a29b99`.
- Tests: 48 retained numeric cases, native workspace, 106 harness tests, fmt, Clippy `-D warnings`, typecheck, build; explicit final browser and cheap-check receipts in `pass3-gates.json`. Capture runner exit 1 preserves unassessed automated visual approval.
- Visuals: `.flow/evidence/fn9/REPORT.md`, `pass3-visual.json`, `pass3-captures.json`, `pass3-shared.json`, `qa-preview/pass3-*.png`. Bulk `/tmp/fn99-fix3-qa`; numeric `/tmp/fn99-fix3-final-replay`; prior report `REPORT-pass2.md`.
- Diagnosis: `pass3-scaffold-audit.json` records retained/lost upper descendants for all 13 visual specimens.
- Next: preserve upper retention; target remaining oak scaffold spacing/directions; inspect spruce foliage-bearing versus bare secondary lengths and overlap before revising hierarchy; resolve actual polygonal surface/socket contact. Preserve all seeds and frozen targets; no global foliage-count substitute.
- PRs: none; blocked and ineligible for make-pr.
