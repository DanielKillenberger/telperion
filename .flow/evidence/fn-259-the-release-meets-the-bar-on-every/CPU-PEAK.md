# fn-259: where F4's extra CPU peak comes from

Measured 2026-10-19 on the owner's desk (RTX 3080, driver 610.57.04, Vulkan through wgpu 30.0.1). Question from the host: F4's one-build CPU peak (`VmHWM`, `frames` with `ONE_BUILD=1`) is about 760–800 MB on every preset against 250–350 MB for 0.1.4, an excess that looks the same for the palm as for Telperion.

## Finding

- **The excess appears in `prepare_programs`, and nowhere else.** With the shader disk cache off, the palm's `VmHWM` goes from 236 MB to 799 MB in that one call. Telperion's goes from 237 MB to 771 MB. No later stage, including the build, raises the peak on either preset. 0.1.4 has no such stage: its peak is set by `Renderer::new` (250 MB), and for Telperion by its CPU build (351 MB).
- **Our own allocations are not the cause.** A counting global allocator in the probe shows the Rust heap at 20 MB live after `prepare_programs`, with a peak of 26–37 MB, on both presets. The rest of the rise is anonymous memory from outside the Rust allocator: the NVIDIA driver's compiler and the glibc arenas of the threads it runs on.
- **What drives it:**
  1. **206 compute pipelines against 10.** F4 makes 206 compute pipelines (counted with a scratch counter on the palm). 0.1.4 makes 10 generation pipelines, plus 3 for select.
  2. **All of a module's entries are compiled at once.** `Program::make_many` (`crates/telperion-render/src/programs.rs`) spawns one OS thread per entry point. `Kit::new` and `io::pair` also run the kernels, the tools and the curve passes side by side, so dozens of driver compiles run concurrently.
