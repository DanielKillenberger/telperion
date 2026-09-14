# telperion

A procedural tree generator with an authored silhouette, seeded specimens and named species presets. One Rust core grows the structure; consumers choose surface meshes, instanced foliage, structural data or spatial fields.

```ts
import { TreeEngine, TELPERION } from "telperion";

const engine = await TreeEngine.create();
const family = structuredClone(TELPERION);
family.skeleton.seed = 7;
const tree = engine.build(family, { surface: true, foliage: true });
// tree.surface: Float32 positions/normals, Uint32 indices, bounds.
// tree.foliage: one leaf mesh, column-major Float32 instance matrices, bounds.
engine.release(); // returned arrays are owned copies and remain usable
engine.dispose();
```

`ORDINARY`, `TELPERION` and `LAURELIN` come from Rust preset metadata. Parameters define the family; the seed selects a specimen. `PRESETS` contains all five named templates; `TWO_TREES` contains Telperion and Laurelin. `createRenderer(canvas)` puts the Rust renderer on a canvas and draws the tree its own module generates; it is the package's only rendering path and it has no runtime dependencies. The native core uses pinned `libm` and `slotmap`; its optional `json` feature supplies the wire schema used by the Wasm binding.

For a block-based consumer, request occupancy without constructing a wood surface or transferring render buffers:

```ts
const engine = await TreeEngine.create();
const tree = engine.build(family, { field: true });
const flags = tree.field!.query(new Float64Array([
  0, 1, 0, 0.5, // cell centre x/y/z and half extent; zero means a point
]));
// Each byte: wood = 1, foliage = 2; zero is a valid empty cell.
engine.dispose();
```

Field handles expire on the next native build or release; copied query results remain owned. Field construction places retained foliage internally. `{ structure: true }` returns six f64 values per node (xyz, distal/proximal/base radius) and three u32 values (parent, branch, kind). The root parent is `0xffffffff`; kinds are structural 0, branch 1 and twig 2. An empty output selection still generates structure for diagnostics. Invalid inputs throw, and cap diagnostics distinguish incomplete growth from a finished tree.

Structure-only output uses the canonical grown wood without constructing foliage
contacts. One-shot builds release the annual history before allocating transfer
buffers; retained specimens continue to own their history for later reads.

