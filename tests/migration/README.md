# Rust migration foundation

Install Rust with rustup using its standard installer, then from the repository root:

```sh
rustup toolchain install 1.98.1 --profile minimal --component rustfmt --component clippy --target wasm32-unknown-unknown
npm ci
npm run rust:build
npm run rust:test
```

The toolchain file pins native and Wasm builds. No build script depends on `/tmp` or a machine-specific toolchain installation. The core and Wasm crates have no external Rust dependencies. `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings` check style and compiler diagnostics.

For the browser proof, install Playwright separately or make it available to this repository, install its Chromium browser, then run `npm run rust:test:wasm`. `PLAYWRIGHT_MODULE` may name an absolute Playwright `index.mjs`; `CHROMIUM_EXECUTABLE` optionally selects a local Chromium. These are explicit test dependencies, not production dependencies. The runner builds Wasm, invokes the native example, and compares the same deterministic 64-point envelope sample in Chromium.

The foundational ABI owns one output buffer per Wasm instance. `sample_envelope` returns status 0 (success, including empty), 1 (invalid input), or 2 (resource limit). Every request clears the previous logical output, including errors. `output_ptr` and `output_len` borrow memory only until the next mutating call; callers copy it before `release` or another request. `release` drops storage and is idempotent. No caller-provided pointer is dereferenced. This narrow proof will be extended by the full generator binding task.

## Reference

```sh
npm run migration:reference
npm run migration:reference -- ordinary capped
REFERENCE_BUFFERS=1 REFERENCE_OUTPUT=/path/to/output npm run migration:reference -- telperion
```

The exporter requires final FN6 commit `fdafb099b1495519de75a6b9a66d37f7d07e47bd` in the local Git object database. Missing or mismatched provenance fails visibly. It extracts that exact Git archive into a disposable directory, bundles only the reference runner there, executes cases sequentially and removes the directory. Production never imports the archived generator. Use `npm ci` first; the archive borrows the installed dependencies. Generated data lives in ignored `tests/migration/generated/` by default.

Cases cover ordinary seed 42, authored Telperion and Laurelin presets, zero attractors, zero width, high crown base (envelope crossing), and a 20-node cap. Their raw input parameter manifests, fixed camera definitions, attractor/node samples, structural diagnostics, stage sizes and SHA-256 hashes are exported. `REFERENCE_BUFFERS=1` also emits exact typed-array bytes for nodes/parents/branch allocations/radii/surface/foliage. The JSON states value counts; `.bin` types follow the corresponding TS arrays (positions/radii Float64, surface/foliage Float32, parents/branch IDs Int32, indices Uint32, twig Uint8). Buffers use host endianness; the captured host is little-endian. Giant buffers are reproducible, not committed. `fixtures/` holds compact final-reference records; timings are observations, not performance gates.

## Comparison policy selected before botanical ports

The old generator is a diagnostic reference, not an exact topology or byte-compatibility requirement. `node scripts/compare-migration.mjs REFERENCE_DIRECTORY CANDIDATE_DIRECTORY` reports node/vertex/foliage count ratios, bounds drift, cap state and stage hash changes. Candidate records follow the exported JSON schema. A missing case, mismatched reference revision, or false candidate invariant fails. A changed topology or hash alone does not fail. Record and investigate drift, then compare the ordinary and giant images with the same camera, neutral material, viewport and foliage visibility. Foundation does not claim the botanical visual comparison has happened.

Required invariants: finite positions/transforms; parent-before-child tree order; positive solved radii; proximal edge radius at least distal; surface indices in range; explicit cap diagnostics; valid empty regions. Fork conservation above crossover, branch-local taper, twig anatomy and shell containment belong to the stage ports. Boundary fixture `empty` records old FN6's one root node and empty representations; it does not require Rust to retain an artificial root for an empty tree.

For matched numeric primitives/native-Wasm comparisons, use absolute position tolerance `1e-10 * max(1,height)`, radius tolerance `1e-9 * max(1,radius)`, float32 output tolerance `8 * 2^-23 * max(1,height)`, and unit-frame component tolerance `1e-5`. Field signed-distance comparisons use the position tolerance, except occupancy exactly at a boundary where distance and cell overlap must be reported. Reference-vs-rewrite topology drift invalidates elementwise comparisons; compare bounds, quantiles, count ratios, invariants and images instead. Exact RNG u32 streams remain portable; hash matches diagnose determinism, not old architecture compatibility.