- **Each part is checked by an experiment** (cache off, seed 1, three runs each, `raw/cpupeak/variants.log`):

  | F4 variant | palm `VmHWM` MB | Telperion `VmHWM` MB |
  |---|--:|--:|
  | as shipped, one thread per pipeline | 778–800 | 743–776 |
  | `make_many` serialized (scratch build, `PROBE_SERIAL=1`) | 441–466 | 497–498 |
  | as shipped, with `MALLOC_ARENA_MAX=2` | 460–502 | 475–504 |
  | as shipped, with a warm shader disk cache | 292–295 | 351–354 |
  | 0.1.4, cache off / cache on | 249–250 / 136–138 | 350–351 / 304–306 |

  - **Concurrency (−300 MB).** Serializing the compiles takes about 300 MB off the peak, and so does capping glibc's arenas. Most of that 300 MB is therefore per-thread malloc arenas and simultaneous driver working sets, which glibc does not return to the OS afterwards: `VmRSS` stays 630–690 MB after the call.
  - **Pipeline count (about +200 MB).** Even serialized, F4 holds about 265 MB anonymous memory after compiling, against 65 MB for 0.1.4. That gap is the driver's compiler and its compiled objects for 20 times as many pipelines.
  - **Warm cache.** With a warm disk cache almost all of this goes away. The palm still sits about 156 MB above 0.1.4: about 82 MB anonymous (the driver's retained pipeline objects) and about 72 MB file-backed. The file-backed part appears at `prepare`, the first build. It is most likely host-visible device memory mapped through the driver (not examined further).
- **The excess is a fixed cost, not tree-sized**, as the host suspected. It scales with the program count and the compile threads, not with the tree.

## Would a browser pay it?

Not in the page's Wasm heap. In a browser, `make_many` runs no threads (the `wasm32` branch asks `create_compute_pipeline_async` of each pipeline, `programs.rs`). Dawn compiles in Chrome's GPU process, on Chrome's own worker pool, through the same driver. The page's Wasm heap would carry what the Rust heap carries here, a peak of 26–37 MB. The driver's compile memory would show in the GPU process, transiently, bounded by Chrome's pool rather than by 206 threads. Chrome's persistent GPU caches would make a returning visit look like the warm-cache row. None of this was measured (no agent runs WebGPU); it is read from the code and the process model.

## Proposed fix (not implemented; the host decides)

- **What:** cap the native `make_many` at a small pool, for example `std::thread::available_parallelism()` or a fixed 4–8 workers, in place of one thread per entry. The browser path is untouched.
- **Expected saving:** close to the serialized row, about 300 MB off the cache-off peak on every preset, bringing the palm to about 450–500 MB and Telperion to about 500 MB. The remaining gap to 0.1.4, about 200 MB cold and 70–160 MB warm, is the pipeline count itself.
- **What it costs:** fresh compile time. fn-256 (DESIGN.md 9.3) chose one thread per entry so that a module's cold compile takes as long as its slowest entry, not the sum. These runs did not time the serialized or pooled variants. Measuring `fresh_ms` with a pool of 4, 8 and 16 workers is the step before choosing a size.
- **A cheaper option, process-level only:** the native harness and CLI could set `M_ARENA_MAX` (`mallopt`) at start-up. It measured about the same saving with no code change in `programs.rs`, but it is a glibc setting, not a property of the library.

## Method

- **Probe:** `examples/cpu_stages.rs`, an untracked example beside `frames`, removed after the runs, built `--profile ci`. It follows `frames`' `ONE_BUILD=1` route exactly. At each stage it prints `VmRSS`, `VmHWM`, `RssAnon` and `RssFile` from `/proc/self/status`, plus the live and peak bytes of a counting `#[global_allocator]`.
- **0.1.4 counterpart:** the same probe against 0.1.4's API, in a scratch export of `v0.1.4` with `v014-probe.patch` applied, built in its own target directory (`v014-target`). 0.1.4 has no `prepare_programs`, so that row repeats `Generator::new`.
- **Runs:**
  - one job at a time under `telperion-heavy 6 --gpu 3 --threads 4`, with `RAYON_NUM_THREADS=4`, seed 1;
  - three runs per version, preset and cache setting;
  - cache off: `__GL_SHADER_DISK_CACHE=0`;
  - cache on: a private `__GL_SHADER_DISK_CACHE_PATH` per version, warmed by one unrecorded run.
- **Not run through `gpu-row.mjs`:** the probe prints no JSON row, so these jobs have no foreign-process check. Runs were consistent within about 30 MB.
- **Scratch variants (reverted):**
  - the pipeline counter and `PROBE_SERIAL` were a temporary edit to `programs.rs`, built and copied out, then reset with `git checkout`;
  - 0.1.4's counter was an edit in the scratch export only.
- **Raw logs** (ignored): `raw/cpupeak/stages.log`, `raw/cpupeak/variants.log`, `raw/cpupeak/count.log`.

## Stage tables (MB, median of 3; `VmHWM` with its range)


### f4 date-palm cache off (n=3)

| Stage | VmRSS | VmHWM | RssAnon | RssFile | Rust heap live | Rust heap peak |
|---|--:|--:|--:|--:|--:|--:|
| start | 3 | 3 (3–3) | 0 | 2 | 0 | 0 |
| adapter+device | 151 | 151 (151–151) | 27 | 118 | 0 | 0 |
| Renderer::new | 191 | 236 (234–237) | 64 | 121 | 4 | 5 |
| Generator::new | 219 | 236 (234–237) | 91 | 121 | 5 | 6 |
| prepare_programs | 691 | 799 (780–813) | 560 | 123 | 20 | 26 |
| prepare | 761 | 800 (780–813) | 560 | 193 | 21 | 26 |
| submit_prepared | 761 | 800 (780–813) | 560 | 193 | 21 | 26 |
| first build+frame | 761 | 800 (780–813) | 560 | 194 | 21 | 26 |
| four frames | 761 | 800 (780–813) | 560 | 194 | 21 | 26 |

### f4 date-palm cache on (n=3)

| Stage | VmRSS | VmHWM | RssAnon | RssFile | Rust heap live | Rust heap peak |
|---|--:|--:|--:|--:|--:|--:|
| start | 3 | 3 (3–3) | 0 | 2 | 0 | 0 |
| adapter+device | 126 | 126 (126–126) | 25 | 96 | 0 | 0 |
| Renderer::new | 135 | 135 (135–135) | 32 | 96 | 4 | 5 |
| Generator::new | 139 | 139 (139–139) | 36 | 96 | 5 | 6 |
| prepare_programs | 209 | 209 (208–211) | 103 | 98 | 20 | 36 |
| prepare | 293 | 293 (292–294) | 117 | 169 | 20 | 36 |
| submit_prepared | 293 | 293 (292–294) | 117 | 169 | 20 | 36 |
| first build+frame | 293 | 293 (292–295) | 117 | 169 | 21 | 36 |
| four frames | 294 | 294 (292–295) | 117 | 169 | 21 | 36 |

### v014 date-palm cache off (n=3)

| Stage | VmRSS | VmHWM | RssAnon | RssFile | Rust heap live | Rust heap peak |
|---|--:|--:|--:|--:|--:|--:|
| start | 3 | 3 (3–3) | 0 | 2 | 0 | 0 |
| adapter+device | 151 | 151 (150–151) | 27 | 118 | 0 | 0 |
| Renderer::new | 190 | 250 (249–250) | 64 | 121 | 3 | 4 |
| Generator::new | 190 | 250 (249–250) | 64 | 121 | 4 | 4 |
| prepare_programs | 190 | 250 (249–250) | 64 | 121 | 4 | 4 |
| prepare | 192 | 250 (249–250) | 65 | 121 | 5 | 5 |
| submit_prepared | 192 | 250 (249–250) | 65 | 121 | 4 | 5 |
| first build+frame | 192 | 250 (249–250) | 65 | 121 | 4 | 5 |
| four frames | 192 | 250 (249–250) | 65 | 121 | 4 | 5 |

### v014 date-palm cache on (n=3)

| Stage | VmRSS | VmHWM | RssAnon | RssFile | Rust heap live | Rust heap peak |
|---|--:|--:|--:|--:|--:|--:|
| start | 3 | 3 (3–3) | 0 | 2 | 0 | 0 |
| adapter+device | 126 | 126 (125–126) | 24 | 96 | 0 | 0 |
| Renderer::new | 132 | 132 (132–132) | 30 | 96 | 3 | 4 |
| Generator::new | 134 | 134 (134–134) | 32 | 96 | 4 | 4 |
| prepare_programs | 134 | 134 (134–134) | 32 | 96 | 4 | 4 |
| prepare | 136 | 136 (136–136) | 33 | 96 | 5 | 5 |
| submit_prepared | 136 | 136 (136–136) | 33 | 96 | 4 | 5 |
| first build+frame | 136 | 136 (136–136) | 33 | 97 | 4 | 5 |
| four frames | 138 | 138 (138–138) | 35 | 97 | 4 | 5 |

### f4 telperion cache off (n=3)

| Stage | VmRSS | VmHWM | RssAnon | RssFile | Rust heap live | Rust heap peak |
|---|--:|--:|--:|--:|--:|--:|
| start | 3 | 3 (3–3) | 0 | 2 | 0 | 0 |
| adapter+device | 151 | 151 (151–151) | 27 | 118 | 0 | 0 |
| Renderer::new | 192 | 237 (235–237) | 65 | 121 | 4 | 5 |
| Generator::new | 220 | 237 (235–237) | 92 | 121 | 5 | 6 |
| prepare_programs | 648 | 771 (765–777) | 518 | 123 | 20 | 29 |
| prepare | 767 | 771 (766–784) | 561 | 199 | 22 | 29 |
| submit_prepared | 767 | 771 (766–784) | 561 | 199 | 22 | 29 |
| first build+frame | 768 | 771 (766–785) | 561 | 200 | 22 | 29 |
| four frames | 768 | 771 (766–785) | 561 | 200 | 22 | 29 |

### f4 telperion cache on (n=3)

| Stage | VmRSS | VmHWM | RssAnon | RssFile | Rust heap live | Rust heap peak |
|---|--:|--:|--:|--:|--:|--:|
| start | 3 | 3 (3–3) | 0 | 2 | 0 | 0 |
| adapter+device | 126 | 126 (126–126) | 25 | 96 | 0 | 0 |
| Renderer::new | 135 | 135 (134–135) | 32 | 96 | 4 | 5 |
| Generator::new | 138 | 138 (138–139) | 35 | 96 | 5 | 6 |
| prepare_programs | 209 | 209 (208–210) | 104 | 98 | 20 | 37 |
| prepare | 353 | 353 (351–354) | 171 | 174 | 22 | 37 |
| submit_prepared | 353 | 353 (351–354) | 171 | 174 | 22 | 37 |
| first build+frame | 353 | 353 (351–354) | 171 | 174 | 22 | 37 |
| four frames | 353 | 353 (351–354) | 171 | 174 | 22 | 37 |

### v014 telperion cache off (n=3)

| Stage | VmRSS | VmHWM | RssAnon | RssFile | Rust heap live | Rust heap peak |
|---|--:|--:|--:|--:|--:|--:|
| start | 3 | 3 (3–3) | 0 | 2 | 0 | 0 |
| adapter+device | 150 | 150 (150–150) | 27 | 118 | 0 | 0 |
| Renderer::new | 190 | 249 (249–250) | 64 | 120 | 3 | 4 |
| Generator::new | 190 | 249 (249–250) | 64 | 121 | 4 | 4 |
| prepare_programs | 190 | 249 (249–250) | 64 | 121 | 4 | 4 |
| prepare | 255 | 351 (350–351) | 64 | 185 | 4 | 111 |
| submit_prepared | 255 | 351 (350–351) | 64 | 185 | 4 | 111 |
| first build+frame | 255 | 351 (350–351) | 64 | 185 | 4 | 111 |
| four frames | 255 | 351 (350–351) | 64 | 185 | 4 | 111 |

### v014 telperion cache on (n=3)

| Stage | VmRSS | VmHWM | RssAnon | RssFile | Rust heap live | Rust heap peak |
|---|--:|--:|--:|--:|--:|--:|
| start | 3 | 3 (3–3) | 0 | 2 | 0 | 0 |
| adapter+device | 125 | 125 (125–125) | 24 | 96 | 0 | 0 |
| Renderer::new | 132 | 132 (132–132) | 30 | 96 | 3 | 4 |
| Generator::new | 134 | 134 (134–134) | 32 | 96 | 4 | 4 |
| prepare_programs | 134 | 134 (134–134) | 32 | 96 | 4 | 4 |
| prepare | 209 | 305 (304–306) | 43 | 160 | 4 | 111 |
| submit_prepared | 209 | 305 (304–306) | 43 | 160 | 4 | 111 |
| first build+frame | 209 | 305 (304–306) | 43 | 160 | 4 | 111 |
| four frames | 209 | 305 (304–306) | 43 | 160 | 4 | 111 |
