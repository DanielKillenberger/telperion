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
