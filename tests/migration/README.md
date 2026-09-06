# Rust migration foundation

Install Rust with rustup using its standard installer, then from the repository root:

```sh
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy --target wasm32-unknown-unknown
npm ci
npm run rust:build
npm run rust:test
```

The toolchain file pins native and Wasm builds. No build script depends on `/tmp` or a machine-specific toolchain installation. The native core is dependency-free; `serde_json` is confined to the Wasm binding. `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings` check style and compiler diagnostics.

Run `npm run wasm:build` before `npm test`, then `npm run typecheck` and `npm run build`. Playwright is a dev dependency: install Chromium with `npx playwright install chromium`, then run `npm run rust:test:wasm`. The runner tests the actual full binding in headless Chromium, including native parity, independent outputs, empty requests, invalid inputs, owned copies, stale fields and release/dispose. `PLAYWRIGHT_MODULE` and `CHROMIUM_EXECUTABLE` are optional environment overrides.

The public API is documented in [README](../../README.md). Request parsing uses bounded owned allocation rather than accepting caller pointers. Native build failures clear stale output. Returned JS arrays are owned; field handles are valid only until the next native build or release. Wasm capacity persists after release, while disposing the engine makes the instance eligible for host garbage collection. These are distinct from live heap and process memory.

## Reference

```sh
npm run migration:reference
npm run migration:reference -- ordinary capped
REFERENCE_BUFFERS=1 REFERENCE_OUTPUT=/path/to/output npm run migration:reference -- telperion
```

The exporter requires final FN6 commit `fdafb099b1495519de75a6b9a66d37f7d07e47bd` in the local Git object database. Missing or mismatched provenance fails visibly. It extracts that exact Git archive into a disposable directory, bundles only the reference runner there, executes cases sequentially and removes the directory. Production never imports the archived generator. Use `npm ci` first; the archive borrows the installed dependencies. Generated data lives in ignored `tests/migration/generated/` by default.

Cases cover ordinary seed 42, authored Telperion and Laurelin presets, zero attractors, zero width, high crown base (envelope crossing), and a 20-node cap. Their raw input parameter manifests, fixed camera definitions, attractor/node samples, structural diagnostics, stage sizes and SHA-256 hashes are exported. `REFERENCE_BUFFERS=1` also emits exact typed-array bytes for nodes/parents/branch allocations/radii/surface/foliage. The JSON states value counts; `.bin` types follow the corresponding TS arrays (positions/radii Float64, surface/foliage Float32, parents/branch IDs Int32, indices Uint32, twig Uint8). Buffers use host endianness; the captured host is little-endian. Giant buffers are reproducible, not committed. `fixtures/` holds compact final-reference records; timings are observations, not performance gates.

## Comparison policy selected before botanical ports

The old generator is a diagnostic reference, not an exact topology or byte-compatibility requirement. `node scripts/compare-migration.mjs REFERENCE_DIRECTORY CANDIDATE_DIRECTORY` reports node/vertex/foliage count ratios, bounds drift, cap state and stage hash changes. Candidate records follow the exported JSON schema. A missing case, mismatched reference revision, or false candidate invariant fails. A changed topology or hash alone does not fail. Record and investigate drift, then compare the ordinary and giant images with the same camera, neutral material, viewport and foliage visibility. Matched ordinary and giant numerical/clay comparisons are recorded with the browser cutover evidence; the final [FN8 report](../../.flow/evidence/fn8/REPORT.md) owns the measurement conclusions.

Required invariants: finite positions/transforms; parent-before-child tree order; positive solved radii; proximal edge radius at least distal; surface indices in range; explicit cap diagnostics; valid empty regions. Fork conservation above crossover, branch-local taper, twig anatomy and shell containment belong to the stage ports. Boundary fixture `empty` records old FN6's one root node and empty representations; it does not require Rust to retain an artificial root for an empty tree.