The optional `tree.field!.snapshot()` exports an owned schema-1 CPU field snapshot
for experiments and other consumers. It contains f64 wood segments and BVH bounds,
u32 topology, revision, union bounds and extraction timings; the
[exact layout](scripts/benchmarks/generation.md#portable-snapshot-contract) defines
each array. Existing builds and queries make no snapshot copies. Extraction needs
a live field revision: release, rebuild (including a failed native rebuild) and
disposal invalidate the handle. Already copied arrays survive these operations,
and caller mutations cannot change the CPU field. Extraction errors throw without
a partial snapshot; temporary Wasm staging is released after copying or failure.
The giant snapshot alone is about 129 MB, so opt in only when needed.

## Architecture

The retained `branching::Specimen` owns scaffold and local frontiers. Nodes
carry a monotone birth order and a generational key. The annual timeline stamps
shed nodes with their death year and keeps their slots and topology unchanged;
`Specimen::node(identity)` finds survivors and rejects dead identities. Packed
reads filter dead wood and order the structural segment before local nodes.
The legacy full-envelope builder retains its existing compaction path.
The core uses pinned pure-Rust `libm` for transcendental functions. Run
`npm run wasm:build && npx vitest run harness/parity.test.ts` to compare the
five preset node buffers byte for byte across native and wasm targets.
The native `Specimen::build(&family)` path starts at a seedling and grows to
`family.age`; `advance(years)` continues its retained frontiers. Age supports
0 through 1,000,000 years, rounded to one twelve-billionth of a year at each API call (the original
billionth-of-a-month resolution). Whole years run in order and the integer
sub-year remainder carries between calls;
zero pauses, and backward inspection filters the retained chronicle at the earlier age.
The same quantized elapsed time produces the same annual history. `growth.rate`
and `growth.shape` are numeric Chapman–Richards traits and blend with age.
Production `mesh::build`, the Wasm generator and the measurement runners build
through growth at `family.age`. Each preset derives its default age from its
final growth traits with `GrowthTraits::mature_age()` (oak 432 years, spruce
158, Ordinary and the Two Trees 173); callers may override the requested age.
The oak and spruce rates and shapes fit composed reference heights at three
ages. The [FN31 report](.flow/evidence/fn31/REPORT.md) records the measured diameters,
reference deviations and owner judgment slots. Ordinary and the Two Trees keep
provisional growth traits without species age calibration. Their authored
shedding threshold is restored to 0.45; oak and spruce retain zero by choice.
A light-dependent vigour floor prevents exposed branches dying from age alone.

```rust
use telperion_core::{branching::Specimen, presets::Preset};
let mut family = Preset::OregonWhiteOak.parameters();
family.age = 10.0;
let mut tree = Specimen::build(&family)?;
let mut buffers = tree.buffers()?;
let changes = tree.advance(0.25)?;
changes.validate(&buffers, &tree.buffers()?)?; // optional reconciliation check
changes.apply(&mut buffers)?;
let skeleton = tree.tree();
let earlier = tree.read_at_age(5.0)?; // owned skeleton, envelope, placements and shed identities
# Ok::<(), telperion_core::Error>(())
```

`read()` returns an owned view at the frontier; `read_at_age(years)` filters the
chronicle at an earlier age without simulation. The view contains the packed
skeleton, envelope, placements and shed identities. Radii use the last keyframe
at that age, and copied shoot histories omit future observations. A read beyond
the frontier is refused with the frontier's age.

Each native advance returns born, resized and shed runs and born, moved and shed
leaf placements. `buffers()` reads those outputs in identity order. To check a
record, call `changes.validate(&previous_buffers, &tree.buffers()?)` before applying
it; a mismatch names the run's birth identity. Application needs no fresh read.
Run-buffer radii are exact canonical keyframe values; the family's resize
tolerance controls frame creation without a separate consumer rounding grid.
Placement matrices reconcile bit for bit, including movement caused
by an adjacent branch changing a surface-contact polygon. A clock-only advance
returns newly reached cohorts, deriving transforms only for their shoots.
Once those cohorts are full, clock-only advances derive no placements.

Negative/non-finite advances and invalid ages name the field and value. A node
cap rolls back the failed year, retains completed years, sets `node_capped`,
and refuses the next advance. `set_node_ceiling` can raise the resource limit
and resume the same frontier. The curve's final work quantum defines saturation;
advancing beyond it jumps directly to the requested age.

Annual scaffold stations release their lateral buds while the parent axis is
still extending. Boundary pauses leave the growth budget for eligible shoots;
local terminal and lateral buds retain separate allocation state. Paused structural
axes keep their terminal buds until extension finishes. Seedling height is
introduced in year one; shoot steps and twig lengths are bounded by live height.
Juvenile lateral recruitment fills the small crown. Local planning transitions
from current room to authored room between one and two juvenile heights.
Permanent axes retain the crown base according to `growth.crownBaseRetention`.
The FN31 report compares mature populations and bounds with the previous build.

Annual shoots retain birth/death years, terminal/lateral fate and year-stamped
crown-depth vigour observations. `shoot.vigour()` reads the latest observation;
tolerance counters are retained in the same append-only event sequence.
Attractor consumption likewise retains its first consumption year.
The existing habit `sheddingThreshold` is the annual vigour threshold;
`growth.sheddingTolerance` counts consecutive active years below it, in years.
Equality resets the clock. Vigour is exposure multiplied by
`max(1 / (1 + rate * nodeAge), vigourFloor)`, with a default floor of 0.75.
Shade still reduces exposure below the threshold. A lit descendant
supports its ancestors. Each slice snapshots decisions before growth, sheds at
most 32 subtrees in birth order, and protects the main structural leader.
`growth.apicalControlLoss` weakens terminal control with age and releases lateral
allocation. Its default is zero; the tolerance defaults to two years. These are
uncalibrated numeric traits. Oak and spruce still have threshold zero.

Trunk scale multiplies live height by an age-dependent fraction from
`juvenileRadius` to one. `thickeningDelay` and `thickeningShape` control that
secondary thickening over the derived lifetime; the pipe-model fork split is
unchanged. Surviving structural and local radii never decrease. Local allocations and taper
are re-derived from current parents without changing twig lengths. Dead shoots
leave the growth frontiers while their records remain. A cut invalidates only
surviving pipe ancestor paths; local widths propagate from changed parents.
Dead records keep canonical final widths independent of advance partitions.
The annual solve records radius keyframes only along changed paths. A frame is
appended when any radius exceeds the last frame by more than
`growth.resizeTolerance` (metres, range 0–1, default `1e-9`); births always get a
frame. Radii never decrease. One ten-year advance retains the same frames as ten
yearly advances. Output radii still materialize once per advance, from the latest
frames, and packing remains lazy.

`branching::generate` and `Specimen::grow` retain the legacy full-envelope API.
Production mesh builds and the browser use the annual growth path. The JSON wire
round-trips and validates age and all numeric growth traits, including
`seedlingHeight`, `shootStep`, `juvenileBranching`, `juvenileHeight`,
`seedlingRadius`, `juvenileRadius`, `thickeningDelay`, `thickeningShape`,
`crownBaseRetention` and `vigourFloor`; the blend walks every row.
`Specimen::placements()` returns owned leaf transforms, each identified by its
shoot's generational identity and station ordinal, before optional canopy shell
culling. `growth.leafLifetime` is a numeric family trait: one year by default and
for oak, provisionally six for spruce; zero bears no leaves. Stations are spread
across `ceil(leafLifetime)` annual cohort offsets, beginning at birth. A one-year
lifetime fills immediately; a longer lifetime fills over its first years and
then holds the same station identities while the shoot lives. Wood above the
twig anatomy's bearing diameter carries no foliage. Station randomness is keyed
by shoot identity. Unchanged wood reuses its cached transforms; changes to radii
or neighboring contact polygons re-derive only the affected shoots. Production uses this timeline
foliage path. `seedlingRadius` also admits foliage on slender seedling stems,
bounded by the same anatomy bearing diameter; shoots stop bearing as they thicken
beyond the threshold.
Visual judgment remains the owner’s.
`Specimen::changes_between(from, to)` filters birth/death years, radius frames
and cohort offsets in either direction. Growing advances use the same filter.
Records carry exact keyframe radii and selected station transforms, including
motion caused by neighboring contact paths; no whole-buffer diff is computed.
`ChangeRecord::apply` updates identity-keyed consumer buffers atomically.
`build_with_history_cap(family, years)` and `set_history_cap(years)` set retention;
the default is 10,000 years. Reads older than the retained window refuse with the
cap and earliest available age. Increasing the cap cannot restore discarded data.
Compaction drops old dead geometry, shoot histories, radius frames and placements,
retaining a compact death index for the cumulative shed set and reserving identity
slots. It preserves frontier bytes, later growth and node-ceiling behavior.
`TreeEngine.buildSpecimen(family, historyCap?)` returns a retained handle.
`read()` defaults to its frontier; `read(age)` and `changes(from, to)` inspect
retained ages. `advance(years)` returns the new frontier and its change record.
Reads, records and snapshots are owned JavaScript copies. Successful specimen
rebuild/import, handle release, engine release and disposal invalidate the old
handle. Failed rebuilds/imports preserve it. `setNodeCeiling(limit)` resumes a
capped specimen, and `setHistoryCap(years)` changes retention. Native consumers
have the same operations through `specimen::SpecimenStore` (snapshot operations
require the `json` feature), or use `branching::Specimen` directly.

```ts
const specimen = engine.buildSpecimen({ ...OREGON_WHITE_OAK, age: 10 }, 100);
const before = specimen.read();
const { changes } = specimen.advance(0.25);
const earlier = specimen.read(5);
const snapshot = specimen.snapshot(); // optional, never a mesh
const restored = engine.importSpecimen(snapshot); // invalidates specimen
restored.advance(1);
```

The schema-1 specimen snapshot carries the chronicle, retained frontiers,
integer clock, cap/floor, identity slots and writer state. It omits packed reads,
foliage contact caches, crown caches and meshes. The owned `Uint8Array` is the
same byte format as native `Specimen::snapshot()` / `from_snapshot(bytes)`:
`TLPS`, a little-endian u32 schema (1), fixed-integer little-endian bincode 1.3.3
state in the declared `Specimen` field order, then an eight-byte FNV-1a checksum
of the preceding bytes. Lengths and native indices encode as u64; unlimited
canopy counts encode as UINT64_MAX, compacted identity indices as UINT32_MAX.
The adapters restore native sentinels on import. Invalid size, schema, checksum
or payload refuses before replacement. The current staging limit is 512 MiB.
`harness/parity.test.ts` exchanges snapshots in both directions between native
and wasm, advances them, and compares earlier and frontier node/placement bytes
for every preset.

The harness's age number and slider inspect the one retained specimen; beyond
its frontier they advance it. Rebuild starts a new specimen at the chosen age.
Play uses the page's years-per-second setting and carries fractional years.
`SpecimenView` applies interval records to its identity buffers, sweeps wood
again when a year changes and submits the updated placement transforms. Camera
framing remains explicit. The ordinary production `mesh::build` and
`branching::generate` routes retain the envelope build until calibration.
Its pipe cache recomputes insertion/deletion ancestor paths. An ordered scale
index visits structural wood only when its historical width can be exceeded;
local width changes propagate to descendants in birth order. Crown exposure uses
an indexed profile and caches samples until that envelope or position changes.
With shedding disabled, only frontier shoots sample vigour; other nodes retain
their last sampled state. With shedding enabled, the slice-start survival pass
refreshes the live crown and propagates descendant support.
Structural births append without moving local storage inside a slice. Internal
frontiers and pipe reductions use node kinds. Consumer reads lazily pack a
structural-first view without moving the retained frontiers' storage.
Chronicle slots are never reused, so retention boundaries cannot change
future handles. Local seeding retains unallocated stations and structural child
counts.
Full-tree validation remains available to callers; annual mutations validate
new or resized nodes. Native cost measurements, including sparse and dense
changes on large trees, run with
`FN11_MEASURE=1 cargo test --release -p telperion-core --lib monthly_cost_report -- --nocapture`.
The command retains its historical name; it now measures annual slices. Widths
finalize once per advance from the annual radius keyframes;
consumer packing is lazy and timed separately. Fixed-geometry shoots sleep until
the crown can reach them. An unchanged queue keeps its identity order and an
empty local frontier makes no width queries. Radius-dependent failures still retry.
R10 selected annual slices after the mature monthly oak measured 5.906 seconds
(native three-build median), above the approximately half-second target. The
change-record timings are included in that command. For the mature oak's
optional snapshot export/import and fresh-build equivalence checks, run
`FN11_SNAPSHOT=1 cargo test --release -p telperion-core --lib monthly_cost_report -- --nocapture`.
The final cost report and calibration remain later work.
The early annual medians were 779 ms oak and 561 ms spruce. The annual oak also
misses the target; the closed-form design remains the owner's reserve. See the
[measurement and convergence figures](scripts/benchmarks/generation.md#annual-slice-choice-fn-11-native-2026-09-13).

The native entry is `branching::generate(&family.skeleton, family.radii)`. Its solved `Tree` can feed `surface::build`, foliage placement/culling, or `Field::new` independently. The Wasm binding assembles the requested stages; `src/browser` loads it and copies output arrays. There is no TypeScript generator and no TypeScript renderer.

| Owner in `crates/telperion-core/src` | Responsibility |
|---|---|
| `envelope`, `colonization`, `bias` | Authored crown, attractor points, directional fields and turn constraints |
| `branching/traits`, `branching/scaffold` | The numeric habit traits and the one builder that grows every axis from them |
| `branching/local`, `twigs`, `radius` | Radius-driven branch generations, fixed twig anatomy, local taper and fork conservation |
| `surface` | Continuous swept wood, transported frames, lobes, twist and sockets |
| `foliage` | Leaf outline and lean from numeric traits, anatomical stations, phyllotaxis and shell retention |
| `field` | Wood and foliage cell occupancy, independently of render meshes |
| `presets` | Named parameter sets and scale choices |

`crates/telperion-render` draws that output on wgpu, and is the only renderer in the repository. It compiles to two targets from one code path: a wasm module the page loads through `src/browser/render.ts`, which generates and uploads inside its own linear memory, and a native offscreen target whose `headless` example writes a PNG at the hero pose and, on request, a GPU timing record. The renderer owns the whole scene - ground, sky, one sun and the single depth map it casts its shadow through, the 1.8 m scale figure, camera and the whole, bare, single-leaf and clay views - and it never names a species: only the headless entry point resolves a preset id. Appearance is numbers, not textures: a material row on the family carries the bark and leaf colours, the bark's roughness, the hue and brightness ranges each leaf varies inside and how far the crown's interior is darkened, while a separate scene row carries the sun, the sky, the ground and the shadow coarsening and filtering controls; both reach the shaders as uniforms and no shader path branches on a species. Colour and depth are drawn at four samples a pixel where the adapter offers them and resolved into the single-sample target, and one filmic tone map closes every lit frame. The clay view draws the neutral room this renderer started as - no material, no sun, no tone map - so geometry can still be judged with nothing over it. The crown is not one draw: a compute pass reads every placement once per frame, projects the leaf element's extent to a pixel size and gives that leaf the coarsest level whose outline deviation projects under half a pixel, or the unseen bucket if it is outside the frustum; the renderer then issues one indexed indirect draw per level, over the instances that chose it. A level is a subset of the element's own sections, so a coarse leaf's vertices are a fine leaf's and no placement, count or bound changes with the level drawn.

The sun writes one 1,024-square depth map. Wood casters are a prefix of whole runs ordered by largest radius; foliage casters use a fixed placement stride at the coarsest level, scaling each retained surface by the square root of the stride about its centre. The connector stays fixed. The shared read averages a square of comparisons after offsetting the receiver along its normal; taps outside the map are lit. The caster sets do not depend on camera selection. Reordering placements changes the sampled subset, and changing run radii retunes the threshold; future vertex motion must also move the casters.

The crown envelope controls the silhouette. One scaffold builder grows every axis inside it from a table of numeric habit traits - apical dominance, whorl strength, station spacing, pitch, rise, crookedness and the rest - and every growth unit of every axis takes its heading from one sum of the rule heading, the pull of the envelope's attractors and the bias field. Attractor pull is a trait weight in that sum rather than a phase of its own, and no species has a builder or a field the others lack: an oak and a spruce are two rows of the same table, and any point between them is a tree. The leaf is a row too - lobe count, lobe depth and section roundness draw the outline, forward lean, lean rise and surface contact place it on its shoot - so there is no anatomy to switch between. Local branch laws continue from the scaffold down to leaf-bearing twigs and read the same traits. Forks conserve cross-sectional area with a tunable exponent. Branch resolution and lateral count are independent. Terminal twig anatomy is measured in metres; the giant presets retain fine twigs instead of uniformly enlarging them.

Botanical and output changes have separate owners. A representative development exercise and the retained test mapping are in [the migration guide](tests/migration/README.md#changing-a-rule-or-an-output). Native mesh-free examples are exercised in `crates/telperion-core/tests/field.rs`; browser binding ownership and failure recovery are checked in `tests/browser/bindings.mjs`, and the page on a real GPU in `tests/browser/render.mjs`.

## Build and develop

Install Rust through rustup, then:

```sh
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy --target wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.128   # the version Cargo.lock pins
npm ci
npm run rust:build
npm run wasm:build
npm run render:build
npm run rust:test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run typecheck
npm run build
npx playwright install chromium
npm run rust:test:wasm
npm run test:render
npm run dev
```

The repository pins Rust in `rust-toolchain.toml`, and `render:build` checks the installed `wasm-bindgen` against the version the crate pins before it generates the glue - a mismatch fails with the command to run. `dev` and `build` regenerate both Wasm modules and the preset metadata. Build them before running the Node harness tests from a clean checkout. The package embeds both binaries; consumers do not need Rust. The development viewer exposes every generator parameter, the material and scene rows through the same numeric controls, the five presets and the whole, bare, single-leaf and clay views, with a GPU timing session on the button beside them.

A still without a browser, from the same renderer:

```sh
cargo run --release -p telperion-render --example headless -- \
  --preset norway-spruce --seed 7 --view whole --out /tmp/spruce.png \
  [--timing /tmp/spruce.json] [--orbit] [--level <n>] [--scene <json>]
```

`--view` takes `whole`, `bare`, `leaf` or `clay`; the first three are lit and the last is the neutral room, which is how geometry is inspected with no material, sun or tone map over it. `--scene <json>` replaces the default scene row for the frame - the sun's azimuth, elevation and colour, the sky's zenith and horizon and the ground, plus the shadow controls - as the row's own JSON under the field names the panel shows (`{"sunElevation": 24, "skyHorizonBlue": 1.1}`, every field optional); an unknown field is refused with the list of the real ones and a value outside its range with the range it wanted. The shadow defaults are `casterTexels: 1` (range 0–8), `casterStride: 4` (1–64), `shadowFilterTexels: 1` (0–3) and `shadowNormalOffset: 1` (0–4). Threshold and offset use the fitted world size of a map texel; stride and kernel radius truncate to whole counts. Threshold zero keeps every wood run, stride one keeps every placement, and filter radius zero keeps a single hardware comparison. A threshold above every run radius or a stride above the placement count legally empties that caster set. The material row is the family's, so it arrives with the preset. `--level <n>` holds every leaf of the frame at one level instead of letting selection choose, which is how a single level's cost is measured on its own. `--orbit` turns the camera one full revolution about the subject at the hero pose's elevation and distance while the timing session runs; the still beside it is always the hero pose, because the orbit is what is measured and not what is judged.

The same target walks between two presets:

```sh
cargo run --release -p telperion-render --example headless -- \
  --preset oregon-white-oak --to norway-spruce --seed 7 --frames 240 \
  --out /tmp/walk/frame.png
```

The material row also controls procedural surface detail: `ridgeScale` and
`plateScale` (0–1 metres), `furrowStrength` and `roughnessDetail` (0–1),
`veinScale` (0–32 pairs per blade), `veinContrast` (0–1),
`transmissionStrength` (0–1), linear `transmissionRed/Green/Blue` (each 0–1),
and `thickness` (0–8 optical depth). Transmission attenuates by
`exp(-thickness)` and the existing shadow comparison. Round sections suppress
vein and margin tone. Ridge scale sets circumferential ridge spacing; plate
scale sets staggered scale height, bounded to 1.5–2 ridge widths so long
furrows still carry short scales.
Larger plate-to-ridge ratios deepen and lengthen the shouldered furrows.
Furrow strength independently narrows and shallows those gaps; zero keeps
the plates and flakes with narrow outlines. It defaults to one for older rows.
Flat faces have lifted lower edges and finer flakes derived from those same
two lengths. Zero ridge scale disables relief; zero plate scale leaves ridges
without cross-fissures. The young-wood fade spans diameters of two to five
ridge widths; relief continues strengthening with girth on mature runs.
Both axial distance and circumferential arc length supply pixel footprints.
Noise, plates and flakes fade to their means before becoming unresolved;
edge support expands with the footprint to suppress sharp normal harmonics.
The shading normal differentiates this filtered height at the fragment,
holding the footprint fixed, so changing the filter does not create relief.
Four subpixel shading evaluations integrate the normal's nonlinear lighting;
the geometry coverage and existing shadow lookup are unchanged.
These fields affect shading only.

Foliage selection compacts each level in placement-index order. Equal-depth
leaf samples therefore resolve consistently when the same frame is redrawn.

`--to <preset>` renders a numbered PNG sequence instead of one still: `--frames <n>` frames, 240 by default, each the blend of the two families at the one seed, all of them at the hero pose the first frame's bounds fixed. `--out` names the sequence, so `--out /tmp/walk/frame.png` writes `/tmp/walk/frame-0001.png` onward with `transition.json` beside them, naming both presets, the seed, the size, the frame count, the rate of 24 a second and what the encoder did. When `ffmpeg` is on the path the frames are assembled into `transition.mp4` at that rate; when it is not, the run says so in one line and keeps the sequence, which is the artefact either way.

The same walk stated in seconds, eased, with the camera between the two trees:

```sh
cargo run --release -p telperion-render --example headless -- \
  --preset oregon-white-oak --to norway-spruce --seed 7 \
  --walk 12 --hold 2 --sweep 35 --size 1920x1080 --out .flow/evidence/fn25/whole/frame.png
```

`--walk <seconds>` gives the blend a length in seconds at 24 frames a second instead of a frame count, and eases it with a smoothstep so the walk leaves and arrives at rest. `--hold <seconds>` holds that many seconds of frames at each end, at the near family before the walk and the far one after it. `--sweep <degrees>` turns the camera that many degrees of azimuth across the whole sequence, holds included, so the shot keeps drifting while the tree stands still. A walk also changes what the camera is: instead of the first frame's pose held throughout, it eases between the two ends' own hero poses - what it looks at, how far back it stands and how high - so neither tree is framed for the other. `--view leaf` walks the same way on the leaf of each end. `--walk` and `--frames` are two ways to say one length and refuse each other; a walk of zero seconds, a negative hold, or a sweep outside -360 to 360 degrees is refused by the name of the flag. `transition.json` carries the walk, the hold, the sweep and the ease beside the frames.

```sh
node scripts/scenic-cut.mjs --out .flow/evidence/fn25/scenic.mp4
```

`scripts/scenic-cut.mjs` cuts a whole-tree sequence and a leaf sequence, both already on disk, into the one clip: a half-second crossfade from the first into the second, a light grade of contrast, warmth and a soft vignette, and one ffmpeg encode to 1920 by 1080 H.264 at 24 frames a second. It writes `scenic.json` beside the clip naming every input, the grade and the exact invocation, and it re-reads the finished file rather than assuming it. It renders nothing itself, refuses a sequence it cannot find by naming the path, and on a machine with no `ffmpeg` says so in one line and leaves the sequences standing.

`npm run rust:test:wasm` holds the Wasm binding to its contract in a plain headless browser, which needs no adapter at all. `npm run test:render` drives the page on hardware WebGPU: it needs a display, and skips with the renderer's own words when the machine offers no hardware adapter. `npm run species:qa` renders the species stills through the headless target.

## Measurements and limits

Rendering is measured by the renderer's own GPU timing session - conditioning frames, warmup and measured samples, with a verdict that is `valid`, `unavailable`, `disjoint` or `contended`, and no millisecond figure at all unless the verdict is valid. A record carries `p50_ms` and `p95_ms` for the vegetation pass, `selection_p50_ms` and `selection_p95_ms` for the compute pass that chose the levels, `shadow_p50_ms` and `shadow_p95_ms` for the depth pass the sun writes its map in, `total_p50_ms` and `total_p95_ms` for the three added frame by frame and then ranked, `multisample`, the samples a pixel the frame was actually drawn at, `caster_triangles` and `caster_instances`, the wood triangles and foliage placements submitted to the sun, and `levels`, one entry per level with its tolerance in metres and the median instance count it drew, the last being the leaves no level drew. An orbit session adds `wall_p50_ms`, `wall_p95_ms`, `wall_max_ms` and `wall_frames`, the frame-to-frame wall clock of the loop that drew them. An invalid GPU record omits GPU percentiles and level medians; multisample and caster counts remain, and a valid wall series is reported independently. The adapter line identifies the comparison sampling mode (WebGPU reports the requested mode; native adapters are checked for linear filtering). The [FN22 report](.flow/evidence/fn22/REPORT.md) records the native and browser sessions for oak and spruce before level selection, the [FN23 report](.flow/evidence/fn23/REPORT.md) the same sessions after it, and the [FN14 report](.flow/evidence/fn14/REPORT.md) the same sessions again with the sun, its shadow map and four samples a pixel in the frame, each beside the stills they were measured on. The [FN27 report](.flow/evidence/fn27/REPORT.md) adds the coarse casters and filtered comparison: it closes fn-14's coarse-caster, browser-type and stale-view-documentation follow-ups, returns the native oak under 3.8 ms, and records the spruce with needle aggregation as its next performance step. Its visual verdicts remain with the owner. That is the rendering path, and it is unrelated to the rejected GPU query backend below, which was about generation and occupancy rather than drawing.

The [FN8 report](.flow/evidence/fn8/REPORT.md) owns the matched full-build measurements, binding costs, native observations, memory-domain limits and GPU results. CPU generation latency and GPU frame time are separate measurements. Wasm linear-memory capacity is a high-water allocation, not live heap or total browser memory; release allows allocator reuse and dispose allows host reclamation once references are gone. Scene replacement retains the previous tree until the new build succeeds, so transient coexistence matters.

The [FN12 generation report](.flow/evidence/fn12/REPORT.md) records a **rejected
production GPU query candidate**. All cold workloads were slower and f32 contact
results differed from the f64 CPU reference. A giant 64³ resident query improved
locally, but did not qualify a complete lifecycle or the general precision contract.
Skeleton and field-construction GPU performance remain inconclusive; only their
CPU stages and code dependencies were examined. Production generation and queries
remain synchronous CPU operations, with no automatic backend selection or new GPU
entry point. The rejected WebGPU implementation, runner and dedicated tests have
been removed. The report retains final timing and correctness evidence from Linux
Chromium 151 and an RTX 3080. The snapshot API and
[CPU reproduction tools](scripts/benchmarks/generation.md) remain in use by the
field-generation follow-up's correctness checks and measurements.

The archived [FN7 surface experiment](experiments/rust-surface-benchmark/REPORT.md) measured a narrower and older workload. Its numbers are historical, not a full-engine migration result. Further botanical realism and species visual QA remain future work. Bark and foliage are now judged lit - colour, sun, shadow and the crown's own depth, beside the reference photographs - and the clay view remains for judging geometry alone; procedural bark relief, leaf veins and two-sided leaf transmission are implemented, with owner judgments at the close-up scales still pending. Full lifecycle simulation is not implemented.

## License

MIT

Historical measurement payloads and the frozen FN7 implementation live in Git history. The reports link to their pinned archive. The remaining generation benchmark runners live in `scripts/benchmarks/` and write results outside the repository by default; the browser rendering runners retired with the Three.js stage. Retrieve the old evidence without changing this checkout:

```sh
mkdir -p /tmp/telperion-history
git archive 1922505a8a396d73b335974eabf6a9faf33ccd62 .flow/evidence experiments/rust-surface-benchmark | tar -x -C /tmp/telperion-history
```

## Species and reproducible specimens

The viewer's species selector exposes Oregon white oak (`oregon-white-oak`,
*Quercus garryana*) and Norway spruce (`norway-spruce`, *Picea abies*), alongside
Ordinary, Telperion and Laurelin. Choose species independently of the unsigned
32-bit specimen seed; changing species preserves the seed. Identical family
parameters and seed reproduce the specimen. Different seeds vary structure and
placement, not species identity. Whole, bare-branch and single-leaf views
support inspection; the leaf view isolates one placed unit at generated scale.
Open `/?species=norway-spruce&seed=1` to load a full-foliage specimen directly.

Browser consumers can use `presetById('oregon-white-oak')`, set
`family.skeleton.seed`, then pass the family to `TreeEngine.build`. Native
consumers use `Preset::OregonWhiteOak.parameters()` or
`Preset::NorwaySpruce.parameters()`. Unknown identities are rejected. Natural
presets disable the separate supernatural group; Telperion and Laurelin enable
it explicitly. Botanical lean and gravitropism remain independent.

Native needle placement can use `foliage::place_on_surface` with the family’s
`SurfaceParams` to attach to the rendered polygonal sweep and fork sockets without
building mesh indices or normals. The browser engine and species measurement
runner read the growth chronicle's surface-attached placements and living cohorts. `foliage::place` retains the circular-radius placement API.

The [frozen botanical profiles](.flow/evidence/fn9/profiles.json) define mature
open-grown contexts, source-backed dimensional gates, contextual estimates and
unknown quantities. [References](.flow/evidence/fn9/REFERENCES.md) attribute the
photographs and research; [cross-seed QA](.flow/evidence/fn9/REPORT.md) records
remaining fidelity failures. A numeric pass alone is not botanical approval.

CPU-only measurement needs Rust, not a GPU. The stills additionally need a
hardware adapter, which the renderer requires and names when it is missing; no
browser, page or Playwright is in that path any more. Run
`npm run species:qa -- --output /tmp/species-run` for the whole protocol, or
`npm run species:measure -- --output /tmp/species-run` for the numbers alone. See
the [migration guide](tests/migration/README.md#species-evidence-and-replay) for
the full replay protocol and failure semantics.

The [comparative botanical benchmark](.flow/evidence/fn19/REPORT.md) freezes twelve
mature oak/spruce specimens, structural distributions and matched anatomy views.
Its [record](tests/migration/README.md#comparative-botanical-benchmark-fn19)
separates collection failures, engineering inspection and pending independent
botanical assessment; the rig itself measured the Three.js renderer and is
retired with that renderer. To expand the catalogue, use the
[species onboarding workflow](docs/species-onboarding.md),
[dispatch template](templates/species-profile.md) and
[independent example packets](.flow/evidence/fn19/onboarding-examples/README.md).
