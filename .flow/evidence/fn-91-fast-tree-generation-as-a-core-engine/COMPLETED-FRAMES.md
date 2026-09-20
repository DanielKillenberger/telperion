> Historical raw-output references: see the [archive and recovery instructions](README.md).

# Completed-frame browser comparison

The CPU candidate improves measured end-to-end delivery but does not meet the 10x target. The benchmark now waits for the actual renderer GPU queue after submission, including pending uploads and drawing. It does not measure the display compositor or the instant pixels become visible.

Both runs use Chromium, the recorded RTX 3080 desktop, a 1280×720 canvas and the renderer hero camera. Each fixture has one first build and five warm builds with newly generated geometry; p50 and maximum below describe those five observations, not a reliable population p95. The workstation is shared.

| Fixture | Original p50 / max (ms) | CPU candidate p50 / max (ms) | Speedup | 10x target (ms) |
|---|---:|---:|---:|---:|
| oregon-white-oak / 1 | 1570.0 / 1620.3 | 1001.0 / 1029.1 | 1.57x | 157.0 |
| oregon-white-oak / 7 | 1862.5 / 1903.9 | 1168.0 / 1185.8 | 1.59x | 186.2 |
| norway-spruce / 1 | 7628.6 / 7703.4 | 6137.0 / 6165.5 | 1.24x | 762.9 |
| norway-spruce / 7 | 7384.0 / 7464.7 | 5831.7 / 5850.0 | 1.27x | 738.4 |

Submitted counts and bounds agree exactly across all samples and revisions. Rendering bytes were not read back for this timing run; native output fingerprint evidence remains in CPU-CANDIDATE.md. The overall timing change is measured, but timing variation within the unchanged rendering stage is not attributed to the CPU algorithm.

The original generator was built in an isolated checkout at b63d38c3. The candidate uses the already built CPU-patch Wasm corresponding to f2c7e81c; later uncompiled GPU source work cannot affect it. Both binaries have SHA-256 identifiers in completed-frame-comparison.json. That file also records module/renderer initialization and first delivery separately for every fixture. Vite compilation/cache behavior makes these harness initialization observations unsuitable as a production website cold-start qualification.

The new protocol is opt-in: GENERATION_COMPLETED=1. It captures exactly one renderer device during creation, restores the browser prototype afterward, uses the hero camera and fences every frame. Default submission-only measurement remains available. Phone hardware, production page startup, native completed rendering and whole CPU/GPU peak memory remain separate gaps. The 100 ms stretch goal remains unmet on every fixture.
