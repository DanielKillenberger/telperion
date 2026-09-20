---
satisfies: [R1, R2, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.3 Measure wood construction stages before selecting the next GPU boundary

## Description
The completed-frame browser matrix still spends 229–255 ms in oak wood construction and 165–172 ms in spruce. Before selecting another GPU stage, measure where this time goes. This is a bounded diagnostic task under the unchanged fn-91 target; it does not close the parent performance requirements.

**Touches:** .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/** (diagnostic scripts, scratch-source patch, raw JSON and report only).

Prepare a reproducible temporary checkout at the measured revision. Instrument the actual surface::build implementation in that scratch checkout only, preserving operation order and geometry. Add a native diagnostic example there that generates each solved tree once and repeatedly measures wood construction. Report validation/path/allocation preparation, radius-ranking sample pass and sort, emission sample/frame preparation, vertex/coordinate emission, index/cap emission, normals accumulation/normalization, and final bounds. Also time the existing pure CPU wood/radius.rs::radii recovery on the resulting mesh as a separate submission-side cost (reuse the source in the scratch example; no GPU renderer required). Aggregate per-run durations rather than printing per run. Include original uninstrumented control measurements using the same preset/seed/build profile; quantify timer overhead and do not attribute it to a production stage. If instrumentation overhead is material, report its limitation rather than repeating an open-ended profiling effort.

Native release oak and spruce seeds 1/7, one first plus three warm builds each is sufficient. Hash all output surface arrays, bounds, dropped count and run table; exact output parity against the control is required because instrumentation is observational. Preserve source revision, patch/script, commands, compiler/profile, hardware, counts, warm medians and stage totals. No screenshots, no full generator leaf builds, no whole-workspace tests. Do not install a profiler or change local system configuration. Work after .2 measurements finish to avoid timing interference.

The host owns the next implementation design. Return measurements and a bounded interpretation of the dominant stages; do not implement a GPU wood pipeline or change the tree shape. Report what fraction of wood time and end-to-end time a chosen stage could at most remove, explicitly as an upper bound, not a promised speedup. Record friction immediately. Remove no existing worktrees or historical evidence.
## Acceptance
- Reproducible native release wood-stage profile covers oak/spruce seeds 1/7 with matching output hashes across instrumented/control runs, or explicitly reports why a reliable profile cannot be obtained.
- Report includes stage costs, complete wood time, measurement overhead, allocation/lifetime implications visible in the source, and the boundary a next candidate would have to replace. No isolated-stage gain is called end-to-end qualification.
- Product source/API/default behavior remains unchanged; diagnostic code and raw results are retained as evidence with revision and commands.


## Done summary
Measured native release wood stages for oak/spruce seeds 1/7 with scratch-only instrumentation. All 32 builds match complete surface hashes; report, raw data, reproducible driver and source patch live in `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile/REPORT.md`. Material observed overhead of 6.5–21.3% limits exact attribution; parent performance acceptance remains open.

baseline: none (no Quick commands; task excludes whole-workspace suites).
Verification: control and instrumented release builds passed; analyzer checked 32 samples, four fixture hashes and bounded stage accounting; Bash/Python syntax and git diff checks passed. Gate classify returned docs-only; no defined full gate was skipped or fabricated.

Tier: session (jev long_running 0.14) (explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)

Frictions are recorded in FRICTION.md; host-authored FRICTION-REVIEW.md included. Product source/API/default behavior is unchanged. The host owns next-boundary design and parent .1 remains in_progress.

Host direct inspection completed; corrected the oak upper-bound wording to include other preparation plus submission (255/287 ms after removing wood). No review backend was dispatched.
## Evidence
- Commits: 92d1c025641de2efccdce73bb2e79cc34994ed09
- Tests: baseline: none, bash .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile/run.sh (control and instrumented release builds; 32 samples), python3 .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile/analyze.py (32 samples; four full-surface parity hashes; stage accounting passed), bash -n .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile/run.sh, python3 -m py_compile .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile/analyze.py .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/wood-profile/instrument.py, git diff --check, flowctl gate classify --base b0c38c259bb733f1b85b137f4cf36916e4352724 (TIER_B; no defined full gates)
- PRs: