# fn-260 friction

## 2026-10-09, task 1

- **The dcg hook blocked two ordinary commands.** It refused a log redirect through a shell variable path and a `git checkout --` that restored one temporarily edited preset file. Each had to be retyped, with a literal path and a `sed` restore. Cost: about 2 minutes. This is a local setup issue (fn-248 reported the same), reported here and not specced.
- **`cargo nextest` is not installed on this machine.** AGENTS.md names nextest as the profile CI runs under, so per-test times came from `cargo test --profile ci` with `--nocapture` timings instead. Cost: under a minute. Local setup, not specced.
- **`cargo clippy -p telperion-render` cannot reach the render crate.** It stops on telperion-core's pre-existing clippy errors (approximate `FRAC_PI_2` in the generated `presets.rs`, among others), the failure fn-248 reported and fn-237 R7 takes. The new render code got no lint pass. Cost: about 3 minutes for one clippy build. What would remove it: fn-237 R7's clean lib clippy.
- **Package-by-package gating breaks telperion-jev's replay test.** `tests/replay.rs` runs telperion-core's `species_measure` example, which only a workspace build (or `-p telperion-core`) produces, so jev run before core fails two tests with "is missing: run the workspace gate". Cost: one rerun of jev's replay after core, about 2 minutes. What would remove it: the replay test building its example itself, or its gate order written down beside the package-by-package gate.
- **The render half ran 8.5 minutes and its cold-compile test went red under its own neighbours again.** `grow` took 20.96 s against its 20 s bound with 4 test threads, and 15.3 s alone (fn-237 R9, fn-248 reported the same). Cost: one extra locked GPU run, about 1 minute.