For matched numeric primitives/native-Wasm comparisons, use absolute position tolerance `1e-10 * max(1,height)`, radius tolerance `1e-9 * max(1,radius)`, float32 output tolerance `8 * 2^-23 * max(1,height)`, and unit-frame component tolerance `1e-5`. Field signed-distance comparisons use the position tolerance, except occupancy exactly at a boundary where distance and cell overlap must be reported. Reference-vs-rewrite topology drift invalidates elementwise comparisons; compare bounds, quantiles, count ratios, invariants and images instead. Exact RNG u32 streams remain portable; hash matches diagnose determinism, not old architecture compatibility.

## Full browser comparison

Generate the pinned fixture buffers and Three reference normals, then compare the running Rust harness (all launches default to headless):

```sh
REFERENCE_OUTPUT=/tmp/telperion-surface-reference node crates/telperion-core/tests/surface_reference.mjs ordinary telperion laurelin
npm run dev -- --host 127.0.0.1 --port 5185
# In another terminal, after the server is ready:
REFERENCE_DIR=/tmp/telperion-surface-reference BROWSER_URL=http://127.0.0.1:5185 node tests/browser/migration.mjs
```

The exporter runs its native comparison too. `BROWSER_EVIDENCE` selects capture/report output; `CHROMIUM_EXECUTABLE` selects an installed Chromium when Playwright's bundled browser is unavailable. The browser runner streams fixtures and application modules through one origin. Restart the development server after rebuilding Wasm before starting a measurement run, to avoid mixing Vite HMR module instances.

## Retained test ownership

The deleted TypeScript suites remain in Git at the pre-cleanup revision. This map covers their behavioral responsibilities, not an equivalence claim based on test counts. Native owner paths below are relative to `crates/telperion-core/tests/`, except `foundation`, which is this directory's `foundation.rs`.

| Removed `src/` suite | Retained owner and behavior |
|---|---|
| `rng.test.ts` | `foundation`: known seeded stream, determinism and range |
| `noise.test.ts` | `foundation`: distinct seeds, bounded fBm, local smoothness and lattice continuity, divergence-free curl and wavelength |
| `envelope.test.ts` | `foundation`: profile maximum/bounds/continuity, containment, sampling, zero volume and invalid inputs |
| `torsion.test.ts` | `foundation`: identity/unit output, invalid parameters, sampling floors for writhe/spiral, scale invariance and upward progress; harness bias dials retain each term's observable reach |
| `skeleton/colonize.test.ts` | `colonization.rs`: deterministic growth, parent order, envelope/trunk guards, collisions, caps and invalid/empty cases; `crown_reference.rs`: frozen ordinary/giant diagnostics |
| `skeleton/fill.test.ts` | `colonization.rs`: occupied shell voxels, clustering boundaries and unmeasurable domains |
| `skeleton/persistence.test.ts` | `colonization.rs`: bounded turn and coincident/collision behavior; `growth.rs`: retained local attachment and resolution |
| `skeleton/grow.test.ts`, `presets/two-trees.test.ts` | `growth.rs`: solved deterministic presets, family/seed controls, finite constraints, empty and capped generation; `growth_reference.rs`: seven finished FN6 input classes |
| `skeleton/law.test.ts` | `growth.rs`: branch length/radius law, generation termination, local taper, lateral attachment and resolution independence |
| `skeleton/twigs.test.ts` | `growth.rs`: fixed terminal anatomy, topology, cap propagation; `foliage.rs`: twig stations and phyllotaxis |
| `skeleton/shed.test.ts` | `growth.rs`: whole-run retention/remapping and terminal transitions; `foliage.rs`: shell boundary/underside retention |
| `skeleton/continuity.test.ts` | `growth.rs`: crossover attachment, fork conservation and local radius independence; `surface.rs`: contained sockets and continuation; `growth_reference.rs` / `surface_reference.rs`: actual final FN6 presets |
| `radius.test.ts` | `growth.rs`: conserved structural forks, local solved radii, appended-radius independence, finite rails |
| `mesh/paths.test.ts`, `mesh/frames.test.ts` | `surface.rs`: thickest continuation, contained branch sockets, straight and reversed transported frames; `surface_reference.rs`: final solved-tree connectivity |
| `mesh/surface.test.ts` | `surface.rs`: taper, bounds, winding, normals, nonzero triangles, empty/duplicate/overflow limits and socket containment; `surface_reference.rs`: matched final FN6 meshes |
| `canopy/element.test.ts` | `foliage.rs`: anatomical leaf mesh, bounds and winding; invalid/resource handling |
| `canopy/place.test.ts` | `foliage.rs`: twig stations, deterministic owned frames, zero-edge fallback, clumping, phyllotaxis, invalid/budget handling; `foliage_reference.rs`: retained preset transforms |
| `canopy/cull.test.ts` | `foliage.rs`: whole-leaf shell and underside boundaries, valid emptiness, malformed/nonfinite input rejection; `foliage_reference.rs`: actual preset retention |

`harness/skeleton-view.test.ts` retains parameter translation, deterministic geometry, seed/dial reach, crown draw counts, cap reporting, preset round trips and comparison aggregation. Its centreline/radius probes now read native structure values/topology. `harness/stage.test.ts` owns consumer materials, scene replacement/disposal and bounds. `tests/browser/integration.mjs` owns actual Wasm loading, field-only consumers, buffer ownership and visible failure/retry; `tests/browser/migration.mjs` owns matched browser outputs and captures. New field behavior is independently covered by `field.rs`.

Intentional retirements under the approved no-compatibility capture: tests asserting Three.js output-vector mutation or old wrapper return shapes (including wrong-length parallel radius arrays, now represented by validated native nodes); permissive nonfinite/negative fallback behavior now replaced by explicit invalid-input errors; and historical FN5 fixed-depth/threshold expectations superseded by the radius-driven branch law. Archived FN6 counts/hashes diagnose drift; they do not require reproducing structural defects. Shared mathematical/geometric properties remain native tests, rather than running a second production generator as an oracle.

## Changing a rule or an output

These bounded development exercises identify the owner and the observable assertion; they do not introduce speculative production options.

1. **Botanical rule:** change a family's branch length/radius factor in `presets.rs`, or change its evaluation in `branching/local.rs`. Run `cargo test -p telperion-core --test growth local_anatomy_attachment_taper_and_resolution`. This test varies geometric resolution, checks attachment/taper and preserves terminal twig dimensions; extend its fixture with the proposed branch-law value and assert the intended length change. Inspect the ordinary viewer with the same seed before and after; a rule change should alter structure and derived outputs without changes in `surface`, `field`, or the browser adapter. For a shipped change, adjust its focused expectation and record silhouette/topology differences rather than restoring obsolete FN6 bytes.
2. **Output change:** change a family's surface radial resolution in `presets.rs`, or its ring construction in `surface.rs`. Run `cargo test -p telperion-core --test surface tapered_closed_surface_has_normals_and_bounds`. Check winding, normals and nonzero triangles, then compare `{ structure: true }` and `{ field: true }` before and after: they retain identical structure and occupancy because neither calls the surface builder. The independent request assertions in `tests/browser/integration.mjs` and mesh-free block consumer in `field.rs` enforce that separation. A representation-only change requires no new botanical generator or compatibility layer.

The existing tests exercise those boundaries with small fixtures. Full-size preset captures and timing evidence supplement them; they do not replace the focused checks.

## Species evidence and replay

From the repository root, after installing the pinned Rust toolchain and Node
dependencies:

```sh
npm run wasm:build
npx playwright install chromium
npm run species:measure -- --output /tmp/species-replay
# Separate terminal, leave running:
npx vite --host 127.0.0.1 --port 5184
# Back in the first terminal:
npm run species:qa -- --capture-only --output /tmp/species-replay
```

Use a new output directory for a new measurement run. Without `--output`, bulk
artifacts go to a uniquely named OS temporary directory printed at startup.
The runner uses the durable [seed manifest](../../.flow/evidence/fn9/seeds.json):
12 fixed and 12 fresh per species. Fresh seeds were drawn with OS cryptographic
randomness after calibration and committed before generation. Replay never
draws replacements. `--draw-seeds --seeds NEW_FILE` is for a separately recorded
future protocol, not retrying a failing specimen.

