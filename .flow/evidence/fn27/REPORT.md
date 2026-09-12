# FN27: coarse shadow casters, early proof stopped

2026-09-12. **NEEDS_HUMAN.** Implementation reached step 3 and stopped on its
early proof point. This is a partial implementation, not completion of R1–R7.
Changes remain in the working tree; no flow state or Git index was changed.

## Blocker

The wood prefix alone, at the specified default threshold, measured a valid
shadow p50 of **1.0629 ms**, above the early proof point's “below about 0.9 ms”
condition. It drew 90,760 wood caster triangles instead of 8,255,000, while all
869,310 foliage placements still cast. Total native p50 was **4.2109 ms**, above
3.8 ms. The early proof clause requires stopping before stride or the kernel
when this happens, so neither was implemented or measured and the task was not
retuned or retried. Continuing requires the owner to resolve that stop. This
total is the prefix-only intermediate measurement, not a completed R3 run.

## Protocol

One headless run, seed 7, whole view, hero pose, 1600 by 1000, default scene row,
four samples per pixel. NVIDIA GeForce RTX 3080, NVIDIA 610.57.04, Vulkan.
Eight conditioning frames, eight warmup frames, 120 measured frames. Verdict:
valid. No retry. No forest capture, receipt, video or frame sequence was opened.
One image was viewed: the prefix-only oak hero.

```bash
cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --out /tmp/flow-handover-fn27/oak-prefix.png --timing /tmp/flow-handover-fn27/oak-prefix-timing.json
```

Command output and the complete timing record were written to
`/tmp/flow-handover-fn27/child-notes.md` before reading. Evidence copies:
[oak still](oak-prefix.png), [timing record](oak-prefix-timing.json).
The timing record retains its existing schema: the two caster fields belong to
step 7, which the early stop prevented. The wood caster count is in the
headless command output and the table below.

## What was implemented

1. Wood emission is ordered by each run's largest sampled radius, descending,
   with stable ties. An additive `run_table` records complete index spans and
   radii, while the existing `runs` count remains unchanged. Submission rejects
   invalid spans, radii, order or incomplete coverage before upload. The table
   stays on the host; no second GPU index buffer is allocated.
2. The four numeric row values share one field table, including the struct and
   defaults as well as validation, JSON and parsing. Defaults are casterTexels
   1, casterStride 4, shadowFilterTexels 1, shadowNormalOffset 1. Only
   casterTexels affects rendering at this intermediate stop; the other three
   are declared and validated but await steps 4 and 5.
3. The wood subject retains the table and binary-searches a single prefix at
   submission and scene-row changes. The threshold is casterTexels times the
   larger fitted map-texel axis in metres. The full draw is unchanged; only the
   depth draw uses the prefix. The renderer exposes the wood caster triangle
   count without adding it to FrameStats.

## What the lever bought

The prefix removed 8,164,240 wood caster triangles. Shadow p50 fell by
0.7501 ms, but did not clear its early gate. Vegetation and selection remain
effectively at fn-14's costs. Stride and kernel have no measured benefit here
because the stop prevented implementing them. Total percentiles rank the
per-frame sum; they are not sums of separately ranked percentiles.

| Pass | fn-14 p50, ms | Prefix p50, ms | Prefix p95, ms |
|---|---:|---:|---:|
| Vegetation | 3.050 | 3.0536 | 3.3659 |
| Selection | 0.087 | 0.0868 | 0.0896 |
| Shadow | 1.813 | **1.0629** | 1.2452 |
| Total | **4.949** | **4.2109** | 4.6579 |

| Geometry | Full count | Prefix-only caster count |
|---|---:|---:|
| Oak wood vertices | 4,262,170 | — |
| Oak wood triangles | 8,255,000 | **90,760** |
| Oak foliage instances | 869,310 | **869,310** |
| Spruce wood vertices (existing identity pin) | 2,888,144 | not measured |
| Spruce wood triangles (existing identity pin) | 5,580,040 | not measured |
| Spruce foliage instances (existing identity pin) | 7,012,326 | not measured |

Oak measured level counts: level 0, 861,024; level 1, 8,286; levels 2–5 and
unseen, zero. Full frame log: 157,776,961 reported triangles, 869,310 instances,
nine calls. These are existing frame statistics, not caster counts; no new
claim about their meaning is made here.

## Native spruce and browser orbits

Step 8 was not reached. No native spruce session or browser orbit was run.
Historical numbers below are comparisons, not fn-27 results.

| Session | fn-14 wall p50 / p95 / max, ms | fn-27 |
|---|---|---|
| Oak browser orbit | 10.00 / 10.10 / 10.20 | not run: early stop |
| Spruce browser orbit | 30.00 / 30.20 / 30.40 | not run: early stop |
| Spruce native | not recorded in fn-14 | not run: early stop |

