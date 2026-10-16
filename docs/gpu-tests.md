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
