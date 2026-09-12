# FN27: coarse shadow casters and a filtered shadow

2026-09-12. **NEEDS_HUMAN: R2 and R5 await the owner.** The native oak
passes R3 at **3.4243 ms total p50**, against 3.8 ms and fn-14's 4.949 ms.
Both browser orbits are valid; oak holds its cadence bounds. The implementation,
occupancy-audit repair, evidence and documentation are in the working tree for
the host. No commit, index operation, task or spec write was made in this pass.

## Protocol and stills

NVIDIA GeForce RTX 3080, NVIDIA 610.57.04, Vulkan natively; Chromium
153.0.8010.12, WebGPU over Vulkan in the browser, reporting NVIDIA Ampere.
Seed 7, whole view, hero pose, 1600 by 1000, four samples per pixel, default
scene row. Each GPU session has eight conditioning frames, eight warmup frames
and 120 measured frames. The browser additionally records ten seconds of a full
orbit on its animation clock, on the 100 Hz display. All four completed sessions
are valid, with no measurement retry. GPU totals rank each frame's sum; they
are not sums of independently ranked percentiles.

The row defaults are casterTexels 1, casterStride 4, shadowFilterTexels 1 and
shadowNormalOffset 1. Stride and kernel radius use whole counts; threshold and
normal offset use the fitted world size of a map texel. Native format support
selected linear comparison filtering. WebGPU does not expose that native
format capability: its adapter line records the requested linear mode, with
backend point fallback possible. The square kernel still averages either mode.
The point fallback was not exercised on this hardware.

One final native capture per tree, one browser orbit per tree, and one untimed
clay still. The browser used the existing `tests/browser/render.mjs` session,
flags and oak cadence gate, through a temporary runner that invoked only the
two requested orbits, with `RENDER_EVIDENCE=.flow/evidence/fn27`. It ran on
**port 5187** with strict port selection. An earlier setup interpreted Vite's
port zero as 5173; it was stopped before a timing record was written and is
recorded in the handover notes. No native or browser measurement was retuned.

Commands and every timing/count record read are in
`/tmp/flow-handover-fn27/child-notes.md`, including the exact temporary browser
runner. Stills and records have copies in that directory. The runner was removed
from the working tree after capture. No forest, receipt, video or frame sequence
was opened. Four images were viewed across this pass: the two lit heroes, clay
oak, and the crown comparison. A crop-gravity mistake in the comparison was
corrected without another view; both final crop regions were verified byte for
byte against their source pixels, with no resize or colour change.