fn-14's spruce browser shadow p50 was 13.51 ms. Its 60 fps status was not
remeasured. Needle aggregation remains the named follow-on; this partial run
cannot close fn-14's coarse-caster or browser-type/documentation follow-ups.

## Gates and pins

Focused core surface, identity and mesh tests passed. Focused row and renderer
submission tests passed. Existing identity literals held: counts, bounds,
skeleton, placements and element hashes were unchanged. There is no wood-buffer
hash in the current identity test to re-pin, and the small surface tests keep
their existing order. The identity test documents why no literal changed.
No pin or tolerance was loosened.

All four required gates passed:

- `cargo test --release --workspace` — passed, including unchanged clay,
  lit-sky, leaf-offset and view pins. Repeated after the type-location fix;
  both full runs passed. Existing ignored tests remain ignored.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed after the
  type-location fix described below. No lint was suppressed.
- `npm run typecheck` — passed, including the renderer wasm build.

Logs are in `/tmp/flow-handover-fn27/`: `cargo-test-final.log`, `clippy.log`,
`typecheck.log`. The type relocation changed no rendering behavior, so the
early measurement was not repeated. `git diff --check` also passed.

## Deviations

- The mandatory early proof stop leaves steps 4–8 unimplemented/unmeasured and
  step 9's completion documentation undone. This blocker report and empty owner
  slots record that stop; they do not claim the full task completed.
- The current production `mesh.rs` has no validation method at the referenced
  lines. Run-table validation was added to the existing renderer `fits` check,
  and coverage/radius assertions to the core mesh test. Because the table is
  host-only, no GPU allocation/limit entry was added for it. Refusals name
  “wood run” and the defect, but do not include the offending run's ordinal.
- `SurfaceRun` lives beside `SurfaceMesh` in `surface.rs` (388 lines), rather
  than `surface/paths.rs`: the occupancy diagnostic directly includes the
  latter, so declaring the unused type there failed the required all-targets
  Clippy gate. Moving the definition fixes that without a lint suppression.
- No identity literal was re-pinned: the checked-out tests do not hash wood
  buffer order and their existing literals pass unchanged. The reason is
  documented in the test header.
- Capture paths use the explicitly requested handover directory; copies reside
  here. No flow task/spec file was edited, per the workspace rule; owner verdict
  slots therefore live here.

Follow-up outside the named implementation surface: `examples/occupancy_audit.rs`
reconstructs pre-permutation path order to select wood triangles. Its selection
must be adapted before that diagnostic is next used. The ignored FN6
`surface_reference` export test likewise compares legacy buffer order directly;
it is not part of the enabled gates and was not run or weakened here.

## Owner verdict

These remain the owner's words. The prefix-only still is not an R2 filtered
shadow candidate, and an R5 orbit aid does not exist from this stopped run.

### The crown's self-shadow, R2

> _verdict (owner, ):_

### Thinning, stepping and shimmer, R5

> _verdict (owner, ):_

## Aids

- [Prefix-only oak hero](oak-prefix.png) and [its timing](oak-prefix-timing.json).
- [fn-14 oak hero](../fn14/oregon-white-oak-hero.png) and
  [fn-14 report](../fn14/REPORT.md), unchanged.
- `/tmp/flow-handover-fn27/child-notes.md`: all read timing numbers and commands.

## Host verification

The implementer was **gpt-6-astra via `codex exec` at high reasoning effort**,
bridged from the host session; the host wrote no code. The host re-ran all four
gates on the final tree and each passed: `cargo test --release --workspace`,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, `npm run typecheck` (log: `/tmp/flow-handover-fn27/host-gates.log`).
The suite was green at `667bdab` before the bridge, so the green here is this
tree's own. The timing record in this directory is byte-identical to the one the
implementer read, and every number in the tables above matches it.

One image was viewed by the host: a 900 by 420 crop of the ground under the
tree, fn-14's hero above this run's prefix-only hero, at the same crop. The
silhouette, the dapple and the trunk's own shadow read the same in both; the
prefix has not thinned the ground shadow or dropped a limb from it. That is the
second half of the early proof point, and it holds. The first half, the pass
below about 0.9 ms, does not.

**The permutation left one diagnostic silently wrong.**
`crates/telperion-core/examples/occupancy_audit.rs:152-181` walks `paths.runs`
in path order and steps an offset through `mesh.indices` run by run. The closing
`assert_eq!(index_offset, mesh.indices.len())` still holds, because the total is
unchanged, so nothing fails: every run's triangles are now read from another
run's span. The run table this spec adds is the fix, one span lookup instead of
the reconstruction, and it was not taken here. Read no occupancy audit output
until it is.
