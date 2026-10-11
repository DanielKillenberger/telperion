# fn-255 friction

## 2026-10-11, task 1: start
- **Doing:** the Phase-1 anchor in the `light` worktree.
- **Hindered:** `flowctl` is not on PATH in the worker's shell; found under the plugin cache by hand (fn-254 reported the same).
- **Cost:** about 1 minute.
- **Would remove it:** `flowctl` on PATH for dispatched workers (local setup, not a repository change).

## 2026-10-11, task 1: the lattice-scale candidate runs
- **Doing:** the frozen and accumulated comparisons of the volumes at the lattice's scale (leaf 11.75) on the oak at 80.
- **Hindered:** each run takes about 9 minutes, all of it in the candidate's reads (522 s frozen, 476 s grown), against 0.5 s for the lattice. Run only once (seed 1) for that reason.
- **Cost:** about 20 minutes of waiting.
- **Would remove it:** nothing in tooling; the slowness is the finding (R3.md).

## 2026-10-11, task 1: the amplification probe compared axes by lineage
- **Doing:** re-taking fn-254's light-to-totals gain on the cycle.
- **Hindered:** matching axes by lineage made two identical runs look different from cycle 3 (a lineage can name more than one axis), which first read as an apex flipping on a sky change of 1e-7. An identical-input control exposed it; axes are now compared by index.
- **Cost:** about 10 minutes and two extra runs.
- **Would remove it:** a documented unique key per axis (lineage plus its place in the chain) for tree diffs.

## 2026-10-11, task 1: small shell and tool snags
- **Hindered:** the dcg hook refuses `>` to a path held in a shell variable (literal paths only); `/usr/bin/time` is absent (peak memory via a Python wrapper); system Python has no numpy (ran the sheet through `uv run --with numpy`).
- **Cost:** about 5 minutes in all.
- **Would remove it:** local setup, not a repository change.

## 2026-10-11, task 1: restoring a throwaway patch reverted my own edits
- **Doing:** reverting the throwaway stills route in `grow.rs` with `git checkout` after building the renderer for early test 2.
- **Hindered:** `grow.rs` also held uncommitted edits of this step (the deposit and census fields), which the checkout discarded; the next test build failed on 16 errors. The results were from binaries built before the revert; re-applied and checked that the rebuilt code reproduces them.
- **Cost:** about 5 minutes.
- **Would remove it:** apply throwaway patches only on a committed tree, or reverse them with `git apply -R` of the saved patch.

## 2026-10-11, task 1: no sampling profiler for the cycle's cost
- **Doing:** finding where the cycle's Units and Sketch time goes against F4-Grow.
- **Hindered:** `samply` needs `perf_event_paranoid` at 1 or below (it is 2) and there is no sudo, so the cost is read from the grower's stage timers and a throwaway floor probe instead of a profile.
- **Cost:** about 10 minutes; the per-unit cause stays inferred.
- **Would remove it:** `kernel.perf_event_paranoid = 1` on the owner's machine (local setup, not a repository change).
