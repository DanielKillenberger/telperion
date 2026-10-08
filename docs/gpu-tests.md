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
  `every_pipeline_within_2_5_s_and_the_first_tree_within_5_s_cold` builds
  a first tree with the driver's cache off (`__GL_SHADER_DISK_CACHE=0`).
