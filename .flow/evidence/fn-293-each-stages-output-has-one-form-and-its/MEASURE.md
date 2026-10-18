# fn-293 measurements (R5)

The before-numbers, taken at `bf84ef6ca` (the plan commits over
`f8c014681`; no source differs) in a temporary detached worktree of that
commit. Task 7 takes the after-numbers the same way at the spec's head.

## Non-test lines per crate

`measure-lines.sh` (beside this file), from the checkout's root: every `.rs`
and `.wgsl` file under each crate's `src/`, less `*_tests.rs`, `tests.rs` and
anything under a `tests/` or `suite/` directory.

| crate | rs | wgsl |
| --- | ---: | ---: |
| telperion-core | 27962 | 0 |
| telperion-jev | 27779 | 0 |
| telperion-render | 31799 | 21711 |
| telperion-space | 27197 | 0 |
| telperion-wasm | 525 | 0 |

## Shipped artifacts

The package build the size budget reads (`npm run build`'s steps, the files
`scripts/artifact-budgets.json` names), measured by `measure-build.sh
<checkout>`; gzip is `gzip -9`. Logs: `raw/measure-base.log`,
`raw/measure-base-2.log` (ignored, on disk).

| artifact | raw bytes | gzip bytes |
| --- | ---: | ---: |
| dist/telperion.wasm | 1568583 | 517048 |
| dist/telperion-render.wasm | 3571814 | 1139270 |
| dist/telperion.js | 193862 | 17821 |
| dist/field.js | 1917 | 963 |
| dist/wasm-source.js | 64437 | 15556 |
| dist/voxelize.js | 4362 | 1837 |

## Release Wasm build time

Wall time of each build step from a fresh `target/` (removed before each
run; no compiler cache), 32 threads, other workers' jobs running beside it
(load average 3 to 4). Two runs:

| step | run 1 ms | run 2 ms |
| --- | ---: | ---: |
| `npm run wasm:build` (telperion-wasm, release, wasm32) | 17523 | 17362 |
| `npm run render:build` (telperion-render, release, wasm32) | 30868 | 30971 |
| `tsc -p tsconfig.build.json` | 1136 | 1173 |
| `vite build` | 518 | 522 |

The device build's time on the spruce and oak (paired warm, 16 runs) is
task 6's, before and after its change.

## Task 2: the space crate

`measure-lines.sh` after task 2 (the experimental growth passes, their
`Grower` hooks and examples deleted; the kept recursion, the boundary rows
and `agree` compiled only into tests):

| crate | rs before | rs after |
| --- | ---: | ---: |
| telperion-space | 27197 | 21643 |

The script counts files by name, so it still counts the modules now gated
whole by `#[cfg(test)]` but not named as tests: `closed_form/kept.rs`,
`closed_form/kept/{bound,ends,mass,path,section}.rs`,
`closed_form/boundary.rs` and `agree.rs`, 1563 lines. Less those, about
20080 lines of the space crate compile into its non-test build.

## Task 3: the scaffold grower and `Tree.pipe`

`measure-lines.sh` counts files by name and so still counts the scaffold's
modules now gated whole or in part by `#[cfg(test)]`; it reads 27962
before and 27994 after (the gates and comments add lines).
`measure-cfg-lines.py` (beside this file) walks the module tree from
`lib.rs`, skips each `#[cfg(test)]` module and drops each `#[cfg(test)]`
item, so it counts what compiles into the non-test build:

| crate | non-test lines before (`e8f58a85e`) | after |
| --- | ---: | ---: |
| telperion-core | 25106 | 22884 |

## Task 4: one sweep a mesh request

`Request::mesh()` on the CPU, `--profile ci`: the `generation_stages`
example built at the task's base (`a7b254ef7`) and after finding 7
(`1b25167cc`), run alternately (the order flips each pair), seed 1, 16
pairs a preset. Each process builds twice and the warm second build is
kept; the mesh time is the build's total less its growth. Both binaries
print the same output hash on every run. Rows:
`raw/t4-mesh-paired.jsonl`; script `paired.sh` in the task's scratch.

| preset | base median ms | after median ms | paired difference, median | after faster |
| --- | ---: | ---: | ---: | ---: |
| norway-spruce | 4166.2 | 4087.9 | -81.7 ms (mean -75.3, sd 104.4) | 14 of 16 |
| oregon-white-oak | 153.4 | 139.1 | -15.3 ms (mean -15.0, sd 4.7) | 16 of 16 |

The trace now runs inside the rings stage, so `rings_ms` rises (spruce
about 254 to 440 ms) while the separate trace after both threads joined is
gone. The spruce's mesh is bound by its placement (about 3.4 s on the
leaves' side), so the saving there is about 2 %; the oak's is about 10 %.

Non-test compiled lines of telperion-core (`measure-cfg-lines.py`): 22884
after task 3, 22997 after task 4 (the curve-drawn ring entry and
`Scratch::read` add more than the station count and the preparation remove).

## Task 8: validation and the delivery enums

Non-test compiled lines (`measure-cfg-lines.py`), at the task's base
(`fc12f911a`) and after: `validate_skeleton` without the scaffold-only
preconditions (the scaffold module and `inner_envelope` now compile only
into tests), and `Delivery` and `Backend` deleted.

| crate | non-test lines before | after |
| --- | ---: | ---: |
| telperion-core | 22997 | 22825 |
| telperion-render | 23525 | 23499 |

## Task 6: the device build before and after

`raw/t6-build-time.txt`, from `measure-t6-build-time.sh` (beside this
file): `build_time` with `BUILD_PRESETS=norway-spruce,oregon-white-oak`.
Two render test binaries were compared: the base (`2f437cbd4`, before the
change) and after. They ran alternately, 16 pairs, with the order flipped
every pair. Each run timed the second (warm) build in its process, at the
presets' default ages. Another worker's GPU jobs kept the card 43-98 %
busy in 14 of the 16 pairs, so only the paired differences mean
anything. RTX 3080.

| preset | base warm median ms | after warm median ms | paired diff median ms | after faster |
| --- | ---: | ---: | ---: | ---: |
| norway-spruce (40) | 446 | 427 | -15 | 15 of 16 |
| oregon-white-oak (40) | 236 | 231 | -5 | 13 of 16 |

**GPU peak bytes** (the ledger's build peak, `build_time`'s census):
every shipped preset's peak is the same before and after: spruce 1587 MB,
oak 872, ordinary 180, beech 296, birch 577, date palm 34, Telperion 2690
and Laurelin 2470. The build's peak is set by the growth cycle, not by the
drawing stages. The drawing stages no longer hold the 39-word node table
(156 bytes a drawn id) beside the drawn records. The one drawn-record
buffer takes the same words as the per-wave buffers it replaces, plus the
leaf bases' room: two records a base on every axis of the seed's wave,
which only the date palm holds, and its peak did not move.

## After: the spec's head (task 7)

Taken at `b36086f05`, the commit after which task 7 changes only docs,
specs and evidence, so its source is the spec head's. The builds ran in a
temporary detached worktree of that commit, as task 1's did, under
`telperion-heavy 12`. Logs: `raw/measure-after.log` and
`raw/measure-after-2.log` (ignored, on disk).

### Non-test lines per crate

`measure-cfg-lines.py` counts what compiles into each crate's non-test
build (Rust only). The before column is that script on task 1's base
(`bf84ef6ca`, the `crates/` tree from `git archive`). `measure-lines.sh`
counts by file name, so it still counts modules gated whole by
`#[cfg(test)]`; its before column is the table above.

| crate | non-test rs before | after | change | by file name, rs before | after | wgsl before | after |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| telperion-core | 25106 | 22824 | -2282 | 27962 | 28079 | 0 | 0 |
| telperion-jev | 25573 | 25573 | 0 | 27779 | 27779 | 0 | 0 |
| telperion-render | 24167 | 23393 | -774 | 31799 | 31615 | 21711 | 21707 |
| telperion-space | 26561 | 18825 | -7736 | 27197 | 21761 | 0 | 0 |
| telperion-wasm | 504 | 473 | -31 | 525 | 494 | 0 | 0 |
| total | 101911 | 91088 | -10823 | | | | |

The by-file-name WGSL count barely moves. The test-only entries moved into
`positions_admit.wgsl` (task 5), which the count still includes, and task 6
replaced the handoff pass's WGSL with the sweep's own packing
(`pack.wgsl`).

### Shipped artifacts

| artifact | raw before | raw after | change | gzip before | gzip after | change |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| dist/telperion.wasm | 1568583 | 1463135 | -105448 (-6.7 %) | 517048 | 480780 | -36268 (-7.0 %) |
| dist/telperion-render.wasm | 3571814 | 3399922 | -171892 (-4.8 %) | 1139270 | 1076221 | -63049 (-5.5 %) |
| dist/telperion.js | 193862 | 193862 | 0 | 17821 | 17821 | 0 |
| dist/field.js | 1917 | 1917 | 0 | 963 | 963 | 0 |
| dist/wasm-source.js | 64437 | 64437 | 0 | 15556 | 15556 | 0 |
| dist/voxelize.js | 4362 | 4362 | 0 | 1837 | 1837 | 0 |

Both runs gave the same bytes. No ceiling in `scripts/artifact-budgets.json`
moves here (fn-242).

### Release Wasm build time

Wall time of each step from a fresh `target/`, 32 threads. Load average
was 1.0 to 2.4 here, against 3 to 4 at the base, so the after-runs had a
quieter machine.

| step | before run 1 ms | before run 2 ms | after run 1 ms | after run 2 ms |
| --- | ---: | ---: | ---: | ---: |
| `npm run wasm:build` | 17523 | 17362 | 15362 | 15438 |
| `npm run render:build` | 30868 | 30971 | 30028 | 29896 |
| `tsc -p tsconfig.build.json` | 1136 | 1173 | 1247 | 1196 |
| `vite build` | 518 | 522 | 601 | 524 |

The main module's build is about 2.0 s faster (-12 %), and the
renderer's about 1.0 s faster (-3 %). Part of that may be the lighter
load.

### The tasks' timings

- **Task 4 (one sweep a mesh request, CPU):** spruce 4166 to 4088 ms
  median (paired -82 ms, after faster in 14 of 16). Oak 153 to 139 ms
  (-15 ms, 16 of 16).
- **Task 6 (the device sweep reads the drawn records):** spruce 446 to
  427 ms warm (paired -15 ms, 15 of 16). Oak 236 to 231 ms (-5 ms, 13 of
  16). GPU peak bytes are equal on every preset.