The native `species_measure` example measures actual structure, wood and retained
foliage. It reports DBH as a centreline diameter proxy at 1.3 m, axes as operational
estimates, individual units separately from placement counts, and blade sheet
area separately from needle surface area. Profile contextual/unknown fields do
not become gates. Every per-case JSONL includes machine, source, target and
numeric checks. Pending or interrupted cases remain unassessed.

Capture covers fixed 1/2/3, the first three fresh seeds of each species and every
numeric failure, plus Ordinary/Telperion/Laurelin. No height, node or instance
limits replace mature presets. Whole and bare use the same full-specimen bounds;
bare only hides foliage. The capture runner's foliage-detail camera clips a local
shoot around the middle retained instance and retains original neighbouring
matrices and wood. Supplemental `element` images isolate a single placed unit;
those alone cannot establish attachment. This differs deliberately from the
viewer's isolated-unit detail mode.

Captures use one neutral hemisphere-lit frame, 960×720, DPR 1, 38° perspective,
full-tree direction normalized `(0.62,0.28,1)` and framing margin 1.3. JSON records
exact camera position/target/clipping, material environment, preset, renderer,
browser and SHA-256 hashes of geometry and PNG bytes. One frame followed by GPU
completion avoids continuously queueing work on software WebGL. There is no
hardware performance claim. Ground, scale figure, shadows and extra lights are
absent from this diagnostic rig.

`--timeout-ms` bounds each process (default five minutes). Each receipt is saved
before the next capture; failures persist, and missing PNGs never count as
passes. `--case ID` is a partial diagnostic run, never protocol completion.
`--help` lists environment overrides. Capture success remains distinct from
human inspection: the full runner exits 1 while visual assessment is unassessed,
even when every PNG exists. Record trait/case conclusions and actual owner
feedback separately in [REPORT.md](../../.flow/evidence/fn9/REPORT.md). No automated
capture can award owner approval.

For same-host native costs, build `cargo build --release -p telperion-core
--example measure`, then run `target/release/examples/measure ordinary` (and
`telperion`, `laurelin`). It reports one warmup (`sample=-1`) and five measured
samples, stage times, nodes, retained instances, wood counts and buffer bytes;
RSS is on stderr. Species measurements include separate measurement overhead.
Do not compare these directly with historical measurements on another host or
interpret native time as software/hardware GPU time.

Spruce `4250668600` remains a required mature capture even after its numeric width repair. All full-size captures use the authored presets and recorded seeds, without the smaller fixtures used by the binding/UI integration tests. Those 4 m fixtures use three spruce tiers, three primaries per tier and 0.4 m secondary spacing to exercise ownership, anatomy, orbit and retry behavior within bounded software-rendering cost. The 12,000-instance binding budget still rejects overflow; it is never a truncation policy.

The runner captures whole, bare and attached-foliage views for every required specimen, plus selected element, exterior, branch-curtain, peg and socket views. Exterior views follow an actual terminal twig and its connected parent/socket at multiple angles. Peg/socket cameras retain complete wood and foliage at unrestricted depth. Occluded anatomy remains unassessed in that view; a neighbouring branch or a small numerical origin gap is not visual proof of the selected connection.

`--targets FILE` can retain endpoint/parent/socket identities from a previous capture manifest when topology is unchanged. A topology mismatch fails explicitly. After an architectural correction, select new targets and record their identities; do not reuse stale node IDs.

`--frustum-cull` conservatively rejects only foliage whose transformed prototype bounding sphere lies wholly outside the camera frustum. `--batch-instances` submits every original matrix, in order, in batches of at most 100,000. Neither changes maturity, geometry, foliage density or visible depth. Use both for large captures.

On a Linux host with working Vulkan graphics, the optional wrapper enables GPU-backed headless capture:

