# fn-353 before: the baseline the later tasks compare against

Recorded 11 Nov 2026 by task 1, before any change, in `.worktrees/fn-353`.

- `git rev-parse HEAD`: `8a3a20b74056d4878cb5072f7b157b2395d20937`

## Lines under `crates/`, `src/`, `harness/` (measured, `wc -l` over every file of the extension)

| Extension | Lines |
|---|--:|
| Rust (`.rs`) | 202,868 |
| WGSL (`.wgsl`) | 23,756 |
| TypeScript (`.ts`) | 12,637 |
| TypeScript JSX (`.tsx`) | 780 |

Command: `find crates src harness -name "*.<ext>" -not -path "*/target/*" -print0 | xargs -0 cat | wc -l`.

## The unchanged-tree check (measured)

The still runner, `cargo build --profile ci -p telperion-render --example space_oak`, run through `telperion-heavy 12 --gpu 2 --threads 8 -- target/ci/examples/space_oak <dir> 40 --seeds 1001 --shots none`:

```
age 40 seed 1001: grown in 576.5 ms, dressed in 144 ms; 272095 nodes, 47279 leaves; 10.6 m tall, 7.6 x 8.0 m; wood 2.72 km, fine 2.45 km
mesh 6009038 wood vertices, 11914960 wood triangles, 47279 leaves
```

**Oak at 40, seed 1001: 272,095 nodes, 47,279 leaves.** The log is `raw/still-oak-40-1001-before.log` (ignored).
