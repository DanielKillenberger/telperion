# The release: where F4 stands against fn-237 (2026-10-15)

Measured on the owner's desk (RTX 3080, 31 GB), F4 at `7a5b59d5c` plus the R7 format branch, with Chrome, Steam and other sessions running. Raw output is in the session scratchpad, not kept.

## 1. Dependency specs

| Spec | State | Why |
|---|---|---|
| fn-252 ground stop | closed | every task merged; the owner accepted the ground rule (2026-10-14) |
| fn-270 growth continues | closed (task 5 done) | GROWING.md: gate 1,370 passed, npm 168/168, codex SHIP; the owner: "yea it grows now. nice" |
| fn-271 gate in minutes | closed | merged; 25:25 to 18:48, every test kept |
| fn-275 young palm | closed | merged; the drawn rachis moved to fn-283 by host ruling; the palm approved in the species review |
| fn-276 device builds every tree | closed | merged with its sweep and review |
| fn-277 leaf forms by degree | closed | merged |
| fn-281 oak at 80 | closed | merged; its "open after fix A" lines move here (section 3) |
| fn-213, 218, 219, 236, 286 | closed earlier today | the owner's verdicts |
| **fn-257 the site on the device** | **open, not built** | only its two spikes are done; its implementation tasks were never planned. Its commits sit on `fn-257-the-sites-field-grows-only-the-detail`, not on F4 |
| fn-198 F | open | task 9, release readiness (section 4) |
| fn-259 this check | open | below |
| fn-272 the film | open | not this check's |

## 2. fn-237 R4 to R16

| R | State |
|---|---|
| R4 resident tree below ground | not re-measured on today's engine; open |
| R5 field test timeout | superseded by fn-241 (one test per preset); `npm test` 179/179 green on F4 after fn-286 |
| R6 cold-build margin | superseded (`691620d04`) |
| R7 fmt and clippy | done on `fn-237-format`: clippy's two deny-level lints are authored numbers, allowed where they stand; `cargo fmt --all` in one commit, 263 files, no other change. clippy warnings remain (style lints, none deny-level) |
| R8 artifact sizes | **over every ceiling; the owner's decision** (table below) |
| R9 compile time | done (RELEASE-CHECKS.md: load-free guard in the default suite, the cold probe a release check) |
| R10, R12, R13, R15 | done (housekeeping) |
| R11 catalogue pins | confirmed: the four real species' `pins.json` and `suite/identity.rs`'s `PINS` describe the scaffold tree; both tests skip every engine preset, so nothing reads them. Proposal: delete `PINS`, its two tests and `print_pins`; `pins.json` stays a catalogue schema file the species runner and `catalogue-check` require, so its removal or re-pin is a species-runner change, after the release |
| R14 Tint | done (RELEASE-CHECKS.md) |
| R16 film | fn-272, not this check's |

### R8: shipped artifact sizes (`npm run build` on F4)

| Artifact | Bytes | Ceiling | Brotli | 0.1.4's ceiling basis |
|---|--:|--:|--:|--:|
| `telperion.wasm` | 1,727,052 | 1,600,000 | 443,109 | 1,410,330 |
| `telperion-render.wasm` | 3,680,668 | 2,800,000 | 881,709 | 1,985,291 |
| `telperion-field.wasm` | 1,165,252 | 632,580 | 324,744 | 363,305 |
| `telperion.js` | 255,929 | 153,565 | 23,683 | |

CI's size check fails on all four. Crossing a ceiling is the owner's decision (`scripts/artifact-budgets.json`). The field module is fn-242's (post-release); it matters only if the site moves to this release.

## 3. fn-259: the bar

### Harness build, warm, on the RTX 3080 (`build_time`, the harness's `Generator::prepare` resident path)

| Preset | Age | Seed 1 ms | Seed 7 ms | Seed 42 ms | Bar | Device peak MB |
|---|--:|--:|--:|--:|--:|--:|
| ordinary | 40 | 430 | 457 | 481 | 500 | 184 |
| oregon-white-oak | 40 | 243 | 254 | 245 | 500 | 838 |
| norway-spruce | 40 | 447 | 440 | 467 | 500 | 1,595 |
| european-beech | 40 | 168 | 182 | 215 | 500 | 207-312 |
| silver-birch | 40 | 356 | 420 | 398 | 500 | 425-548 |
| date-palm | 40 | 79 | 78 | 79 | 500 | 35 |
| telperion | 82 | 947 | 1,284 | **1,328** | ~1,250 | 2,605-2,665 |
| laurelin | 80 | 855 | 1,158 | 1,217 | ~1,250 | 2,396-2,451 |

Every real species is under 0.5 s at every seed. Telperion at seed 42 is 6 % over "about 1.25 s" and at seed 7 3 % over; the owner set "about", so this is reported, not judged.

### The site's per-tree call (Node, the worker's call: `growField`, the 36-cube grid, query, voxelize; warm median of three after one)

| Species | Seed | 0.1.4 ms | F4 ms | Ratio |
|---|--:|--:|--:|--:|
| oak | 1234 | 258 | 2,607 | 10× |
| oak | 5678 | 302 | 1,857 | 6× |
| oak | 7 | 281 | 1,974 | 7× |
| birch | 1234 | 310 | 1,434 | 5× |
| birch | 5678 | 309 | 800 | 3× |
| birch | 7 | 390 | 1,950 | 5× |
| spruce | 1234 | 215 | 7,021 | 33× |
| spruce | 5678 | 214 | killed at 6 GB | |
| palm | | 17-18 | not reached | |

**The site cannot move to this release.** fn-257, the device field that was to bring it under 0.1.4, is unbuilt. fn-237 R0 names the fallback: "or the part that is worse does not ship (for example the site stays on 0.1.4) until it is."

### The oak's device and CPU trees at 40 (fn-259, found on the way)

Traced with `resident::agree_tests` at `AGREE_AGE=40` on F4 (diagnostics added and removed, not committed):

- **One node:** the device draws a twig of radius 0.0002000003 m that the CPU drops. That is `FINEST` (0.2 mm) itself: the f32 device and the f64 CPU round the same wood to either side of the threshold (fn-281's rule). Its station count follows (47,577 against 47,575), and so does the sweep's one unmatched run. Invisible at every supported view.
- **185 nodes carry different keys:** every one of them stands at the CPU's position within 1e-6 m with the same radii; only its lineage key (and, for 81, its phytomer identity) differs, mostly at relay joins. The 232 "parents differing" are their children pointing at the relabelled parent. Geometry, topology and drawing agree; the labels do not.
- Not fixed: the tie needs a decision on the agreement contract (count a node within float tolerance of `FINEST` as agreeing, or make both executors judge it in one precision), and the relabelling needs its own trace into how a relay's key is drawn on each executor. Neither changes a drawn tree. The release keeps the gate's agreement at age 12.

## 4. Release readiness

- Version: `package.json` 0.1.4. The convention (v0.1.4, `c79bb590e`) bumps it in the release PR and the owner tags.
- No CHANGELOG file; release notes live in the tag and PR.
- `docs/pipeline.md` and `docs/tree-space.md`: current as of 2026-10-12, naming fn-257 as the site's open step.