```bash
CHROMIUM_EXECUTABLE="$PWD/scripts/species-chromium-gpu.sh" \
  node tests/browser/species.mjs --capture-only --output /tmp/species-run \
  --frustum-cull --batch-instances
```

The wrapper follows [Chromium's documented headless GPU route](https://chromium.googlesource.com/chromium/src/+/HEAD/docs/gpu/using-gpu-hardware-in-headless-chrome.md). The renderer string in each capture is authoritative; this is not a hardware frame-rate benchmark. Omit the override to use Playwright's software backend. The CPU-only native measurements require no browser or GPU.

For branch-level diagnosis, `cargo run --release -p telperion-core --example curtain_audit` reports actual support and needle-bearing lengths. The optional occupancy exporter remains available:

```bash
cargo run --release -p telperion-core --example occupancy_audit -- /tmp/species-occupancy
python3 -m venv --system-site-packages /tmp/species-audit-venv
/tmp/species-audit-venv/bin/pip install numpy pillow
/tmp/species-audit-venv/bin/python scripts/analyze-species-occupancy.py /tmp/species-occupancy
```

These geometric diagnostics do not replace intact rendered views or establish a botanical acceptance threshold. Earlier iterative reports and captures are archived intact in [experiments/fn9-iterations](../../experiments/fn9-iterations/README.md); [the current report](../../.flow/evidence/fn9/REPORT.md) records the latest implementation and verdict.

## Comparative botanical benchmark (fn19)

The [report](../../.flow/evidence/fn19/REPORT.md) and compact
[final evidence](../../.flow/evidence/fn19/final/) supplement fn9 without changing
its historical receipts. Inputs are the frozen
[protocol](../../.flow/evidence/fn19/protocol.json) and
[reference inventory](../../.flow/evidence/fn19/references.json). The twelve mature
cases include six holdouts frozen before diagnostics. Once generated and
inspected these are regression cases; their historical `holdout-at-freeze` role
does not make them fresh for subsequent tuning.

Run from the source checkout, using new directories at every collection step:

```bash
npm ci
npm run wasm:build
cargo build --release -p telperion-core --example geometry_benchmark
cargo test --release -p telperion-core --test geometry_benchmark --test species_metrics
node --test tests/browser/geometry-benchmark.test.mjs tests/browser/geometry-compare.test.mjs
npm run typecheck
# Freeze source/tool/binary content before mature collection.
target/release/examples/geometry_benchmark \
  --protocol .flow/evidence/fn19/protocol.json \
  --references .flow/evidence/fn19/references.json --output /tmp/fn19-numeric-NEW
# Separate terminal; keep this server on this exact source checkout:
npx vite --host 127.0.0.1 --port 5199 --strictPort
# Back in the collection terminal:
export BROWSER_URL=http://127.0.0.1:5199
export CHROMIUM_EXECUTABLE="$PWD/scripts/species-chromium-gpu.sh"
node tests/browser/geometry-benchmark.mjs --prepare --output /tmp/fn19-conditions-NEW
node tests/browser/geometry-benchmark.mjs \
  --conditions /tmp/fn19-conditions-NEW/conditions.json --output /tmp/fn19-visual-NEW
```

The GPU wrapper is optional; actual backend strings are authoritative. Do not
change cameras, parameters, physical resolution, sampling, seeds or foliage to
make a case pass. Preparation generates geometry and freezes semantic targets,
1600×1000 orthographic views and crown ROI bytes. It does not capture images;
its run is intentionally failed/partial for image collection even when its exit
code reports successful condition preparation. Each case has a five-minute cap.
Full capture retains all 84 terminal view records, including failed or unavailable
ones. `--case`/`--view` are partial diagnostics, never full baseline evidence.
Failed preparation targets remain failures in replay; do not fit substitute views.

Replay another source revision by building it in an isolated checkout, running
its numerical collector, serving its own Wasm and capturing against the **same
complete baseline conditions directory**. Transfer `run.json`, `conditions.json`
and all referenced ROI files together. Never run `--prepare` on a candidate.
Keep metric/capture tool bytes identical on both sides. Source revisions and
geometry hashes may differ; protocol/reference/case/parameter and camera rules
must agree. Dirty source is allowed only when every relevant source and binary
is content-identified. Native identity covers core files; visual identity adds
bindings. Their common core files and native support binary must agree within a
combined run, while their aggregate domain hashes naturally differ.

```bash
node scripts/benchmarks/geometry-compare.mjs \
  --protocol .flow/evidence/fn19/protocol.json \
  --references .flow/evidence/fn19/references.json \
  --baseline /tmp/fn19-numeric-BASE --candidate /tmp/fn19-numeric-CANDIDATE \
  --baseline-visual /tmp/fn19-visual-BASE --candidate-visual /tmp/fn19-visual-CANDIDATE \
  --output /tmp/fn19-comparison-NEW
```

Omit both visual options for an explicitly numerical-only comparison. Exit 0
means comparable measured evidence, not unchanged geometry or biological approval.
Metric/image changes are surfaced; failed/missing/interrupted cases, changed
protocols, stale artifacts and incomparable cameras remain inconclusive (exit 1).
Costs stay a separate inconclusive domain: native `generation` measures only
skeleton branching, not mesh/foliage generation, measurement or end-to-end time.
Browser generation, capture preparation, CPU memory, Wasm capacity, estimated GPU
allocation and GPU time retain their own domains and unavailable reasons.
Uncoordinated or contended samples cannot establish a speedup.

A cheap independent native replay and failed/changed comparison controls use
artificial 2m specimens, never substitute mature evidence:

```bash
python3 -B crates/telperion-core/examples/geometry_benchmark/native_controls.py \
  --binary target/release/examples/geometry_benchmark \
  --output /tmp/fn19-controls-NEW --receipt /tmp/fn19-controls-NEW.json
node scripts/benchmarks/geometry-compare.mjs \
  --protocol /tmp/fn19-controls-NEW/protocol.json \
  --references .flow/evidence/fn19/references.json \
  --baseline /tmp/fn19-controls-NEW/baseline --candidate /tmp/fn19-controls-NEW/candidate \
  --output /tmp/fn19-controls-comparison-NEW
```

The [onboarding guide](../../docs/species-onboarding.md),
[template](../../templates/species-profile.md) and
[oak/spruce packets](../../.flow/evidence/fn19/onboarding-examples/README.md)
assign research, profile, capability, implementation and validation ownership.
Research/profile work and species-owned template changes can run concurrently
in separate worktrees. Shared capabilities, registry/binding edits, catalogue
integration and exclusive measurement windows require one coordinator and ordered
dependencies. Recheck admitted species after shared integration.

The later illustrative manifest is validated by
`python3 -B .flow/evidence/fn19/onboarding-examples/verify.py`. It parses Scots pine
but retains `unsupported-anatomy`, `implemented=false`: paired-needle fascicles
and adequate profile evidence are unmet. Never generate pine from the borrowed
parser fixture. A new cohort requires its own benchmark/reference version and
both comparison sides; comparing it with fn19-v1 is inconclusive. Older cohorts
are immutable, not a catalogue size limit.

Fn20/fn21 can cite the stable finding IDs in the report, per-case IDs in
`final/numeric.json`, per-view IDs in `final/visual.json` and the independent
review packet. Collection success, implementer inspection and independent
botanical assessment remain separate. No qualified independent feedback is
available in this baseline: R3's independent portion stays unresolved, and
biological superiority is unestablished.

To reproduce the final combiner's deliberately changed/failed receipt controls
following the tiny native replay above:

```bash
python3 -B .flow/evidence/fn19/final/replay-controls.py \
  --native-controls /tmp/fn19-controls-NEW --output /tmp/fn19-combiner-controls-NEW
```

Those mutations test reporting and rejection paths only. They are labeled
synthetic fixtures and never presented as a production geometry revision.

For a real visual replay of that same artificial cohort, keep the same built
source, Wasm, tool files and browser, and run these sequentially against the
server configured above. Use a disk-backed output root if `/tmp` is small.

```bash
node tests/browser/geometry-benchmark.mjs \
  --protocol /tmp/fn19-controls-NEW/protocol.json --prepare \
  --output /tmp/fn19-visual-controls-NEW/conditions
node tests/browser/geometry-benchmark.mjs \
  --protocol /tmp/fn19-controls-NEW/protocol.json \
  --conditions /tmp/fn19-visual-controls-NEW/conditions/conditions.json \
  --output /tmp/fn19-visual-controls-NEW/baseline
node tests/browser/geometry-benchmark.mjs \
  --protocol /tmp/fn19-controls-NEW/protocol.json \
  --conditions /tmp/fn19-visual-controls-NEW/conditions/conditions.json \
  --output /tmp/fn19-visual-controls-NEW/candidate
python3 -B .flow/evidence/fn19/final/visual-replay-controls.py \
  --native-controls /tmp/fn19-controls-NEW \
  --visual-controls /tmp/fn19-visual-controls-NEW \
  --output /tmp/fn19-visual-comparison-controls-NEW
```

The last command compares both independent runs and asserts unchanged measured
values and image hashes. It then substitutes an existing second-azimuth PNG into
a separate receipt fixture and verifies that the image change is reported.
Original captures remain unchanged. Projected-gap values and geometry changes
are also emitted per view. This small replay tests the workflow; the twelve-case
mature baseline remains a separate cohort.

The mature baseline predates the semantic key-order validator fix. Its capture tool
identity remains historical. For a later geometry comparison, use that
same historical measurement tool on the candidate, or collect both source
revisions again with one corrected tool version and retain the original evidence.
Never relabel old captures with a new tool hash. The small replay uses the corrected
tool on both sides and is not compared directly with the mature cohort.

### Versioned local-anatomy visibility supplement

The separate `fn19-visibility-v2.1` diagnostic protocol lives at
[visibility-v2/protocol.json](../../.flow/evidence/fn19/visibility-v2/protocol.json).
It preserves the v1 population, parameters, selected semantic targets and all
connected wood geometry. Fork captures hide foliage; attached-shoot captures
retain only the original element transforms assigned geometrically to that
terminal segment, including connectors. This declared filtering exposes anatomy
and must never be used as a natural full-foliage or density comparison. Original
v1 occluded views and their statuses remain unchanged.

With the pinned generator, built Wasm and local Vite server available, run:

```bash
BROWSER_URL=http://127.0.0.1:5199 \
CHROMIUM_EXECUTABLE=/tmp/fn9-chromium-gpu \
node tests/browser/geometry-visibility.mjs \
  /tmp/fn19-mature-visual-20260906 /tmp/fn19-visibility-NEW
```

`CHROMIUM_EXECUTABLE` is the actual local GPU wrapper used for this evidence;
replace it with an available browser executable for another machine and retain
the resulting backend identity. The runner refuses an existing output directory,
checks every generated array against the original mature receipt, records every
retained foliage instance index and searches the same finite camera directions
for each specimen. The optional final integer selects one case for diagnostics;
a complete supplement requires all twelve cases and both local views.

The output retains original 64- and 128-sample PNGs, source/tool hashes, original
geometry mapping, cameras, visible-probe measurements, failures and pending human
assessment. Every final image still requires direct inspection: a ray probe alone
does not establish that the named anatomy is visible. Keep hidden anatomy
unassessed and preserve failed attempts. Filtered images cannot establish hidden
rear continuity, independent botanical approval or calibrated species dimensions.

This diagnostic runner is restricted to the pinned original mature geometry. It
rejects changed generated array hashes and does not implement changed-revision
comparison. `visibility-v2/capture.json` retains each exact camera, target node
IDs, retained original instance IDs and filtering conditions. A future candidate
adapter must reuse those baseline cameras and declared filters while validating
semantic target/attachment mapping; searching or refitting cameras independently
for a candidate is not a matched comparison. Do not feed these two filtered
channels into the v1 seven-view comparison or projected-gap statistics.
