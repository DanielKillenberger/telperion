# GPU tests beside the harness

The render crate's GPU tests share the card with whatever else is open on
it, the harness included. A test run that takes the driver's address
space can leave an open page's WebGPU device lost, and in Chrome a lost
GPU process can take WebGPU away until a reboot (fn-235).

- **Device-limit probes run only when asked.** A test that takes the device
  toward its limits (a tree at 80 built on it, a budget or out-of-memory
  probe) is marked `#[ignore = "device limits: <what it measures>"]`. The
  default run (`cargo test --profile ci -p telperion-render`) leaves it
  out; `-- --ignored <name>` runs it, alone, under the GPU lock and with
  no harness open. `device_limit_probes_run_only_when_asked` holds every
  marked test to that.
- **The default suite stays small.** Measured 2026-10-09 on the RTX 3080,
  the heaviest default test's device peaks at 2.6 GiB of owned buffers,
  and whole-tree tests take the device one at a time in a process
  (`generation::whole_tree`).
- **A cold driver cache is part of the picture.** The NVIDIA driver caches
  compiled pipelines on disk; a fresh browser profile does not have them.
- **Release checks run only when asked, on a quiet machine.** A test whose
  answer depends on the machine's load or on a fetched tool is marked
  `#[ignore = "release check: <what it needs>"]`, held to that by the same
  test as the device-limit probes, and run before a release PR (fn-237 R9,
  R14):
  - `every_pipeline_within_2_5_s_cold` builds a first tree with the
    driver's cache off (`__GL_SHADER_DISK_CACHE=0`), holds every pipeline
    to 2.5 s and reports the cold total. A compile time measures the
    machine's load as much as the shader (fn-246 FRICTION.md), so the
    default suite holds each entry's inlined size to a ceiling instead
    (`compile_size::tests::every_entry_inlines_under_the_ceiling`), and
    `compile_size::tests::every_entry_made_cold_alone` measures what that
    ceiling is read from.
  - `every_shader_passes_tint` runs every shipped module through Tint, the
    WGSL compiler in Chrome, one entry at a time as Dawn makes a pipeline.
    Tint refuses WGSL that naga accepts (a NaN constant, a barrier under
    non-uniform control, `|` beside `<<` without parentheses). The binary
    is the prebuilt `bin/tint` in Dawn's Linux release tarball, from
    Dawn's GitHub releases (checked with v20261009.190412): `gh release
    download -R google/dawn -p '*ubuntu-latest-Release.tar.gz'`, unpack,
    then `TINT=<dir>/bin/tint cargo test --profile ci -p telperion-render
    every_shader_passes_tint -- --ignored`. It needs no device. The
    default suite holds Tint's NaN, uniformity and operator-mixing rules
    with checks of its own (`tint_rules`, `tint_mix`).
- **The release scorecard's probe** is `frames`, natively (fn-259 R1, R4):
  `AGE=<years> SEED=<seed> cargo run --profile ci -p telperion-render
  --example frames -- <preset> full whole mid close` prints one JSON line
  with the age and seed asked, `fresh_ms` (from `Renderer::new`, shader
  compilation included, to a fenced first frame), `warm_ms` (a second
  build to a fenced frame), frame time per view, `peak_rss_mb` and
  `device_memory` with its source. Its `wgpu_internal_mib` grows with the
  timing loop's pipelined frames, not with the tree. `ONE_BUILD=1` builds
  once and draws four hero frames for rows that only need memory; its
  timing keys are null.
- **A scorecard row runs under `node scripts/gpu-row.mjs <rows.jsonl> --
  <probe command...>`**, inside `telperion-heavy` (fn-259 R4). It holds
  `${XDG_RUNTIME_DIR}/telperion-heavy.lock` for the whole row, so no
  heavy job is admitted meanwhile, and starts only when no other
  `telperion-heavy-*` unit runs and no process naming telperion is on the
  card. It samples `nvidia-smi --query-compute-apps` every 250 ms and
  appends the probe's JSON with the card's state at the start and the
  end. A process that appears during the row and is not the probe's
  makes the row `valid: false`, as a non-zero exit does. The processes on
  the card at the start, the desktop's clients among them, are written
  into the row and do not invalidate it, so a compute job outside
  Telperion already running at the start is recorded, not caught (host,
  2026-10-19). A GPU user that lives under 250 ms is not caught either.
- **Two stills are compared by `node scripts/pixel-diff.mjs <a.png>
  <b.png>`**, which counts the pixels where any channel differs and where
  any differs by more than 8/255, over the whole frame. ImageMagick 7's
  `compare -metric AE` weighs pixels by how much they differ and
  understated a difference 100× (fn-258 FRICTION.md).

## The browser suites

- **The binding suite needs no GPU.** `tests/browser/bindings.mjs` holds
  the Wasm core to its contract in a plain headless Chromium with no
  WebGPU flag: `npm run test:bindings` (`scripts/test-bindings.mjs`)
  builds the core, serves the source on a free port and runs it alone.
  CI's `bindings` job runs it on every run, with no receipt skip. A check
  that needs an adapter does not belong in it.
- **The field suite needs WebGPU.** `tests/browser/field.mjs` grows its
  trees on a WebGPU device in a worker. `npm run rust:test:wasm`
  (`scripts/test-wasm.mjs`) runs it, only in a browser that has WebGPU,
  and no CI job runs it: CI has no GPU runner.
