---
satisfies: [R1, R3, R4, R5]
---
# fn-19-comparative-botanical-geometry-benchmark.4 Publish the baseline assessment and verified revision replay

## Description
Run the frozen population, assemble the comparative assessment and independent review packet, and document reproducible reruns.

Integrate task 5's onboarding guide/template links into the README and replay guide. Verify its additional-species admission fixture through the comparison tooling without changing the original cohort or claiming the fixture is an implemented species. Document which agent stages can run concurrently and which shared integration/measurement steps require coordination.

**Size:** M
**Files:** tests/browser/geometry-benchmark.mjs, tests/browser/geometry-benchmark.test.mjs, .flow/evidence/fn19/REPORT.md, .flow/evidence/fn19/final/**, scripts/benchmarks/geometry-compare.mjs, tests/browser/geometry-compare.test.mjs, README.md, tests/migration/README.md
**Touches:** [tests/browser/geometry-benchmark.mjs, tests/browser/geometry-benchmark.test.mjs, .flow/evidence/fn19/REPORT.md, .flow/evidence/fn19/final/**, scripts/benchmarks/geometry-compare.mjs, tests/browser/geometry-compare.test.mjs, README.md, tests/migration/README.md]

### Approach
- Integration fix discovered by real replay: compare frozen render/view rules semantically, independent of JSON object key order, in the capture adapter. Add a focused reordered-key acceptance / changed-value rejection regression. Preserve the completed mature run and original tool identity; pin the corrected tool consistently for a new small replay pair. This bounded fix does not change any frozen rule value or camera.
- Freeze the final executable source identities, then run all 12 protocol cases with no source changes. Reuse historical evidence only after full identity checks; otherwise capture new artifacts with historical links. Store bulk output outside the repo and retain compact receipts/previews/hashes for reproducibility.
- Implement one baseline/candidate report combiner consuming tasks 2/3 receipts. Validate full case sets, protocol/reference identity and comparable view/cost domains. Compare an unchanged-source replay and fixture-based changed/failed cases; no unrelated production geometry change is required to prove revision comparison.
- Inspect required views and compare to admitted real/competitor references. Rank discrepancies with anatomical location, severity, confidence, source/view IDs and whether evidence supports a geometry or rendering cause. Preserve disagreements and weak evidence rather than assigning one aggregate SOTA score.
- Assemble the independent botanical review packet. Incorporate genuine feedback if available, separately attributed from implementation judgments. With no qualified response, publish the engineering baseline as incomplete for independent biological assessment and preserve the pending portion of R3; do not contact anyone automatically or treat task/report completion as expert approval.
- Record cost observations and unavailable/contended domains. Only run qualified timing in coordinated exclusive windows; no mandatory performance rerun is needed to deliver clearly labeled unavailable timing.
- Add a concise README entry and detailed migration-guide replay section. Explain benchmark versions, used-up holdouts, changed-revision replay, failure/unassessed states and where fn20/fn21 find stable evidence IDs. Keep fn9 history unchanged.

### Investigation targets
**Required:**
- .flow/evidence/fn9/REPORT.md
- .flow/evidence/fn9/final/visual.json
- .flow/evidence/fn9/final/numeric.json
- .flow/evidence/fn9/final/costs.json
- README.md:90-125
- tests/migration/README.md:89-180

### Quick commands
- cargo test --release -p telperion-core --test geometry_benchmark --test species_metrics
- node --test tests/browser/geometry-benchmark.test.mjs tests/browser/geometry-compare.test.mjs
- npm run typecheck
- Run the implemented baseline/candidate CLI against the same pinned small source and deliberately mismatched fixtures. Run full browser integration once only if a shared entrypoint changed.

## Acceptance
- [ ] All frozen cases and required views have terminal measured/captured/failed/unavailable records, with inspected findings distinguished from collection success and missing expert feedback.
- [ ] Report ranks actionable discrepancies for fn20/fn21 and qualifies every competitor/real-tree claim by admitted matching level; no unmatched or absent comparator implies a win.
- [ ] Comparison controls reject changed protocol, missing/duplicate case, stale source/binary identity, interrupted output and incomparable camera/cost conditions; unchanged replay succeeds and deliberate metric/image changes are surfaced.
- [ ] Independent review packet and actual feedback status are preserved; absent expert assessment leaves R3's independent portion explicitly unresolved and prohibits unqualified biological superiority.
- [ ] README and replay guide point to reproducible commands/evidence; focused checks pass and historical fn9 evidence stays unchanged.

## Done summary
Implemented the mature botanical geometry baseline, validated comparison tooling, replay documentation, and evidence report. All 12 mature cases and 84 visual views completed; an independent small execution pair under the corrected capture tool reproduced all 42 views and six native cases without metric, image, geometry, or gap drift.

Integrated by the conductor; focused verification passed. Commit: e2fe534c42195ddf2876f670ef2a4d627637ccee. Baseline: green (species_metrics five tests and typecheck). Final worker gates: nine focused Rust tests, 16 Node tests, typecheck, ten comparison controls, and 22 onboarding checks passed. Detailed commands, red-to-green evidence, logs, and receipts are committed in .flow/evidence/fn19/final/gates.json. Conductor full-suite receipts are explicitly pre-task4 integration (72 Rust passed/six ignored and 106 npm tests), not worker post-integration claims.

The authorized capture integration fix compares object semantics independent of JSON key order while retaining ordered arrays and exact values. Mature receipts retain historical tool identity bda8f89c510bb83e95720feaa6d5dc30207e0d3d39c5b1caa24fa8cfac1e140b; the fresh replay pair consistently uses corrected identity ee62dad3861837b668b6ceee1dade9f92a788d85e783d53512bf35454be78662. Frozen generator source revision is b5249e16c7e2c10f88f8409294fce38c0ddfb435. Native timing is skeleton generation only. Costs remain separately inconclusive; independent botanical assessment remains unassessed (R3 independent portion unresolved), and no competitor superiority is claimed.

Report: .flow/evidence/fn19/REPORT.md. Bulk mature and replay artifacts are preserved under the common git directory's flow-artifacts/fn19-baseline and flow-artifacts/fn19-replay with original paths retained as symlinks. Committed preservation.json and replay-preservation.json document verified hashes. All capture and replay readers are finished; failed and interrupted attempts remain available. No shared Flow state was modified.

stage: impl-review - skipped(config: REVIEW_MODE=none)
stage: plan-sync - skipped(config: planSync.enabled=false; conductor owns lifecycle)

Conductor integrated-target verification: 16 Node capture/comparison tests, 22 onboarding checks, TypeScript checking and Rust formatting passed. Full workspace gates run separately at spec quiescence.
## Evidence
- Commits: e2fe534c42195ddf2876f670ef2a4d627637ccee
- Tests: baseline: green; cargo species_metrics (5 passed) and npm run typecheck before implementation, cargo test --release -p telperion-core --test geometry_benchmark --test species_metrics — exit 0; 9 passed; /tmp/fn19-task4-final-rust.log, node --test tests/browser/geometry-benchmark.test.mjs tests/browser/geometry-compare.test.mjs — exit 0; 16 passed; /tmp/fn19-task4-final-node.log, npm run typecheck — exit 0; /tmp/fn19-task4-final-typecheck.log, python3 -B .flow/evidence/fn19/final/replay-controls.py --native-controls /tmp/fn19-task4-native-controls --output /tmp/fn19-task4-comparison-controls-post-capture-fix — exit 0; 10 controls, Mature frozen protocol: 12/12 native cases and 84/84 visual views passed; complete terminal receipts retained, Actual unchanged-source v3 replay: 6 native cases and 42 visual views per side; comparable with zero metric/image/geometry/gap changes, Changed-image replay fixture detected exactly one image change; interrupted v2 evidence rejected as inconclusive, Semantic capture-rule reordered-key regression observed red before fix, green afterward; changed values and ordered arrays rejected, Onboarding validation: 22 checks passed; unsupported third species remains separate, git diff --check; scoped changes, report links, retained artifact hashes and historical fn9 preservation verified, Integrated e2fe534 target: Node16/16, onboarding22checks, npm run typecheck and cargo fmt --all -- --check passed
- PRs: