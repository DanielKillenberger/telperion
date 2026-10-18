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
