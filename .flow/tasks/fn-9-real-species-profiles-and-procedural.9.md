---
satisfies: [R2, R3, R4, R5, R6]
---
# fn-9-real-species-profiles-and-procedural.9 Run cross-seed visual QA and document the species workflow

## Description
Run cross-seed visual QA and document the species workflow. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `tests/browser/species.mjs`, `package.json`, `README.md`, `tests/migration/README.md`, `.flow/evidence/fn9/REPORT.md`
**Touches:** [crates/telperion-core/src/branching.rs, crates/telperion-core/src/branching/**, crates/telperion-core/src/foliage/**, crates/telperion-core/src/presets.rs, crates/telperion-core/tests/**, tests/browser/species.mjs, scripts/*species*, package.json, README.md, tests/migration/README.md, .flow/evidence/fn9/**]

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

### Resumed architecture correction (2026-09-06)
The owner requested cleanup and completion after fetching pass 7. R6 authorizes targeted shared-rule corrections. The measured seed-2 window cannot be reached by rotations of frozen upper descendants. Inspect the spreading habit's hardcoded structural shell shedding, which can remove already-grown internal subdivisions despite full foliage retention. Spruce places most needle-bearing terminal runs below its descending secondary span; allocate subordinate shoots along that span during growth instead of post-hoc fan transforms. Remove superseded occupancy repairs if the upstream correction replaces them. Add small geometric regressions and re-run frozen numeric seeds and all reference views.

Frozen botanical targets, seed identities and honest geometry/visual evidence remain required. Historical pass reports' unchanged-count/protected-hash/no-lower-infill conditions have no attributed owner requirement in this spec and are superseded by this evidence-driven architecture repair. Record changed counts and outputs; do not weaken profile dimensions, resource/finite checks or natural/supernatural separation. Keep prior evidence recoverable through Git and replace bulky iterative output with a compact reproducible final receipt.
## Acceptance
- [x] All 24 recorded seeds per species have explicit numeric results; required fixed/fresh image sets are inspected against source references.
- [ ] Trait-level evidence supports R3–R6, or outstanding failures remain explicit and prevent task completion; no unearned owner approval is recorded.
- [x] Ordinary and both Two Trees are checked for shared-rule regressions, with same-host CPU/count/size results and renderer identity reported.
- [x] Documented CPU-only replay succeeds, or a concrete environment limitation remains unassessed; missing captures never silently pass.
- [x] Native workspace tests, Wasm checks, harness tests, formatting/clippy, typecheck, build and relevant headless integration all pass; evidence remains compact. Pass-6 isolated Wasm/browser, final mature recorder and angular scan all have explicit exit 0; initial failures remain separately recorded.

## Done summary
Blocked after pass-6 measured empty-reach placement and longitudinal fan redistribution. Production `9e1e889` passes all 48 retained numeric cases; individual foliage and anatomical node counts are unchanged in all cases. All 120 final PNGs inspected. Protected oak geometry and nine whole/bare/detail PNGs exactly match pass 3; all nine shared geometry/PNG comparisons also match. Integrated software gates pass. Botanical AC remains unmet: three oak crowns fail mass/gaps, seven spruce curtains fail, and complete first-target peg/socket remains unresolved. No PR, merge, completion or owner approval.

Blocked:
Pass 6 remains visually blocked after 120 inspected final images. Oak 2/3/2666899686 retain separated upper masses and open windows; all seven required spruce crowns lack substantial S-BRANCH curtains. Complete first-target peg/socket remains unresolved; peg-clear-front is a narrow local relief pass only. Numeric 48/48 and integrated software gates pass at production 9e1e889, including spruce 4250668600 width 8.688966688 <= 9.144 m. Protected oak and all nine shared geometry/PNG comparisons exactly match pass 3. No counts, seeds or profile ranges relaxed. Evidence: .flow/evidence/fn9/REPORT.md and pass6-*. Next: address measured upper-reach limits (oak 2 sampled window about 2.960 m outside feasible radius), target feasible visible windows in oak 3/fresh, improve existing needle-run allocation without structural fork compression, and obtain unobscured original connected peg/socket polygons through elevation/azimuth. No PR, merge, completion or owner approval.
## Evidence

- Report and next steps: `.flow/evidence/fn9/REPORT.md`; pass 5 preserved as `REPORT-pass5.md`.
- Production: `9e1e889`; no profile/seed changes or global foliage increase, no structural fork compression.
- Numeric/counts/costs: `pass6-numeric.json`, `pass6-count-parity.json`, `pass6-costs.json`; bulk `/tmp/fn99-pass6-replay`.
- Software/capture exits: `pass6-gates.json`; native workspace, harness 106, fmt, Clippy -D warnings, typecheck, build, isolated Wasm/browser, diff and Flow validation pass. Initial empty-tree failure fixed before final replay.
- Visual: `pass6-visual.json`, `pass6-captures.json`, `pass6-contacts.json`, `pass6-protected-oaks.json`, `pass6-shared.json`, `qa-preview/pass6-*.png`; bulk `/tmp/fn99-pass6-qa` and `/tmp/fn99-pass6-contact-scan-final`.
- Diagnosis: `pass6-reach-before.json`, `pass6-oak-audit.json`, `pass6-window-rays.json`, `pass6-curtain-audit.json`, `pass6-target-projections.json`. Original shoot membership pinned for needle-versus-wood comparisons; projections confer no seam acceptance.
- Next: address measured upper reach limits, feasible visible inter-mass windows, architectural curtain allocation and unobscured connected first-target polygons while retaining every count/seed/profile/protected hash constraint.
- PRs: none; blocked and ineligible for make-pr.
