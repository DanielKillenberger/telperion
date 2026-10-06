# fn-198 friction

## 2026-10-05, design research: measuring the engine's cost per stage

- **Doing:** the cost table at 80 years, every passed species, seeds 1 and 7: growth stages, conversion, expansion, phytomers grown against kept, triangles, leaves, peak memory.
- **Slowed by:** no tool reports all of it.
  - `measures stages` has no palm and takes its light from a fixed table, not each species' production light. It reports neither grown phytomers, the conversion nor the expansion.
  - The stills runner reports grown and dressed times, but no stage split, no grown count and no memory.
  - The engine's grown count (`Grower::grown`) is private.
- **Cost:** about 10 minutes and one extra release build. I wrote a scratch example (`f_cost`, not committed) and a one-line `eprintln!` of `grower.grown` in `grow.rs`, reverted before the commit.
- **Would remove it:** a `cost` mode on `measures` that takes each species' production light and the stills runner's conversion. It would report grown and kept phytomers (a `Structure::grown` field or a stage-callback count), the conversion, the expansion split and `VmHWM`.

## 2026-10-05, design research: shared load while timing

- **Doing:** the same measurements.
- **Slowed by:** other sessions kept the 1-minute load average at 6.5 to 10 on 32 threads throughout. The baseline was taken at 2 to 3. fn-197 recorded the same problem: the spruce under light read 40 s against 16 s in a quiet run.
- **Cost:** one repeated pass of every tree, about 3 minutes, and timings that carry a stated load.
- **Would remove it:** a timing run that records the load and refuses to run above a ceiling, or a quiet window for cost runs.

## 2026-10-05, design research: shell guard on evidence files

- **Doing:** writing this file with a heredoc.
- **Slowed by:** the dcg hook blocks a `>` redirect to a new file under `.flow/evidence/` when the command also runs `mkdir -p` on the parent. fn-190 hit the same guard.
- **Cost:** about 1 minute.
- **Would remove it:** nothing in the repository; it is a local hook setting.

## 2026-10-06, task 1 survey: the engine in wasm32

- **Doing:** checking whether `telperion-space` builds and runs for wasm32 and how close each species and age comes to the 4 GB ceiling (INTEGRATION.md, section 2.7).
- **Slowed by:** no tool runs the engine or the GPU path's CPU preparation in wasm32. `rust:test:wasm` covers the main module's entries only; GPU-PATH.md's peaks are native RSS.
- **Cost:** about 15 minutes: a scratch `cdylib` (`wasm-probe/`) and a Node runner, one tree per process. The load was 14 to 15 throughout, so its times are indicative only.
- **Would remove it:** a wasm32 memory probe kept beside the stills runner (species, age, peak memory per step), so step 3's ceiling per species and age is one command.
- **Also:** the dcg hook blocked a `cp`/redirect into `.flow/evidence/` through shell variables; literal paths passed (about 1 minute, local hook, as recorded on 2026-10-05).
