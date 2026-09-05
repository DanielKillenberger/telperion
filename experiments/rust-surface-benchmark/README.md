# Rust surface benchmark

An isolated surface-stage experiment comparing the frozen production TypeScript,
a preallocated TypeScript variant, and one Rust mesher compiled for native and
browser Wasm. See [REPORT.md](REPORT.md) for measured results and the migration
decision; [results/measurements.json](results/measurements.json) holds raw samples.
Nothing in production imports this experiment.

## Reproduce

From the repository root, on Linux with Node >=20, npm, Rust with Cargo/rustfmt,
and Chromium installed:

```sh
npm ci
rustup target add wasm32-unknown-unknown
rustup component add rustfmt
cargo fmt --manifest-path experiments/rust-surface-benchmark/Cargo.toml -- --check
node experiments/rust-surface-benchmark/build.mjs
node experiments/rust-surface-benchmark/run.mjs test
node experiments/rust-surface-benchmark/run.mjs bench
npm run typecheck
npx vitest run src/mesh/surface.test.ts src/mesh/frames.test.ts src/mesh/paths.test.ts --maxWorkers=1 --testTimeout=60000
```

The measured toolchain is Rust 1.98.1; `rustup override set 1.98.1` in this
experiment directory can select it. The root npm lock supplies esbuild and Three;
there are no additional npm dependencies or Rust dependencies. Cargo.lock belongs
to this experiment. Override `CHROMIUM=/path/to/chromium` when it is not installed
at `/usr/bin/chromium`. Cargo and rustc must be on PATH. A missing Wasm target is
fixed by the `rustup target add` command above. Run the build first if fixtures or
binaries are missing. Frozen-source hash failures mean the checked-in snapshot
has changed: restore it before comparing results.

The runner starts a loopback HTTP server and a real headless Chromium process
with a temporary profile, disabled GPU, and `--no-sandbox`; run only these trusted
local sources. It does not require a display, browser driver, FN-6 checkout, or
network during measurements. `build/` and `target/` are ignored. Build regenerates
fixtures from the included frozen source, verifies every source hash, builds both
Rust targets in release mode with warnings denied, and bundles the browser code.
`test` writes `build/correctness.json`; `bench` overwrites the checked-in
`results/measurements.json`. Timing values naturally change on another run.

## Inputs and implementations

`provenance.json` records revision
`019234b554a1b387e23b05dd3c7e2626f428f907` and SHA-256 for the copied source files.
The snapshot includes the growth/radius dependencies needed to regenerate the
Telperion and Laurelin inputs from their shipped seeds and parameters. No imports
reach the live production source. The generated fixture manifest and results
include each complete packed input hash, byte count, seed, and preset parameters.
The synthetic cases cover empty/root, straight, fork/root fork, repeated points,
reversal, zero burial/lobes, and parameter sanitization/extreme values.

The optimized TS retains the reference geometry algorithm, path extraction and
transported frames; it counts output capacity first, writes typed arrays directly,
and avoids temporary vertex-vector allocation in the hot loop. The Rust port uses
f64 intermediate math and f32 output with the same frames, lobes, twist, flare,
fork sockets/swell, caps and triangle order. It also uses preallocated flat output.
This comparison therefore measures implementations and data layouts, not the
isolated causal effect of a language change.

The private trusted-input ABI packs little-endian f64 values: node count, envelope
height, nine surface parameters (order in `shared.ts`), then six values per node
(parent, x/y/z, radius, start radius). It intentionally has no general-purpose
validation API. Only this harness produces its inputs.

Wasm `input_resize` reserves an input vector and returns its pointer; the caller
refreshes its memory view and copies the entire packed input once. `compute`
replaces the Rust-owned output vectors. Positions/indices pointers are borrowed
until the next compute; memory growth can invalidate JS views, so views are
created after allocation and output is copied immediately into independently
owned Float32Array/Uint32Array buffers. The module retains input/output storage
between calls and linear memory can grow but does not shrink. No renderer or GPU
upload is involved.

## Correctness and timing boundaries

All four targets must have identical counts and index order (therefore identical
connectivity and winding), finite coordinates and in-range indices. The position
tolerance, selected before execution, is `8 * 2^-23 * max(1, maxAbsInputCoordinate)`.
It allows cross-target f32 rounding; it is not fitted to measured differences.
Small cases additionally verify every undirected edge appears twice with opposite
orientation. SHA-256 checks enforce byte-identical TS variants and determinism
within each target; native is invoked twice in test mode. Preset output is checked
in actual Chromium against the native serialized output, not a Node Wasm proxy.

Browser TS compute and caller paths both build independently owned output from
already available JS skeleton/radius objects. Wasm compute starts with packed
input resident in linear memory and ends with Rust-owned output there. Wasm
caller timing includes packing the same JS objects, resizing/copying input,
compute, and copying both outputs into independent JS buffers. Thus only the
caller measurement has comparable returned ownership. Compute-only ratios omit
real Wasm costs. Each browser path gets three warmups, then ten measured samples;
implementation order rotates each repetition. Compute always precedes caller.
GC is not forced, and CPU clocks, scheduling and thermal state are uncontrolled.

Native runs separately, one process per case. Its compute timer includes the
shared build from packed f64 input; caller includes decoding input bytes into an
owned vector and building owned output. Three build warmups precede ten samples
per path. File I/O, process startup, final output serialization and fixture
generation are excluded from these steady-state timers. Native is not a
browser end-to-end substitute. Median is the middle-pair average; p95 uses nearest
rank, so with ten samples it is the maximum. All timings are milliseconds.

The separately labeled browser cold timing is one fetch/compile/instantiate on a
fresh profile using loopback HTTP, not application startup or a network deployment
measurement. Memory labels and their limitations are detailed in the report;
`totalBrowserPeak: null` means unavailable, not zero.
