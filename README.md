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

`ORDINARY`, `TELPERION` and `LAURELIN` come from Rust preset metadata. Parameters define the family; the seed selects a specimen. `PRESETS` contains all five named templates; `TWO_TREES` contains Telperion and Laurelin. `createRenderer(canvas)` puts the Rust renderer on a canvas and draws the tree its own module generates; it is the package's only rendering path and it has no runtime dependencies. The native core has no external Rust dependencies; `serde_json` belongs to the Wasm binding only.

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

`crates/telperion-render` draws that output on wgpu, and is the only renderer in the repository. It compiles to two targets from one code path: a wasm module the page loads through `src/browser/render.ts`, which generates and uploads inside its own linear memory, and a native offscreen target whose `headless` example writes a PNG at the hero pose and, on request, a GPU timing record. The renderer owns the whole scene - ground, clay hemisphere lighting, the 1.8 m scale figure, camera and the whole, bare and single-leaf views - and it never names a species: only the headless entry point resolves a preset id. The crown is not one draw: a compute pass reads every placement once per frame, projects the leaf element's extent to a pixel size and gives that leaf the coarsest level whose outline deviation projects under half a pixel, or the unseen bucket if it is outside the frustum; the renderer then issues one indexed indirect draw per level, over the instances that chose it. A level is a subset of the element's own sections, so a coarse leaf's vertices are a fine leaf's and no placement, count or bound changes with the level drawn.

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

The repository pins Rust in `rust-toolchain.toml`, and `render:build` checks the installed `wasm-bindgen` against the version the crate pins before it generates the glue - a mismatch fails with the command to run. `dev` and `build` regenerate both Wasm modules and the preset metadata. Build them before running the Node harness tests from a clean checkout. The package embeds both binaries; consumers do not need Rust. The development viewer exposes every generator parameter, the five presets and the whole, bare and single-leaf views in neutral clay, with a GPU timing session on the button beside them.

A still without a browser, from the same renderer:

```sh
cargo run --release -p telperion-render --example headless -- \
  --preset norway-spruce --seed 7 --view whole --out /tmp/spruce.png \
  [--timing /tmp/spruce.json] [--orbit] [--level <n>]
```

`--level <n>` holds every leaf of the frame at one level instead of letting selection choose, which is how a single level's cost is measured on its own. `--orbit` turns the camera one full revolution about the subject at the hero pose's elevation and distance while the timing session runs; the still beside it is always the hero pose, because the orbit is what is measured and not what is judged.

The same target walks between two presets:

```sh
cargo run --release -p telperion-render --example headless -- \
  --preset oregon-white-oak --to norway-spruce --seed 7 --frames 240 \
  --out /tmp/walk/frame.png
```

`--to <preset>` renders a numbered PNG sequence instead of one still: `--frames <n>` frames, 240 by default, each the blend of the two families at the one seed, all of them at the hero pose the first frame's bounds fixed. `--out` names the sequence, so `--out /tmp/walk/frame.png` writes `/tmp/walk/frame-0001.png` onward with `transition.json` beside them, naming both presets, the seed, the size, the frame count, the rate of 24 a second and what the encoder did. When `ffmpeg` is on the path the frames are assembled into `transition.mp4` at that rate; when it is not, the run says so in one line and keeps the sequence, which is the artefact either way.

`npm run rust:test:wasm` holds the Wasm binding to its contract in a plain headless browser, which needs no adapter at all. `npm run test:render` drives the page on hardware WebGPU: it needs a display, and skips with the renderer's own words when the machine offers no hardware adapter. `npm run species:qa` renders the species stills through the headless target.

## Measurements and limits

Rendering is measured by the renderer's own GPU timing session - conditioning frames, warmup and measured samples, with a verdict that is `valid`, `unavailable`, `disjoint` or `contended`, and no millisecond figure at all unless the verdict is valid. A record carries `p50_ms` and `p95_ms` for the vegetation pass, `selection_p50_ms` and `selection_p95_ms` for the compute pass that chose the levels, `total_p50_ms` and `total_p95_ms` for the two added frame by frame and then ranked, and `levels`, one entry per level with its tolerance in metres and the median instance count it drew, the last being the leaves no level drew. An orbit session adds `wall_p50_ms`, `wall_p95_ms`, `wall_max_ms` and `wall_frames`, the frame-to-frame wall clock of the loop that drew them. A record whose verdict is not valid carries none of these. The [FN22 report](.flow/evidence/fn22/REPORT.md) records the native and browser sessions for oak and spruce before level selection, and the [FN23 report](.flow/evidence/fn23/REPORT.md) the same sessions after it, each beside the stills they were measured on. That is the rendering path, and it is unrelated to the rejected GPU query backend below, which was about generation and occupancy rather than drawing.

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

The archived [FN7 surface experiment](experiments/rust-surface-benchmark/REPORT.md) measured a narrower and older workload. Its numbers are historical, not a full-engine migration result. Further botanical realism and species visual QA remain future work; leaf appearance is currently judged as geometry in clay. Full lifecycle simulation is not implemented.

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
runner use this path. `foliage::place` retains the circular-radius placement API.

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
