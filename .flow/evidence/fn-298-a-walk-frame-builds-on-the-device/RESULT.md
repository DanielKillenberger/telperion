# fn-298 task .3: the film's walk frames, agreement and bytes

Run at `b634869b9` and the task's commits, 2026-10-21. One of the 9 `rounds` frames is one node apart between the CPU and the device; the host ruled on it (R4, below), and its node check is an ignored test naming fn-300.

## R3: the sweep

`crates/telperion-render/src/generation/film_walk_tests.rs` lays out the film's segment table (`release.json`), the frames at 30 fps, and `t = ease((i + 1) / n)` with the smoothstep. Frames are numbered from 1.

| Test | Kind | Frames | Result |
|---|---|--:|---|
| `every_film_walk_frame_builds_on_the_device` | `#[ignore]`, the full sweep | 1,489 (every frame of the six walks at seeds 1, 7 and 42, less fn-299's 44, plus the Laurelin at 20 at each seed) | **1,489 built, 0 refused** (`raw/t6-film.log`) |
| `the_film_walks_build_on_the_device` | ordinary, the stated subset | 48 (each walk's first and last frame at each seed, the 9 `rounds` frames, the Laurelin at 20 at each seed) | all built |
| `fn299_oak_to_palm_frames_build_on_the_device` | `#[ignore = "fn-299: ..."]` | 44 | 44 refused, as expected: frames 106 to 119 at every seed on `FRAMES` ("12 units a cycle on one apex, past the 11"); frames 76 and 77 at seed 42 with "the build passed its sizes at cycle 36: UnitRounds (0x1)" |

R1's other rows (beech to spruce at t 0.00037, 0.01 and 0.1, seeds 7 and 1; the Laurelin at 20, seed 7) are covered by `cpu_builds_tests`' own ordinary tests, and the 0.2160 frame by `fn298_the_beech_to_spruce_at_0_216_builds_on_the_device`. The subset also holds that frame as beech to spruce, seed 7, frame 27.

**Why the sweep is ignored** (measured): the full sweep ran inside a 1,052 s test binary run, alongside the subset, the counts test and fn-299's frames. The gate's last run took about 37 minutes (`raw/gate-task2-rerun.log`). From the survey's rate of about 0.55 s a frame, the subset takes about 30 s (inferred).

**Frames 76 and 77 at seed 42** (inferred, not traced): R2.md recorded their living units rounds used at 6. The walk's two ends use 3 (the oak) and 1 (the palm), so the frames fall outside the range their ends span, as fn-299's `FRAMES` frames do. The blended row behind them was not traced to the one fn-299 names: the shared cause is inferred, not traced. The host accepted the exclusion on that footing (2026-10-21); it un-ignores fn-299's frames on F4 once fn-299 and fn-298 are both there, which settles it.

## R4: agreement (narrowed: node counts equal, instances within 1)

`the_rounds_frames_have_the_cpus_counts`, ordinary, checks the 9 `rounds` frames' instances and 8 of them's node counts; `fn300_the_spruce_to_birch_frame_56_has_the_cpus_nodes`, ignored under fn-300, checks the ninth's nodes. Each frame is built by the CPU (`mesh::build`, `executor::grow`) and on the device (`prepare`, `tree::drawn`), and matched with `cpu_builds_tests::apart`.

| Frame | Instances, device / CPU | Nodes, device / CPU | Matched |
|---|---|---|--:|
| beech to spruce, s1, f41 | 124,209 / 124,209 | 276,586 / 276,586 | 276,586 |
| beech to spruce, s7, f27 (t 0.2160) | 96,524 / 96,524 | 275,557 / 275,557 | 275,557 |
| spruce to birch, s1, f63 | 149,705 / 149,705 | 311,716 / 311,716 | 311,691 |
| spruce to birch, s7, f63 | 192,650 / 192,650 | 388,725 / 388,725 | 388,683 |
| **spruce to birch, s42, f56** | 175,291 / 175,291 | **323,513 / 323,512** | 323,503 |
| spruce to birch, s42, f59 | 168,407 / 168,408 | 314,733 / 314,733 | 314,726 |
| oak to palm, s1, f36 | 167 / 167 | 3,789 / 3,789 | 3,789 |
| oak to palm, s1, f37 | 197 / 197 | 5,793 / 5,793 | 5,789 |
| oak to palm, s1, f48 | 111 / 111 | 948 / 948 | 947 |

Every frame's instances are within 1. On 8 of 9 frames the node counts are equal. **Spruce to birch, seed 42, frame 56 has one node more on the device.**

**The limits do not cause it** (measured): with every stepped limit at its bound (`R2_FULL`, scratch `raw/t5-scratch-all.patch`, applied and reverted), all 9 rows come out identical (`raw/t6-counts-full.log`). R2.md records a gap of the same kind on the base, with no limit involved: the spruce alone at 40 is one node off (966,171 / 966,172). The frame itself is refused on the base (`HierGens`), so the base cannot be compared on it. So this is a CPU-device difference on walk frames, the class fn-300 owns, and not a refusal.

**Ruled (host, 2026-10-21):** the node check stays exact, since topology agreement is meant to be exact (fn-300's draft contract). This frame's node check moved to its own ignored test naming fn-300. The other 8 frames keep the exact node check, and all 9 keep the instance check within 1. The base precedent is the one-node gap R2.md records on the spruce alone at 40 (966,171 device / 966,172 CPU, base and change alike). The host adds the frame to fn-300's R1 on F4 after the merge. After the split, the focused run is green on the ordinary tests, and the fn-300 test is red as expected (`raw/t7-focused-2.log`).

## R4: bytes

`DIGEST_SEEDS=1,7,42` `resident::digest_tests::digest` ran on this branch and on the spec's base `7422b110b`. The base was exported with `git archive` under `target/fn298-base/`, which has its own `CARGO_TARGET_DIR`. **All 24 lines are identical.** sha256 of the 24 lines, both sides: `3c8549fe58e70f5a08411604627bb79ad761dd4cffd4b0c03473e5f7e7c41399`. Logs: `raw/t6-digest-base.log`, `raw/t6-digest-change.log`.

```
ordinary 1: leaves c02fe7a3a8a85be1 points 93976 07180e7c1238d783 clusters 8283 6c3fd30e273cce78 sections cbf29ce484222325 demand 3d7b807e90145b04
ordinary 7: leaves 7b5e15a72345c4b9 points 141692 656f9bb762cf477e clusters 12374 b87c04ad0208df3b sections cbf29ce484222325 demand be2e3ce02eee767b
ordinary 42: leaves 1c05ded0ca70a508 points 165579 a5590606bae538cc clusters 14523 44c99f0050c9ac4b sections cbf29ce484222325 demand 0f6d33f2446182d4
oregon-white-oak 1: leaves 7af73ed6b6614c50 points 301765 b0cecb658df306ea clusters 27157 22c8c9c3defc3bf2 sections cbf29ce484222325 demand 746e3bff8b53f92f
oregon-white-oak 7: leaves eeaf1214d260cbfc points 421025 2c99346cfa4b97af clusters 38410 17eb8e19786e021e sections cbf29ce484222325 demand 85a3cc2a042ea18b
oregon-white-oak 42: leaves ab092d0efdab19a9 points 312370 e2657d7cf89d29ac clusters 27879 ea49c91f8a1ae75b sections cbf29ce484222325 demand 80d6f8bd270afac9
norway-spruce 1: leaves 971121193207f750 points 1195455 c8091b23f6569926 clusters 206416 fd6e6a2ba837121f sections cbf29ce484222325 demand d6d18dee17bda8ce
norway-spruce 7: leaves 4f0bb8d39f911d8c points 1157978 e36c03519a4c7a4c clusters 197653 d2e30837b6ea9c88 sections cbf29ce484222325 demand ad47d2456373c585
norway-spruce 42: leaves 01b261b59836943d points 1206281 7f326bef63512c88 clusters 206593 e9d0aee8a4db8f1b sections cbf29ce484222325 demand 99b356f7823a9bdb
european-beech 1: leaves 2476c01b7528cb86 points 124065 1015e11ca76339e9 clusters 9945 64f3df6181ff2482 sections cbf29ce484222325 demand 1e3886162808ef2d
european-beech 7: leaves ecbb718fefa03d7b points 222705 0ba062bb57eb1d70 clusters 18256 83d5e923ba6e8cdb sections cbf29ce484222325 demand 7e0763c346d74179
european-beech 42: leaves eb14feaaebc6c056 points 272892 2f53151087b21db0 clusters 22416 b5de8d1813a38ad1 sections cbf29ce484222325 demand fa7e601107af97a6
silver-birch 1: leaves 43392fbcccce53b6 points 267524 d9a3d34f99ed70cc clusters 26389 abebb6a7b4285dee sections cbf29ce484222325 demand d063d0a5f9a5974e
silver-birch 7: leaves f8ef97afff18e3ab points 392261 30de28d33ca995f3 clusters 39012 d02bf93ce36b2f97 sections cbf29ce484222325 demand 7b4297491f0e24a8
silver-birch 42: leaves 032f18dbee8b8729 points 332297 071e12d736fdaa92 clusters 33155 04a9e8b8c279d6e7 sections cbf29ce484222325 demand 1b79d786b75a2ec6
date-palm 1: leaves 3eeecaf46010a147 points 1249 90ab73a62b329fcd clusters 272 0605078dcfa774b6 sections c088d90a3d18085e demand c55a5cb56660681e
date-palm 7: leaves 034d95019e4d6a84 points 1249 8d21da115d8df61e clusters 272 ed22240f73f16db3 sections 7ec13a13cf49c6fd demand 63987e72e6302ce2
date-palm 42: leaves bc07aaf0200bc5aa points 1249 ed3ef7ef4f39aab6 clusters 272 fd79050447726545 sections 63076d4939dfe79d demand 5d1b708978a43282
telperion 1: leaves 00f0791309cf1e65 points 2131692 d6e0f3307fb810fe clusters 158540 595c090fa2decbef sections cbf29ce484222325 demand 79d39de092982ed3
telperion 7: leaves 99fd62f391c3dcf9 points 3606433 7372daac2b79da34 clusters 268391 3f57562020225928 sections cbf29ce484222325 demand 3a43f6d5939555de
telperion 42: leaves 0fea932c7cc42ae4 points 3606863 35a0a10486b9a083 clusters 268771 7eb4158f1d834db6 sections cbf29ce484222325 demand 9d1c16ca9ccd9dd1
laurelin 1: leaves a499a7f5f2e9c3cc points 1996298 7aa3a40a1dc17d48 clusters 148507 21e984a8906fa11a sections cbf29ce484222325 demand 0d40673a325e3cb3
laurelin 7: leaves 4caf51918188f241 points 3358374 be77ce4bdfb297c7 clusters 250258 5a0dfbc75f695059 sections cbf29ce484222325 demand dc73fa84925910ce
laurelin 42: leaves f076ea1addc0e87f points 3375803 cf2d1d6eafc60590 clusters 252436 e485b6c39b94eb30 sections cbf29ce484222325 demand 6fc9dda5f5612d6f
```

## R5: gate and `npm test`

- Gate, `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, run once at `bc0066e07` (12:35 to 13:11, no wait): **1,410 passed, 0 failed, 115 ignored** across 146 suites (`raw/gate-task3.log`).
- `telperion-heavy 8 --threads 2 -- npm test` at the same commit: **21 files, 175 tests passed** (`raw/npm-test-task3.log`).
- Codex impl review (one correctness reviewer, `gpt-6.1-sol`): **SHIP** in round 1, with no findings.