- [Lit oak hero](oak-final.png), beside [fn-14's oak](../fn14/oregon-white-oak-hero.png).
- [Lit spruce hero](spruce-final.png), beside [fn-14's spruce](../fn14/norway-spruce-hero.png).
- [Clay oak](oak-clay.png), beside [fn-14's clay oak](../fn14/oregon-white-oak-clay.png).
- [Ground comparison, fn-14 above and fn-27 below](oak-ground-comparison.png):
  the same 900 by 420 rectangle of the ground under the tree in both, the host's
  R5 aid.
- [Crown comparison, fn-14 left and fn-27 right](oak-crown-comparison.png):
  the same rectangle, x 430–1110, y 220–670, at original pixel scale.
- The settled prefix-only evidence remains [oak-prefix.png](oak-prefix.png)
  and [oak-prefix-timing.json](oak-prefix-timing.json), unchanged.

```sh
cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --out /tmp/flow-handover-fn27/oak-final.png --timing /tmp/flow-handover-fn27/oak-final-timing.json
cargo run --release -p telperion-render --example headless -- --preset norway-spruce --seed 7 --size 1600x1000 --out /tmp/flow-handover-fn27/spruce-final.png --timing /tmp/flow-handover-fn27/spruce-final-timing.json
cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --view clay --size 1600x1000 --out /tmp/flow-handover-fn27/oak-clay.png
```

## What each lever bought

The radius prefix, already committed before this pass, removed 8,164,240 wood
caster triangles and reduced oak shadow p50 from 1.813 to 1.0629 ms, a saving
of 0.7501 ms. The owner lifted the former early-proof stop; the prefix is a
settled health check, not a failing gate.

Stride four reduces oak foliage casters from 869,310 to 217,328, with retained
surfaces scaled by two about their centres and connector vertices left fixed.
With the shared kernel and normal offset also enabled, shadow p50 falls another
0.7905 ms to 0.2724 ms. Vegetation p50 changes from 3.0536 to 3.0638 ms; total
p50 falls from 4.2109 to 3.4243 ms. The stride and filter were measured together:
there is no isolated kernel-cost measurement to infer from that vegetation delta.

The read at radius one averages nine hardware comparisons after moving the
receiver one world texel along its normal. Radius zero is one hardware comparison;
radius three is 49. Every out-of-map tap counts as lit, including at a kernel
boundary. Wood, crown and ground call this same read. The depth pipeline's
constant bias is unchanged. Clay returns its neutral light before reading any
shadow. Whether the resulting crown reads as dapple belongs to R2 below.

There is still one wood index buffer, one placement buffer and one depth map.
The light uniform grows from 64 to 96 bytes for stride, scale, centre and the
connector boundary; the shared frame uniform adds two 16-byte slots. No second
caster selection or compacted instance list is allocated. Reordering placements
changes the stride subset; changing sampled run radii retunes the prefix; future
vertex motion must move both visible surfaces and their casters by the same rule.

## The oak on the clock

| Pass | fn-14 p50, ms | Prefix-only p50, ms | Prefix-only p95, ms | Final p50, ms | Final p95, ms |
|---|---:|---:|---:|---:|---:|
| Vegetation | 3.050 | 3.0536 | 3.3659 | 3.0638 | 3.3382 |
| Selection | 0.087 | 0.0868 | 0.0896 | 0.0868 | 0.0876 |
| Shadow | 1.813 | 1.0629 | 1.2452 | **0.2724** | 0.3011 |
| Total | **4.949** | **4.2109** | 4.6579 | **3.4243** | 3.7484 |

R3 passes with 0.3757 ms remaining below the 3.8 ms bound. Final native record:
[oak-final-timing.json](oak-final-timing.json). fn-14's native total p95 was
5.412 ms. Its historical report remains unchanged.

## The spruce on the clock

This is the first native spruce session under this protocol. fn-14 has no native
spruce baseline; its browser orbit is compared separately below.

| Native spruce pass | Final p50, ms | Final p95, ms |
|---|---:|---:|
| Vegetation | 18.4955 | 18.6399 |
| Selection | 0.6623 | 0.6707 |
| Shadow | 4.2691 | 4.3500 |
| Total | **23.4161** | 23.5945 |

Record: [spruce-final-timing.json](spruce-final-timing.json). Spruce remains over
16.7 ms. It is recorded, not gated; **needle aggregation** is the named follow-on.
The browser's lower vegetation cost is recorded as a target/session difference,
not attributed to a new optimization or treated as an interchangeable native run.

## The orbit, on the clock

| Browser orbit | Wall p50, ms | Wall p95, ms | Wall max, ms | Frames |
|---|---:|---:|---:|---:|
| Oak, fn-14 | 10.00 | 10.10 | 10.20 | 999 |
| Oak, fn-27 | **10.00** | **10.10** | **10.20** | 999 |
| Spruce, fn-14 | 30.00 | 30.20 | 30.40 | 349 |
| Spruce, fn-27 | **20.00** | **20.10** | **20.20** | 538 |

Oak passes R4: wall p95 is below 16.7 ms and every frame is below 33 ms.
Spruce improves its wall cadence but remains below 60 fps. fn-14's spruce
browser shadow p50 was 13.51 ms; this run is 3.1918 ms, down 10.3182 ms.

| Browser orbit pass | Oak p50 / p95, ms | Spruce p50 / p95, ms |
|---|---:|---:|
| Vegetation | 3.1565 / 3.7734 | 14.0567 / 14.2671 |
| Selection | 0.0922 / 0.0932 | 0.6697 / 0.6820 |
| Shadow | 0.2739 / 0.3379 | 3.1918 / 3.2520 |
| Total | 3.5205 / 4.2045 | 17.9628 / 18.1143 |

Records: [oak-browser-orbit.json](oak-browser-orbit.json) and
[spruce-browser-orbit.json](spruce-browser-orbit.json). They retain the rig's
browser version, flags, canvas, WebGPU identity and quantization note. GPU
percentiles are finer than the nominal 100 microsecond Chrome quantization;
wall intervals sit on that clock's grid. Timing records do not establish an
absence of visible shimmer; that is the owner's orbit judgment.

## What was drawn

| Geometry | Full count | Prefix-only casters | Final casters, native and browser |
|---|---:|---:|---:|
| Oak wood vertices | 4,262,170 | — | — |
| Oak wood triangles | 8,255,000 | 90,760 | **90,760** |
| Oak foliage instances | 869,310 | 869,310 | **217,328** |
| Spruce wood vertices | 2,888,144 | not measured | — |
| Spruce wood triangles | 5,580,040 | not measured | **39,920** |
| Spruce foliage instances | 7,012,326 | not measured | **1,753,082** |

Caster triangles mean wood triangles; caster instances mean retained foliage
placements. Both counts live outside FrameStats and remain on invalid timing
records. Full draw counts and the wasm slots are unchanged.

Oak native level medians are unchanged from the prefix run: 861,024 at level 0,
8,286 at level 1, and zero at levels 2–5 and unseen. Their deviations are
0.010157, 0.005079, 0.002539, 0.000317, 0.000040 and 0 metres. Spruce has
7,012,326 at level 0 and zero at levels 1–2 and unseen; its deviations are
0.000678, 0.000339 and 0 metres. The sun ignores these selected lists.

## Gates and pins

All four final gates passed on this working tree:

- `cargo test --release --workspace` — passed, including the unchanged clay,
  lit-sky, leaf-offset, leaf/bare view, identity and surface pins. Existing
  ignored tests remain ignored; no pin or tolerance changed.
- `cargo fmt --all -- --check` — passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — passed, including
  compilation of the repaired occupancy-audit example; no lint suppression.
- `npm run typecheck` — passed, including the renderer's wasm build.

Logs: `/tmp/flow-handover-fn27/cargo-test-final.log`, `fmt-final.log`,
`clippy-final.log` and `typecheck-final.log`. `git diff --check` also passed.
No rendering behavior changed after the measured captures. Prefix artifacts
were verified byte-identical to HEAD; no task/spec or index changes were made.

The new tests cover default non-empty sets on every preset, fixed counts under
orbit and row changes, full counts at zero threshold/stride one, legal empty
sets, controlled coverage comparisons, and the shared GPU kernel's block mean,
map-edge lighting and receiver normal offset. The no-anatomy gate includes
shadow.rs, wood.rs, common.wgsl and shadow.wgsl. Valid and invalid records retain
the caster counts; the total-equals-per-frame-sum identity is unchanged.

The occupancy audit now reads each run-table span and identifies its path from
the terminal cap, then checks its index count and each ring centre against that
path's nodes. This detects mismatched run membership even when the total index
count is unchanged. The old sequential-offset assumption is removed.

## Host verification

The host's earlier verification applies to the prefix-only implementation,
not to this working tree. That host ran all four gates successfully, verified
the prefix timing record against its tables, and compared a ground crop with
fn-14: silhouette, dapple and trunk shadow were judged unchanged. The earlier
“below about 0.9 ms” stop was lifted by the owner before this pass.

The host also identified the occupancy-audit permutation bug. It is repaired
here, with assertions described above: the ring-centre check against each path's
own nodes is the assertion that would have caught the silent breakage, and it is
the one to keep.

**This pass, verified by the host.** The implementer was **gpt-6-astra via
`codex exec` at high reasoning effort**, bridged from the host session in one
blocking call; the host wrote no renderer code. The host re-ran all four gates
on this working tree and each passed - `cargo test --release --workspace`,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D
warnings`, `npm run typecheck` - logged at
`/tmp/flow-handover-fn27/host-gates-2.log`. The suite was green at 3406739
before this pass, so the green is this tree's own. Every number in the tables
above was read back against `oak-final-timing.json`,
`spruce-final-timing.json`, `oak-browser-orbit.json` and
`spruce-browser-orbit.json` and matches, verdicts included: four valid sessions,
`multisample` 4 on all of them, and `caster_triangles` and `caster_instances`
present on each.

Two images were viewed by the host, no more: the paired crown crop above, and a
900 by 420 crop of the ground under the tree, fn-14's hero above this run's, at
the same crop. What the ground pair shows, as an aid and not as a verdict: the
shadow keeps its whole silhouette and extent, no hole, no ragged edge and no
stepping anywhere along it, and the trunk's own shadow is where it was. The
dapple is visibly softer - fn-14's small hard sun-flecks inside the shadow merge
into broader light - which is the kernel and the square-root-scaled quads doing
what they were built to do. Whether that softening is the dapple the owner asked
for is R2 and R5, and the host renders no verdict on it.

Two blemishes the host found and did not have fixed, neither of them load-bearing:
the browser record's adapter line reads `"; comparison filtering: Linear"`,
because WebGPU reports an empty adapter name and the suffix is appended to it -
fn-14's browser records carry the same empty string, so this is a cosmetic
artefact of the new suffix rather than a regression; and the connector boundary
the caster scale pivots around is found by taking the last vertex whose surface
coordinate is not exactly zero, so a legitimate surface vertex sitting exactly
at the coordinate origin at the end of the buffer would be left unscaled. Both
are noted for whoever next touches those files.

## Deviations

- Inherited from the completed first pass: SurfaceRun is beside SurfaceMesh,
  not paths.rs, because the example directly includes paths.rs and all-targets
  Clippy rejected an unused type there. Run-table validation is in renderer
  submit::fits, the validation entry point present in this checkout. No identity
  literal needed re-pinning: the existing pins do not hash wood buffer order;
  that reason is already documented in the identity test. These were not redone.
- The coverage comparison uses a controlled fixture of separated surfaces.
  A natural crown's enlarged sampled surfaces can fill gaps the full set leaves
  open: the initial ordinary-tree probe measured full foliage coverage
  0.10037422180175781 versus default 0.10325431823730469. Treating that
  non-monotonic union area as guaranteed would contradict the required square-root
  scale. The requested inequalities remain exact on the controlled fixture;
  real-preset default/non-empty and orbit-stability checks remain separately.
- Stride above the placement count explicitly draws zero, as R1 requires;
  ceiling division alone would draw one. Other strides use ceiling division.
- Centred scaling also needs the surface centre and connector boundary in the
  widened light uniform. They are derived from existing geometry/coordinates;
  no public field, geometry buffer or anatomy branch was added.
- Native collection and row-to-uniform filling moved into timing/collect.rs and
  scene/frame.rs to keep their parent files below the line limit. The browser
  build caught and prompted restoration of collection's native-only module guard.
- Browser evidence invoked only the existing rig's orbit function, avoiding
  unrelated conformance, still timing and soak sessions. The port-zero setup
  failure and correction to 5187 are recorded above. WebGPU reports its requested
  comparison mode because it does not expose the native filtering capability.
- The owner verdicts are in this report, not flow state, and all changes remain
  uncommitted, as explicitly directed. Four image inspections were used; the
  corrected comparison aid was checked by pixel equality without a fifth view.

## Follow-ups

- **Needle aggregation for spruce:** native total p50 is 23.4161 ms and browser
  wall p50 is 20.00 ms. Coarse casters reduce the shadow cost but do not recover
  a 60 fps spruce. No aggregation work was added here.
- fn-26's transmission must call this shared normal-offset, filtered shadow
  function and include its pixel cost in the same frame budget. Bark relief,
  veins and transmission remain fn-26's work.
- Subpixel foliage coverage remains a separate visual question at four samples;
  this pass does not claim to solve it with a shadow filter.
- The ignored legacy FN6 surface-reference export compares old buffer order;
  it remains a follow-up, not an enabled gate or a loosened pin.
- fn-14's coarse-caster, missing browser record/view declarations and stale
  headless/view strings are addressed here and linked from the README. Its
  dated report is preserved.

## Owner verdict

R2 and R5 require the owner's own words. Both slots remain empty. The spec is
not closed by passing timings or by the implementer's image inspection.

### The crown's self-shadow, R2

> _verdict (owner, ):_

### Thinning, stepping and shimmer, R5

> _verdict (owner, ):_

Aids: [paired crown crop](oak-crown-comparison.png), [lit oak](oak-final.png),
[lit spruce](spruce-final.png), [clay oak](oak-clay.png),
[fn-14 oak](../fn14/oregon-white-oak-hero.png),
[oak orbit record](oak-browser-orbit.json) and
[spruce orbit record](spruce-browser-orbit.json). For the owner's live orbit
judgment, run the harness on a non-5173 port, select either preset at seed 7,
whole view and the default scene row, then orbit at the hero pose:
`npm run dev -- --port 5187 --strictPort`.
