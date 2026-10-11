# FRICTION: fn-263

## 2026-10-11: no readback of the device's per-phytomer sag inputs

- **Doing:** R1. I was finding which input of the birch's sag differs between the executors.
- **Slowed by:** the device keeps a phytomer's lever, its running direction before the sag and its turn in private scratch, and nothing reads them back. I had to patch `walk.wgsl` and `lay.wgsl` temporarily to write those values into the laid buffer's spare floats, then revert the patch. Separately, the dcg hook refused shell redirects to a `$VAR` path, and I rewrote the runner with literal paths.
- **Cost:** about 10 minutes and three rebuilds of about 30 s each. The dcg refusal cost about 1 minute.
- **Would remove it:** a test-only device probe that returns one named phytomer's step inputs, the way `the_gpu_takes_every_decision_the_cpu_does` probes decisions.

## 2026-10-11: the gate's doctests hit the /tmp quota

- **Doing:** the R3 gate, running `cargo test --profile ci -p telperion-core`.
- **Slowed by:** every lib and integration test passed, but rustdoc failed to write its argument file because the per-user quota on tmpfs `/tmp` was exceeded. About 7.9 GB of the session scratchpad, mostly other runs' captures, sits on that tmpfs.
- **Cost:** the doctests had to be re-run with `TMPDIR` on the disk, about 3 minutes.
- **Would remove it:** pointing `TMPDIR` for gate runs at a directory on disk, or pruning old scratchpad captures. This is a local setup problem, not a repository one.

