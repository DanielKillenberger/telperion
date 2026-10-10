# R5 timings: before (base `59e627e7`, contract A) and after (f32)

All runs were on the same machine on 2026-10-10. Load was 0.9 to 4, and before and after runs were interleaved: each preset ran on one side, then the other, three times. Every run was pinned to cores 0 to 3. The scripts are in the session scratchpad (`sweep2.sh`, `cold.sh`).

## Cold `grow` shader compile (beech at 20 on the browser route, driver cache off, GPU lock)

| Run | Before `grow` (ms) | After `grow` (ms) | Before `girths` (ms) | After `girths` (ms) | Before built (s) | After built (s) |
|---|---|---|---|---|---|---|
| 1 | 14,538 | 11,711 | 1,433 | 58 | 19.7 | 15.1 |
| 2 | 15,771 | 12,916 | 1,769 | 57 | 21.6 | 16.4 |
| 3 | 14,590 | 10,829 | 1,398 | 54 | 19.6 | 13.9 |

- `grow` compiles about 21% faster (median 14.6 s to 11.7 s).
- `girths` drops from 1.4 s to 0.06 s.
- The whole browser-route build drops from 19.7 s to 15.1 s.

The `grow` program is one shader for every preset that grows on the GPU (beech, spruce, birch, Telperion, Laurelin). The oak, the ordinary tree and the palm do not compile it for growth (R4a; deferred to fn-256).

## Wasm, full core build (`telperion_wasm.wasm`, structure output), median of 3

Node counts are identical before and after, and every build returns status 0.

| Preset | Before (ms) | After (ms) | Change |
|---|---|---|---|
| european-beech | 623 | 260 | -58% |
| oregon-white-oak | 4,835 | 1,765 | -63% |
| norway-spruce | 9,548 | 2,009 | -79% |
| date-palm | 22 | 19 | -13% |
| silver-birch | 2,204 | 670 | -70% |
| ordinary | 2,869 | 924 | -68% |
| laurelin | 6,602 | 2,710 | -59% |
| telperion | 9,016 | 3,776 | -58% |

## Wasm, site field (`telperion_field.wasm`, n 36, cut 1024), median over seeds 1234 and 5678 and 3 repeats

| Preset | Grow before (ms) | Grow after (ms) | Change | Query before (ms) | Query after (ms) |
|---|---|---|---|---|---|
| european-beech | 63 | 52 | -16% | 13.6 | 12.9 |
| oregon-white-oak | 950 | 409 | -57% | 87.5 | 82.4 |
| norway-spruce | 297 | 158 | -47% | 32.1 | 32.6 |
| date-palm | 23 | 23 | -1% | 15.6 | 15.8 |
| silver-birch | 804 | 692 | -14% | 93.2 | 92.0 |
| ordinary | 11,001 | 11,147 | +1% | 13.9 | 13.4 |
| laurelin | 321 | 221 | -31% | 109.8 | 112.0 |
| telperion | 480 | 311 | -35% | 197.7 | 196.6 |

Every preset's voxel counts are identical before and after.

## Artifact sizes, built the way the package builds them
| Artifact | Before (bytes) | After (bytes) | Change |
|---|---|---|---|
| `telperion.wasm` | 1,957,570 | 1,763,891 | -9.9% |
| `telperion-field.wasm` (stripped) | 1,370,715 | 1,194,585 | -12.9% |
| `telperion_render.wasm` (before bindgen) | 4,466,742 | 4,218,179 | -5.6% |

All three shrink. Their ceilings in `scripts/artifact-budgets.json` (1.6, 0.63 and 2.8 MB) were already crossed at the base. The tree-space engine grew them on the F branches, so the ceilings are a release decision for fn-237, not one this spec makes. In the field sweep, the after module still carried its symbol names (1,629,916 bytes), which does not change its timing.
