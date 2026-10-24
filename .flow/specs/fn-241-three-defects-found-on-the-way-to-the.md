# Three defects found on the way to the release

## Goal

The release map (fn-237) carried three defects found while building the release; this spec fixed them before the release PR (host, 2026-10-09). Release blocker for fn-237; these three defects only.

## Decisions

- R1 cause: the drawn wood already agreed (`mesh::build`, the wood surface and the curve's rings all reach the same lowest point); the difference was the fine path's bounding box, the box of every cluster's sphere, hanging up to 2.6 m below ground (oak at 80). Host decision (2026-10-09): tighten the box from each ring's own extent on both the CPU curve (`Curve::bounds()`) and the device handoff (`handoff.wgsl`), in the existing pass; no drawn byte changes, framing, shadow fit and figure height move to the true tree. Built as the ring's extent in its own plane, `sqrt(normal_k² + binormal_k²)` per axis.
- R2: growth is almost all of the field test's time and the two chains it compares cannot share it; the test became one test a preset with the modules shared, the 180,000 ms override removed.
- R3, host decision (2026-10-09): delete the 5 s aggregate first-tree bound ("Question, delete, then optimise"); Chrome's watchdog acts on one pipeline's creation, so the per-pipeline 2.5 s bound and `grow`'s 20 s exception stay asserted and the first tree's total is reported. The two output-preserving changes stay (place threshold log-odds from the CPU table; one `lead_q`/`presence` call site in `take_ask`, `asks` 1.07 → 0.87 s). No further shader slimming, no concurrent creation here.

## Acceptance Criteria

- **R1:** the device tree reaching below ground (0.227 m against 0.040 m) reproduced as a test, cause found and fixed so both paths agree within fn-125's screen — met.
- **R2:** the field test finishes well inside its 60 s timeout without losing what it proves — met.
- **R3:** (amended) aggregate cold-build bound deleted and reported; per-pipeline bounds asserted — met.
- **R4:** gate and npm test green; shipped species byte-identical — met.

## Result

- R1: `ground_tests::the_fine_path_stands_where_mesh_build_does`, red on the base, green after: ground offset ≤ 2.7e-9 m and largest corner ≤ 1.45e-4 m against screens of 1.7e-4 to 7.3e-4 m; at 40 by default, at 80 as a device-limits probe.
- R2: longest per-preset test 19.1 s (spruce) against its 60 s timeout; the file still takes ~66 s in all.
- R3: test renamed `every_pipeline_within_2_5_s_but_grow_within_20_s_cold`; first tree cold 23.34 s, 4.65 s besides `grow`.
- R4: render half 216 passed / 41 ignored; workspace half 1,065 passed / 50 ignored; npm test 150 passed; `space_hash` 16 rows equal to the merged bases'.

## Open

- Impl-review P2: `CurveCluster::bounds` encloses the stored rings but not Hermite-tessellated wood between them on strongly bent stretches, nor shaped sections' cylinder-wrapped vertices; shipped species measure within the screen. A general fix would bound the interpolated envelope and shaped runs.

## Friction

- Fresh worktree builds cold before the first measurement -> the separate-target rule's price; nothing to change.
- First R1 instrument ran out of device memory -> instrument drops each tree before the next.
- `cargo fmt -p` rewrote 26 untouched files (~5 min) -> rustfmt-clean workspace or CI format check proposed; `rustfmt --check` on changed files meanwhile.
- An instrument hung holding the GPU lock (~12 min) -> `timeout` on every GPU instrument run; hang not diagnosed.
- Eight-tree bounds test ran spruce at 80 out of device memory -> device poll after each drop, now in the test.
- R3's named route bought 0.2 s of 1.4 s (~15 min) -> a per-pipeline cold-compile budget names the shader at fault; host deleted the aggregate bound.
- R1 test at 80 ran out of device memory in the gate -> moved to a device-limits probe; `device_limit_probes_run_only_when_asked` could also flag default tests naming age 80.

## Evidence

- `.flow/evidence/fn-241-three-defects-found-on-the-way-to-the/FRICTION.md` (cited by fn-237)
