# Phase 5 census: M1 to M4, the floor and the faint tops, 12 Nov 2026

Measurement only. Worktree `.worktrees/census-p5`, detached at `48edda0a7` (fn-356's result: the rule reverted whole), its own `target/`. Nothing committed; no product behaviour changed. The settled oak at 80 and 120 on seed 1002 repeats fn-356's head census to the unit (held 4,683,475 and 9,175,834; under 0.2 mm 1,161,509 and 3,888,756), so this checkout grows the tree fn-356 called "head".

Tags: [read] read from the grower's stores by the reader below; [replicated] recomputed from the grower's stores by the same arithmetic as the product code, checked against the product's own totals; [derived] arithmetic on read numbers.

## What was built (uncommitted, worktree only)

- `crates/telperion-space/src/grow/cycle/phase5.rs`: the reader, `Paused::phase5()` (one `pub fn` added in `paused.rs`, one `mod phase5;` in `cycle.rs`). Reads the grower's private stores after `grow_to(age)`, before any settle.
- `crates/telperion-space/src/closed_form.rs`: `expected_sub_counts(species, age, &[(pa, m)])`, `Walk::counts` for a lateral bud of PA `pa` grown `m` cycles with `Carried::default()` and no first-unit law, under `Windows::new(species, age)`, exactly as `expected_counts` reads a substructure.
- `crates/telperion-core/examples/p5census.rs`: drives it. Species and request from `executor::growth(&family)` (the pipeline's own), `Paused::begin(.., Retiring::AtAge, census=false)`; a counting global allocator (bytes requested, its most since the last stage) and `/proc/self/status` VmHWM.
- `.flow/evidence/census-p5/raw/{run.sh,batch.sh,tables.py}`; logs `census-*.log`, `pipeline-*.log` beside them.

## Commands

```
# build (queued)
telperion-heavy 8 --threads 8 -- cargo test --profile ci -p telperion-space --no-run
# each run its own heavy job, one at a time (batch.sh -> run.sh builds the example, then runs it)
bash .flow/evidence/census-p5/raw/batch.sh "census oregon-white-oak 80 1002" "census oregon-white-oak 120 1002" \
     "pipeline oregon-white-oak 80 1002" "pipeline oregon-white-oak 120 1002"
bash .flow/evidence/census-p5/raw/batch.sh "census european-beech 40 1002" "census norway-spruce 40 1002" \
     "pipeline european-beech 40 1002" "pipeline norway-spruce 40 1002"
#   inside: telperion-heavy 16 --threads 8 -- bash run.sh <mode> <preset> <age> <seed>
#   run.sh: cargo build --profile ci -p telperion-core --example p5census; target/ci/examples/p5census <mode> <preset> <age> <seed>
python3 -I .flow/evidence/census-p5/raw/tables.py .flow/evidence/census-p5/raw/census-<tree>.log
```

## Definitions: exactly what was read

- **Held**: in `Grower::axes` at the end of the last cycle. This includes **retired-held** axes: retired by `retire.rs` (fate `Root` or `Inside`), their phytomers already dropped, their axis, draws, pencil and per-axis records still held until `compact` lets them go (it runs only once the loose ones reach max(a quarter of the axes, 4,096)).
- **Faintness of an axis, f**: its rough-layout pencil's `base_scale * running` [read]. `base_scale` = bearer node's unsized scale x made presences x dominance share (`draw.rs`); `running` = the product of its laid units' `survive * persist`. It leaves out `kept` and the unit's light size. Every held axis had a pencil (the "no pencil" bin is empty everywhere).
- **Faintness of a phytomer (own)**: its rough-layout `scale`, which the layout leaves unsized: base_scale x running at its unit x its node's presence x its unit's grown share [read]. M1's main tables bin a phytomer by its axis's f; a second table bins it by its own scale.
- **Bins**: f < 0.05, 0.05 <= f < 0.2, f >= 0.2.
- **Open / closed**: `Working::closed(i)` as the last cycle left it (open: on the way from the root to a living apex or a sleeping bud) [read]. Retired-held is its own column.
- **Living tip**: an entry of `Grower::live` [read]; all are open.
- **The grower's leaf**: the last cycle's deposits, `girth::leaf(state, p, age)` = scale x size x leaf_area x leaf-years kept, over this year's phytomers, plus (spruce only, `leaf_years > 1`) older phytomers of open axes still in leaf [replicated; count and m² equal `Grower::leaves` exactly in every run, see CHECK lines].
- **M4 zero quanta**: per deposit, `Field::quantised_with`'s arithmetic: area as f32, each corner's trilinear f32 weight from `x as f32 / 0.5`, `round(area * w / 2^-20)`; a deposit is "zero" when all eight corners round to 0 [replicated]. "Net area lost" is the deposit's area minus the area its quanta carry (signed).
- **PA names** are the shared chain's (`chain.rs`), which the oak, beech and spruce presets use as their kind names.
- **M3 roots**: lateral axes (`Origin::Lateral`, not woken) with f in the band, not retired-held. Woken laterals and retired-held laterals are counted apart (lines `M3WOKEN`, `M3RETIRED`).
- **M3 realised subtree** (the root and every held descendant, laterals, continuations and relays): `records` = held phytomer records; `scale` = sum of unsized phytomer scale (presence-weighted count, light-free); `leaf` = sum of `girth::leaf` (with the light size); `wood` = sum of internode(PA) x unsized scale (metres, light-free).
- **M3 expectation**, for a bud of PA k born in cycle b, grown m = age - b cycles: `N` = the closed form's expected phytomers (all PAs, all cycles of `expected_sub_counts`); `L` = sum over its table of count x leaf_area(PA) x leaf-years kept at the age; `W` = exp(`expected_log_lengths`[m][k]) metres. The closed form counts the phytomers **grown**, shed ones included, and is light-neutral (no shade hazard, no allotment size), so `scale` and `wood` compare like with like except shedding; `leaf` additionally carries the light size. Ratios are given over f x expectation (as asked) and over b x expectation, b = the root's `base_scale` (the root's own survivals sit inside the expectation, so b is the like-for-like scale). p10/p50/p90 are of per-root ratios; "sum ratio" = sum real / sum expected.
- **The floor**: settled tree (`Paused::settle()`, from a copy) phytomers with radius x supernatural scale < 2e-4 m, as the drawing's `FINEST` [read on the settled structure; the scale is 1 for all four trees].
- **Tops**: lateral axes with f under the band whose bearer node (the parent's phytomer at `node`, its unsized scale) stands at or over the band, not retired-held. "Axes held" and "phytomers held" are those under it, itself included, retired-held axes not counted [read]. `TOPSOTHER` counts continuations and relays that fall under a band below a bearer axis over it.
- **M2**: `Paused::footprint()` at the end of growth [read, `footprint.rs`'s own count: length or capacity x record size]; heap = the counting allocator's bytes requested; VmHWM = peak resident. `census` mode reads growth, then the reader, then `settle()` from a copy. `pipeline` mode is the product path: `executor::grow` (growth, the consuming settle, conversion), `expansion()`, `mesh()`, a separate process.


## Summary (seed 1002, at the last cycle)

**M1. The faint population** [read; shares derived]

| | Oak 80 | Oak 120 | Beech 40 | Spruce 40 |
|---|---|---|---|---|
| Held phytomers | 5,693,800 | 11,558,758 | 571,516 | 1,167,938 |
| ... axis f < 0.05 | 1,197,236 (21.0%) | 4,445,952 (38.5%) | 101,915 (17.8%) | 5,581 (0.5%) |
| ... axis f 0.05 to 0.2 | 603,875 (10.6%) | 1,446,117 (12.5%) | 14,109 (2.5%) | 28,850 (2.5%) |
| ... own scale < 0.05 | 21.9% | 39.4% | 17.8% | 0.6% |
| Held axes | 2,237,329 | 5,081,676 | 65,136 | 705,882 |
| ... retired-held (no phytomers) | 1,200,290 (53.6%) | 2,940,105 (57.9%) | 19,972 (30.7%) | 393,845 (55.8%) |
| ... not retired, f < 0.05 | 281,969 | 1,007,927 | 7,033 | 1,844 |
| ... not retired, f 0.05 to 0.2 | 123,758 | 259,737 | 1,007 | 8,102 |
| Living tips | 374,527 | 719,340 | 33,230 | 106,843 |
| ... f < 0.05 | 97,118 (25.9%) | 328,891 (45.7%) | 5,027 (15.1%) | 537 (0.5%) |
| Grower's leaf, m² | 1,017.7 | 1,307.6 | 113.2 | 160.1 |
| ... share on f < 0.05 | 0.32% | 0.83% | 0.33% | 0.03% |
| ... share on f 0.05 to 0.2 | 1.32% | 1.81% | 0.43% | 0.42% |

On the oak the faint phytomers (f < 0.05) are open wood more than closed: 809k open and 389k closed at 80; 2.89M open and 1.55M closed at 120. By PA at 120: branch 1.44M open + 0.30M closed, twig 0.89M + 0.59M, short 0.52M + 0.63M. Per-PA tables are below.

**Tops: faint laterals on a bearer node that is not faint** [read]

| | Oak 80 | Oak 120 | Beech 40 | Spruce 40 |
|---|---|---|---|---|
| Tops f < 0.05 (bearer >= 0.05) | 4,396 | 8,711 | 143 | 589 |
| ... axes held under them | 197,200 | 773,923 | 7,072 | 976 |
| ... phytomers held under them | 861,373 | 3,345,732 | 102,202 | 3,252 |
| ... of which under the oak's 2 limb tops | 774,378 | 3,011,124 | | |
| Tops f < 0.2 (bearer >= 0.2) | 7,742 | 11,936 | 467 | 2,070 |
| ... phytomers held under them | 1,279,155 | 4,728,624 | 120,169 | 26,448 |

- **Oak 120:** 8,711 records would stand for 3.35M phytomers and 774k axes. The 3.35M overlaps the 4.45M faint by axis f but is not the same set, since a phytomer under a top is not always itself under 0.05.
- **The two limb tops:** two faint limbs hold 3.01M of the 3.35M.

**The 0.2 mm floor** [read, settled]:
- **Oak 80:** 1,161,509 of 4,683,475 (24.8%).
- **Oak 120:** 3,888,756 of 9,175,834 (42.4%): branch 43.8%, shoot 43.7%, twig 50.1%, short 37.2%.
- **Beech 40:** 87,384 of 526,702 (16.6%).
- **Spruce 40:** 26,339 of 1,043,264 (2.5%).

**M2. Memory** [read; bytes per record derived from the footprint's count]

| | Oak 80 | Oak 120 | Beech 40 | Spruce 40 |
|---|---|---|---|---|
| Grower footprint at end of growth, GiB | 3.197 | 7.106 | 0.174 | 1.026 |
| ... per held phytomer, bytes | 603 | 660 | 327 | 943 |
| ... per held axis, bytes | 1,534 | 1,502 | 2,865 | 1,560 |
| ... phytomer stores (`axes.phytomers`, `draws.nodes`) per phytomer | 198 | 199 | 205 | 173 |
| ... every other store per held axis | 1,030 | 1,050 | 1,067 | 1,274 |
| Census process: VmHWM end of growth / after settle from a copy, GiB | 3.30 / 5.05 | 7.31 / 11.05 | 0.19 / 0.31 | 0.90 / 1.40 |
| Pipeline process: VmHWM after grow / expansion / mesh, GiB | 3.37 / 3.37 / **4.49** | **7.31** / 7.31 / 7.31 | 0.23 / 0.23 / **0.57** | 0.96 / 0.96 / **1.58** |
| Pipeline heap most during grow / mesh, GiB | 3.73 / 3.04 | 8.31 / 4.52 | 0.19 / 0.39 | 1.19 / 1.20 |
| Pipeline seconds grow / mesh | 26.8 / 2.0 | 97.2 / 2.8 | 0.7 / 0.2 | 3.9 / 4.7 |

Largest stores at 120, MiB: phytomers 2,067; axes 1,470; pencils 1,318; draws 912; draws.units 155; successor 152.

- **The peak's timing, oak 120:** the product's peak (7.31 GiB) is set inside `executor::grow`. The census process reaches the same 7.31 GiB at the end of growth, before any settle. Neither the consuming settle nor the mesh raises it. At 120 the peak is the grower's steady state at the end of growth.
- **The peak's timing, oak 80, beech 40 and spruce 40:** the mesh sets the peak (4.49, 0.57 and 1.58 GiB), above the end of growth (3.30, 0.19 and 0.90).
- **The settle from a copy** (`Paused::settle`, what a paused or census grower uses) adds 3.7 GiB at 120. The pipeline's own consuming settle does not.
- **Retired-held axes:** 58% of the oak's held axes at 120 are retired placeholders with no phytomers. Each pays the per-axis stores until compaction lets it go. At about 1,050 bytes each that would be about 2.9 GiB of the 7.1 GiB [inferred from the per-axis average; not counted by store].
- **Per settled phytomer** (the drawn tree's), the product's peak is 773 bytes at 80 (3.37 GiB / 4.68M) and 855 bytes at 120 [derived].

**M3. Faint lateral roots against the closed form** [read; expectation computed by the closed form]. Oak 120, f < 0.05, sum ratios (sum real / sum expected):

| PA | roots | records / fN | scale / bN | leaf / bL | wood / bW |
|---|---|---|---|---|---|
| limb | 20 | 1.82 | 0.041 | 0.082 | 0.076 |
| bough | 190 | 2.06 | 0.042 | 0.079 | 0.080 |
| branch | 32,810 | 8.05 | 0.101 | 0.115 | 0.181 |
| twig | 161,126 | 14.9 | 0.275 | 0.113 | 0.361 |
| short | 727,750 | 15.8 | 0.316 | 0.106 | 0.316 |

- **Against the expectation:** the realised presence-weighted subtree (scale, wood) holds 4 to 36% of base x the closed form's expected grown subtree, and its leaf 8 to 12%.
- **Why the gap is not split:** the closed form counts shed wood and no light. The gap is therefore shedding, retirement, the shade hazard and the allotment together, and this census does not separate them.
- **Records:** against f x the expected count, records are 2 to 16. A faint subtree holds many more phytomer records than its presence-weighted expectation.
- **The per-root spread is wide:** twig at 120 reads records/fN p10 0, p50 22.4, p90 749.
- **Rows with p10 = 0:** these are roots holding no phytomer record: a bud whose apex ended at once, or one whose wood retired below it while the root stays open (not distinguished).
- The full per-PA tables for both bands and all four trees, with the f-denominator rows, are below.

**M4. Leaf deposits that round to nothing** [replicated]

| | Oak 80 | Oak 120 | Beech 40 | Spruce 40 |
|---|---|---|---|---|
| f < 0.05: zero deposits, by count | 19,737 of 189,027 (10.4%) | 112,988 of 627,922 (18.0%) | 239 of 16,732 (1.4%) | 61 of 1,625 (3.8%) |
| ... by area | 0.26% of 3.25 m² | 0.36% of 10.9 m² | 0.05% of 0.37 m² | 0.09% of 0.047 m² |
| f 0.05 to 0.2, by count | 78 (0.08%) | 347 (0.18%) | 0 | 0 |
| f >= 0.2, by count | 25 of 615,611 | 46 of 829,684 | 1 of 86,820 | 27 of 398,971 |
| Net area lost to rounding, all bins, m² | 0.076 | 0.204 | 0.0006 | 0.013 |

- **Within the faintest bin:** zero deposits are a tenth to a fifth of its deposits by count, and under 0.4% of its area.
- **Across the whole tree:** they are 0.003% of the oak's leaf area at 120.

**Limits**

- **M3's closed form:** a bud of the lateral's PA, `Carried::default()` and no woken first unit, under the windows of the tree's age. Woken laterals are left out (oak 120: 245 faint woken laterals).
- **Retired descendants:** these are gone from the realised sums, since retirement is not undone for this census. Retired-held roots are excluded from M3 and the tops (oak 120: 1,833,366 faint retired-held laterals).
- **Faintness bins:** M1 and M4 read the axis's f at the end of the last cycle, not at each phytomer's growth. The own-scale table is the per-phytomer alternative.
- **Not run:** one seed only, and no M5.

## Full tables by tree

### RUN census oregon-white-oak 80 1002

```
FOOT counted_gib 3.1967 axes_open_or_kept 1037039 retired_roots 789930 inside_retired 410360
HEAD age 80 axes 2237329 pencils 2237329 live 374527 leaves 900795 lasting false
CHECK deposits replicated 900795 grower 900795 area replicated 1017.6647 grower 1017.6647
TOPSOTHER 0 carried-on axes turning faint on a bearer not faint: 24
TOPSOTHER 1 carried-on axes turning faint on a bearer not faint: 68
M3WOKEN faint woken laterals left out: 80
SCALE 1
exit 0 wall_s 28
```

**M1. Held phytomers by their axis's faintness (PA x bin x open/closed/retired-held)**

| PA | <0.05 open | <0.05 closed | <0.05 retired-held | 0.05-0.2 open | 0.05-0.2 closed | 0.05-0.2 retired-held | >=0.2 open | >=0.2 closed | >=0.2 retired-held | no pencil | total |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 trunk | 0 | 0 | 0 | 0 | 0 | 0 | 92 | 0 | 0 | 0 | 92 |
| 3 low_fork | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 0 | 0 | 0 | 14 |
| 4 leader | 0 | 0 | 0 | 0 | 0 | 0 | 41 | 0 | 0 | 0 | 41 |
| 6 limb | 1,571 | 0 | 0 | 583 | 0 | 0 | 7,299 | 118 | 0 | 0 | 9,571 |
| 7 bough | 7,689 | 0 | 0 | 3,377 | 0 | 0 | 40,051 | 1,002 | 0 | 0 | 52,119 |
| 8 sprig | 0 | 0 | 0 | 0 | 0 | 0 | 157 | 16 | 0 | 0 | 173 |
| 9 branch | 315,880 | 43,369 | 0 | 132,868 | 18,582 | 0 | 688,212 | 139,880 | 0 | 0 | 1,338,791 |
| 12 shoot | 2,875 | 7,379 | 0 | 1,576 | 3,358 | 0 | 14,128 | 31,450 | 0 | 0 | 60,766 |
| 13 twig | 288,539 | 138,140 | 0 | 131,186 | 64,984 | 0 | 693,562 | 364,398 | 0 | 0 | 1,680,809 |
| 14 short | 192,162 | 199,632 | 0 | 117,112 | 130,249 | 0 | 868,319 | 1,043,950 | 0 | 0 | 2,551,424 |
| all | 808,716 | 388,520 | 0 | 386,702 | 217,173 | 0 | 2,311,875 | 1,580,814 | 0 | 0 | 5,693,800 |

**M1. Held axes (PA x bin x open/closed/retired-held)**

| PA | <0.05 open | <0.05 closed | <0.05 retired-held | 0.05-0.2 open | 0.05-0.2 closed | 0.05-0.2 retired-held | >=0.2 open | >=0.2 closed | >=0.2 retired-held | no pencil | total |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 trunk | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 3 low_fork | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 4 leader | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 6 limb | 45 | 0 | 0 | 16 | 0 | 0 | 232 | 3 | 1 | 0 | 297 |
| 7 bough | 201 | 0 | 0 | 86 | 0 | 0 | 1,009 | 33 | 24 | 0 | 1,353 |
| 8 sprig | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 1 | 34 | 0 | 39 |
| 9 branch | 17,753 | 2,457 | 1,939 | 7,270 | 1,132 | 867 | 29,704 | 6,435 | 7,366 | 0 | 74,923 |
| 12 shoot | 757 | 2,403 | 349 | 344 | 1,057 | 147 | 2,147 | 5,814 | 924 | 0 | 13,942 |
| 13 twig | 31,476 | 12,441 | 61,271 | 12,958 | 5,227 | 25,028 | 62,427 | 25,283 | 107,024 | 0 | 343,135 |
| 14 short | 118,984 | 95,452 | 270,755 | 53,026 | 42,642 | 121,946 | 279,850 | 218,367 | 602,615 | 0 | 1,803,637 |
| all | 169,216 | 112,753 | 334,314 | 73,700 | 50,058 | 147,988 | 375,376 | 255,936 | 717,988 | 0 | 2,237,329 |

**M1. Held phytomers by their own rough-layout scale (status summed)**

| PA | <0.05 | 0.05-0.2 | >=0.2 |
|---|---|---|---|
| 2 trunk | 0 | 0 | 92 |
| 3 low_fork | 0 | 0 | 14 |
| 4 leader | 0 | 0 | 41 |
| 6 limb | 1,575 | 597 | 7,399 |
| 7 bough | 7,835 | 3,415 | 40,869 |
| 8 sprig | 0 | 0 | 173 |
| 9 branch | 408,015 | 164,573 | 766,203 |
| 12 shoot | 10,161 | 4,909 | 45,696 |
| 13 twig | 425,971 | 195,886 | 1,058,952 |
| 14 short | 391,989 | 248,260 | 1,911,175 |
| all | 1,245,546 | 617,640 | 3,830,614 |

**M1. Living tips by axis faintness** (by status: {'open': 374527})

| PA | <0.05 | 0.05-0.2 | >=0.2 | no pencil | summed f |
|---|---|---|---|---|---|
| 6 limb | 8 | 2 | 37 | 0 | 34.3 |
| 7 bough | 57 | 22 | 289 | 0 | 258 |
| 9 branch | 4,294 | 1,677 | 9,384 | 0 | 7,761 |
| 12 shoot | 438 | 200 | 1,225 | 0 | 1,007 |
| 13 twig | 20,565 | 8,255 | 38,650 | 0 | 30,973 |
| 14 short | 71,756 | 33,214 | 184,454 | 0 | 150,765 |
| all | 97,118 | 43,370 | 234,039 | 0 | 190,799 |

**M1. The grower's leaf (last cycle's deposits), m², and share of the whole**

| PA | m² | share | share <0.05 | share 0.05-0.2 | share >=0.2 | share no pencil |
|---|---|---|---|---|---|---|
| 6 limb | 0.364 | 0.000357 | 1.76e-06 | 4.95e-07 | 0.000355 | 0 |
| 7 bough | 2.64 | 0.0026 | 9.5e-06 | 7.03e-06 | 0.00258 | 0 |
| 9 branch | 67.8 | 0.0666 | 0.00037 | 0.000796 | 0.0654 | 0 |
| 12 shoot | 6.2 | 0.00609 | 1.73e-05 | 5.04e-05 | 0.00603 | 0 |
| 13 twig | 218 | 0.215 | 0.00107 | 0.00354 | 0.21 | 0 |
| 14 short | 722 | 0.71 | 0.00172 | 0.00878 | 0.699 | 0 |
| all | 1,018 | 1 | 0.00319 | 0.0132 | 0.984 | 0 |

**M4. Last cycle's leaf deposits rounding to zero quanta on all 8 corners**

| bin | deposits | area m² | zero | zero share (count) | zero area m² | zero share (area) | net area lost to rounding m² |
|---|---|---|---|---|---|---|---|
| <0.05 | 189,027 | 3.25 | 19,737 | 0.104 | 0.00836 | 0.00258 | 0.0569 |
| 0.05-0.2 | 96,157 | 13.4 | 78 | 0.000811 | 3.33e-05 | 2.48e-06 | 0.00774 |
| >=0.2 | 615,611 | 1,001 | 25 | 4.06e-05 | 1.52e-05 | 1.52e-08 | 0.011 |

**Tops: faint laterals whose bearer node is not faint, and what is held under them (themselves included)**

| band | PA | tops | axes held | phytomers held | leaf m² |
|---|---|---|---|---|---|
| <0.05 on bearer >=0.05 | 6 limb | 2 | 177,004 | 774,378 | 1.73 |
| <0.05 on bearer >=0.05 | 7 bough | 2 | 1,360 | 3,653 | 0.00451 |
| <0.05 on bearer >=0.05 | 9 branch | 119 | 9,721 | 43,437 | 0.175 |
| <0.05 on bearer >=0.05 | 13 twig | 911 | 5,753 | 23,934 | 0.304 |
| <0.05 on bearer >=0.05 | 14 short | 3,362 | 3,362 | 15,971 | 0.117 |
| <0.2 on bearer >=0.2 | 6 limb | 2 | 244,071 | 1,028,292 | 2.3 |
| <0.2 on bearer >=0.2 | 7 bough | 2 | 9,922 | 49,522 | 0.762 |
| <0.2 on bearer >=0.2 | 9 branch | 259 | 23,585 | 122,918 | 2.29 |
| <0.2 on bearer >=0.2 | 13 twig | 1,495 | 9,521 | 45,730 | 1.88 |
| <0.2 on bearer >=0.2 | 14 short | 5,984 | 5,984 | 32,693 | 1.01 |

**Settled tree: phytomers under the drawing's 0.2 mm floor (radius x scale)**

| PA | held | under 0.2 mm | share |
|---|---|---|---|
| 2 trunk | 92 | 0 | 0 |
| 3 low_fork | 14 | 0 | 0 |
| 4 leader | 41 | 0 | 0 |
| 6 limb | 9,430 | 1 | 0.000106 |
| 7 bough | 51,555 | 355 | 0.00689 |
| 8 sprig | 173 | 0 | 0 |
| 9 branch | 1,220,908 | 266,359 | 0.218 |
| 12 shoot | 47,519 | 11,443 | 0.241 |
| 13 twig | 1,394,385 | 425,210 | 0.305 |
| 14 short | 1,959,358 | 458,141 | 0.234 |
| all | 4,683,475 | 1,161,509 | 0.248 |

**M3. Faint laterals <0.05: realised subtree over expectation**

| PA | metric | roots | p10 | p50 | p90 | sum real | sum expected | sum ratio |
|---|---|---|---|---|---|---|---|---|
| 6 limb | leaf/bL | 8 | 0.0202 | 0.0388 | 0.273 | 2.41 | 51.9 | 0.0465 |
| 6 limb | leaf/fL | 8 | 0.0202 | 0.0388 | 0.273 | 2.41 | 51.9 | 0.0465 |
| 6 limb | records/fN | 8 | 2.22 | 3.22 | 13.4 | 1,085,948 | 511,320 | 2.12 |
| 6 limb | scale/bN | 8 | 0.0337 | 0.0609 | 0.589 | 24,300 | 511,320 | 0.0475 |
| 6 limb | scale/fN | 8 | 0.0337 | 0.0609 | 0.589 | 24,300 | 511,320 | 0.0475 |
| 6 limb | wood/bW | 8 | 0.069 | 0.104 | 0.898 | 298 | 3,581 | 0.0833 |
| 6 limb | wood/fW | 8 | 0.069 | 0.104 | 0.898 | 298 | 3,581 | 0.0833 |
| 7 bough | leaf/bL | 57 | 0.0216 | 0.0814 | 0.219 | 1.54 | 21.9 | 0.0701 |
| 7 bough | leaf/fL | 57 | 0.0216 | 0.0814 | 0.219 | 1.54 | 21.9 | 0.0701 |
| 7 bough | records/fN | 57 | 2.01 | 5.61 | 27 | 670,373 | 240,679 | 2.79 |
| 7 bough | scale/bN | 57 | 0.0417 | 0.115 | 0.833 | 15,731 | 240,679 | 0.0654 |
| 7 bough | scale/fN | 57 | 0.0417 | 0.115 | 0.833 | 15,731 | 240,679 | 0.0654 |
| 7 bough | wood/bW | 57 | 0.0781 | 0.197 | 0.937 | 193 | 1,679 | 0.115 |
| 7 bough | wood/fW | 57 | 0.0781 | 0.197 | 0.937 | 193 | 1,679 | 0.115 |
| 9 branch | leaf/bL | 7,437 | 0.00796 | 0.0761 | 0.356 | 6.28 | 58.7 | 0.107 |
| 9 branch | leaf/fL | 7,437 | 0.00798 | 0.0762 | 0.356 | 6.28 | 58.4 | 0.107 |
| 9 branch | records/fN | 7,437 | 2.53 | 12.4 | 151 | 2,657,259 | 393,177 | 6.76 |
| 9 branch | scale/bN | 7,437 | 0.045 | 0.191 | 0.751 | 49,973 | 393,516 | 0.127 |
| 9 branch | scale/fN | 7,437 | 0.0451 | 0.191 | 0.751 | 49,973 | 393,177 | 0.127 |
| 9 branch | wood/bW | 7,437 | 0.0878 | 0.299 | 0.843 | 588 | 2,785 | 0.211 |
| 9 branch | wood/fW | 7,437 | 0.0878 | 0.299 | 0.843 | 588 | 2,782 | 0.211 |
| 13 twig | leaf/bL | 43,917 | 0 | 0.0538 | 0.384 | 2.68 | 25.7 | 0.105 |
| 13 twig | leaf/fL | 43,917 | 0 | 0.0567 | 0.391 | 2.68 | 22.8 | 0.118 |
| 13 twig | records/fN | 43,917 | 0 | 19.5 | 285 | 742,078 | 52,369 | 14.2 |
| 13 twig | scale/bN | 43,917 | 0 | 0.322 | 0.907 | 17,469 | 59,244 | 0.295 |
| 13 twig | scale/fN | 43,917 | 0 | 0.329 | 0.907 | 17,469 | 52,369 | 0.334 |
| 13 twig | wood/bW | 43,917 | 0 | 0.422 | 0.907 | 163 | 432 | 0.378 |
| 13 twig | wood/fW | 43,917 | 0 | 0.436 | 0.931 | 163 | 382 | 0.427 |
| 14 short | leaf/bL | 214,436 | 0 | 0 | 0.368 | 1.75 | 18.5 | 0.0947 |
| 14 short | leaf/fL | 214,436 | 0 | 0 | 0.388 | 1.75 | 17.1 | 0.102 |
| 14 short | records/fN | 214,436 | 0 | 0 | 129 | 391,794 | 24,522 | 16 |
| 14 short | scale/bN | 214,436 | 0 | 0 | 1.11 | 9,888 | 28,205 | 0.351 |
| 14 short | scale/fN | 214,436 | 0 | 0 | 1.11 | 9,888 | 24,522 | 0.403 |
| 14 short | wood/bW | 214,436 | 0 | 0 | 1.11 | 59.3 | 169 | 0.351 |
| 14 short | wood/fW | 214,436 | 0 | 0 | 1.11 | 59.3 | 147 | 0.403 |

**M3. Faint laterals 0.05-0.2: realised subtree over expectation**

| PA | metric | roots | p10 | p50 | p90 | sum real | sum expected | sum ratio |
|---|---|---|---|---|---|---|---|---|
| 6 limb | leaf/bL | 2 | 0.0267 | 0.0292 | 0.0292 | 0.988 | 34.5 | 0.0286 |
| 6 limb | leaf/fL | 2 | 0.0267 | 0.0292 | 0.0292 | 0.988 | 34.5 | 0.0286 |
| 6 limb | records/fN | 2 | 1.36 | 1.75 | 1.75 | 561,343 | 332,980 | 1.69 |
| 6 limb | scale/bN | 2 | 0.0466 | 0.0587 | 0.0587 | 16,195 | 332,980 | 0.0486 |
| 6 limb | scale/fN | 2 | 0.0466 | 0.0587 | 0.0587 | 16,195 | 332,980 | 0.0486 |
| 6 limb | wood/bW | 2 | 0.0921 | 0.108 | 0.108 | 221 | 2,334 | 0.0949 |
| 6 limb | wood/fW | 2 | 0.0921 | 0.108 | 0.108 | 221 | 2,334 | 0.0949 |
| 7 bough | leaf/bL | 22 | 0.021 | 0.0326 | 0.164 | 1.27 | 22.3 | 0.057 |
| 7 bough | leaf/fL | 22 | 0.021 | 0.0326 | 0.164 | 1.27 | 22.3 | 0.057 |
| 7 bough | records/fN | 22 | 1.02 | 2.01 | 11.4 | 272,789 | 238,608 | 1.14 |
| 7 bough | scale/bN | 22 | 0.0444 | 0.0874 | 0.71 | 13,595 | 238,608 | 0.057 |
| 7 bough | scale/fN | 22 | 0.0444 | 0.0874 | 0.71 | 13,595 | 238,608 | 0.057 |
| 7 bough | wood/bW | 22 | 0.0857 | 0.166 | 0.862 | 173 | 1,665 | 0.104 |
| 7 bough | wood/fW | 22 | 0.0857 | 0.166 | 0.862 | 173 | 1,665 | 0.104 |
| 9 branch | leaf/bL | 3,083 | 0.00568 | 0.0743 | 0.478 | 16.6 | 97.3 | 0.17 |
| 9 branch | leaf/fL | 3,083 | 0.00568 | 0.0745 | 0.478 | 16.6 | 96.1 | 0.172 |
| 9 branch | records/fN | 3,083 | 0.743 | 2.81 | 10.3 | 1,192,773 | 597,486 | 2 |
| 9 branch | scale/bN | 3,083 | 0.0457 | 0.188 | 0.784 | 84,178 | 601,403 | 0.14 |
| 9 branch | scale/fN | 3,083 | 0.046 | 0.189 | 0.784 | 84,178 | 597,486 | 0.141 |
| 9 branch | wood/bW | 3,083 | 0.0879 | 0.29 | 0.863 | 939 | 4,270 | 0.22 |
| 9 branch | wood/fW | 3,083 | 0.0881 | 0.291 | 0.866 | 939 | 4,242 | 0.221 |
| 13 twig | leaf/bL | 18,185 | 0 | 0.0875 | 0.622 | 11.3 | 54.4 | 0.207 |
| 13 twig | leaf/fL | 18,185 | 0 | 0.0905 | 0.636 | 11.3 | 49.7 | 0.226 |
| 13 twig | records/fN | 18,185 | 0.523 | 3.71 | 11 | 393,649 | 124,022 | 3.17 |
| 13 twig | scale/bN | 18,185 | 0.0581 | 0.37 | 0.907 | 46,630 | 136,603 | 0.341 |
| 13 twig | scale/fN | 18,185 | 0.0581 | 0.381 | 0.923 | 46,630 | 124,022 | 0.376 |
| 13 twig | wood/bW | 18,185 | 0.0931 | 0.471 | 0.927 | 416 | 990 | 0.421 |
| 13 twig | wood/fW | 18,185 | 0.0931 | 0.486 | 0.95 | 416 | 900 | 0.463 |
| 14 short | leaf/bL | 95,668 | 0 | 0 | 0.74 | 8.93 | 41.9 | 0.213 |
| 14 short | leaf/fL | 95,668 | 0 | 0 | 0.78 | 8.93 | 38.8 | 0.23 |
| 14 short | records/fN | 95,668 | 0 | 1.83 | 12.9 | 247,361 | 60,709 | 4.07 |
| 14 short | scale/bN | 95,668 | 0 | 0.15 | 1.23 | 31,324 | 69,210 | 0.453 |
| 14 short | scale/fN | 95,668 | 0 | 0.307 | 1.23 | 31,324 | 60,709 | 0.516 |
| 14 short | wood/bW | 95,668 | 0 | 0.15 | 1.23 | 188 | 415 | 0.453 |
| 14 short | wood/fW | 95,668 | 0 | 0.307 | 1.23 | 188 | 364 | 0.516 |

**M2. Stages**

| stage | s | heap held GiB | heap most in stage GiB | VmHWM GiB |
|---|---|---|---|---|
| growth | 23.4 | 3.2 | 3.39 | 3.3 |
| reader | 1.6 | 3.2 | 3.45 | 3.35 |
| settle_from_copy | 2.3 | 3.98 | 5.46 | 5.05 |

**M2. Grower footprint by store at end of growth**

| store | records | MiB | allocations |
|---|---|---|---|
| axes.phytomers | 5,693,800 | 1,016 | 555,675 |
| axes | 2,237,329 | 623 | 1 |
| pencils | 2,237,329 | 558 | 1 |
| draws | 2,237,329 | 387 | 1 |
| draws.units | 1,302,876 | 77.3 | 555,675 |
| working.links | 2,041,359 | 67.8 | 1 |
| working.by_node | 2,041,359 | 67.8 | 1 |
| successor | 2,237,329 | 64.4 | 1 |
| draws.nodes | 5,693,800 | 59.8 | 555,675 |
| pencils.path | 11,259,183 | 59.1 | 1,128,444 |
| retire.fate | 2,041,359 | 33.9 | 1 |
| leaves | 900,795 | 32 | 1 |
| working.pa | 2,041,359 | 22.6 | 1 |
| horizon | 2,237,329 | 21.5 | 1 |
| units | 2,237,329 | 21.5 | 1 |
| roots | 2,237,329 | 21.5 | 1 |
| draws.shade | 1,302,876 | 19.3 | 555,675 |
| working.by_node.each | 2,041,358 | 16.5 | 329,707 |
| working.at | 2,041,359 | 11.3 | 1 |
| working.light | 2,041,359 | 11.3 | 1 |
| working.cost | 2,041,359 | 11.3 | 1 |
| working.sub | 2,041,359 | 11.3 | 1 |
| working.incoming | 2,041,359 | 11.3 | 1 |
| working.bud | 2,041,359 | 11.3 | 1 |
| retire.woken | 2,041,359 | 11.3 | 1 |
| live | 374,527 | 8 | 1 |
| working.list | 422,322 | 4 | 1 |
| retire.fading | 306,396 | 4 | 1 |
| working.open | 2,041,359 | 2.82 | 1 |
| working.living | 2,041,359 | 2.82 | 1 |
| working.closed | 182,416 | 2 | 1 |
| successor.each | 53,488 | 1.78 | 58,213 |
| woken | 290 | 0.0195 | 1 |
| asleep.each | 134 | 0.0129 | 15 |
| asleep | 97 | 0.00256 | 1 |
| working.rest | 1 | 0.000244 | 1 |
| working.kids | 0 | 0.000244 | 1 |
| retire.pending | 3 | 3.05e-05 | 1 |
| axes.units | 0 | 0 | 0 |
| axes.joined | 0 | 0 | 0 |
| marked | 0 | 0 | 1 |
| pencils.inherited | 0 | 0 | 0 |
| retire.log | 0 | 0 | 1 |


### RUN census oregon-white-oak 120 1002

```
FOOT counted_gib 7.1064 axes_open_or_kept 2141571 retired_roots 1935628 inside_retired 1004477
HEAD age 120 axes 5081676 pencils 5081676 live 719340 leaves 1653309 lasting false
CHECK deposits replicated 1653309 grower 1653309 area replicated 1307.5645 grower 1307.5645
TOPSOTHER 0 carried-on axes turning faint on a bearer not faint: 81
TOPSOTHER 1 carried-on axes turning faint on a bearer not faint: 110
M3WOKEN faint woken laterals left out: 245
SCALE 1
exit 0 wall_s 105
```

**M1. Held phytomers by their axis's faintness (PA x bin x open/closed/retired-held)**

| PA | <0.05 open | <0.05 closed | <0.05 retired-held | 0.05-0.2 open | 0.05-0.2 closed | 0.05-0.2 retired-held | >=0.2 open | >=0.2 closed | >=0.2 retired-held | no pencil | total |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 trunk | 0 | 0 | 0 | 0 | 0 | 0 | 92 | 0 | 0 | 0 | 92 |
| 3 low_fork | 0 | 0 | 0 | 0 | 0 | 0 | 14 | 0 | 0 | 0 | 14 |
| 4 leader | 0 | 0 | 0 | 0 | 0 | 0 | 41 | 0 | 0 | 0 | 41 |
| 6 limb | 4,742 | 0 | 0 | 1,857 | 0 | 0 | 15,713 | 1,499 | 0 | 0 | 23,811 |
| 7 bough | 34,101 | 58 | 0 | 13,335 | 0 | 0 | 97,216 | 8,885 | 0 | 0 | 153,595 |
| 8 sprig | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 9 branch | 1,443,781 | 303,124 | 0 | 338,642 | 75,480 | 0 | 897,202 | 270,570 | 0 | 0 | 3,328,799 |
| 12 shoot | 7,966 | 36,817 | 0 | 4,156 | 13,134 | 0 | 20,186 | 54,810 | 0 | 0 | 137,069 |
| 13 twig | 885,127 | 587,513 | 0 | 268,818 | 155,845 | 0 | 891,584 | 556,681 | 0 | 0 | 3,345,568 |
| 14 short | 516,912 | 625,811 | 0 | 255,890 | 318,960 | 0 | 1,220,717 | 1,631,479 | 0 | 0 | 4,569,769 |
| all | 2,892,629 | 1,553,323 | 0 | 882,698 | 563,419 | 0 | 3,142,765 | 2,523,924 | 0 | 0 | 11,558,758 |

**M1. Held axes (PA x bin x open/closed/retired-held)**

| PA | <0.05 open | <0.05 closed | <0.05 retired-held | 0.05-0.2 open | 0.05-0.2 closed | 0.05-0.2 retired-held | >=0.2 open | >=0.2 closed | >=0.2 retired-held | no pencil | total |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 trunk | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 3 low_fork | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 4 leader | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 6 limb | 139 | 0 | 0 | 45 | 0 | 0 | 453 | 51 | 6 | 0 | 694 |
| 7 bough | 1,290 | 3 | 1 | 487 | 0 | 8 | 3,646 | 454 | 268 | 0 | 6,157 |
| 8 sprig | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 39 | 0 | 39 |
| 9 branch | 83,149 | 17,967 | 11,664 | 16,627 | 4,008 | 3,669 | 36,630 | 12,175 | 26,719 | 0 | 212,608 |
| 12 shoot | 2,862 | 13,641 | 1,740 | 872 | 3,389 | 492 | 2,766 | 8,688 | 1,842 | 0 | 36,292 |
| 13 twig | 106,449 | 54,677 | 311,679 | 25,693 | 11,517 | 64,833 | 79,357 | 40,147 | 167,788 | 0 | 862,140 |
| 14 short | 376,040 | 351,710 | 1,157,281 | 106,425 | 90,674 | 289,994 | 370,214 | 319,323 | 902,082 | 0 | 3,963,743 |
| all | 569,929 | 437,998 | 1,482,365 | 150,149 | 109,588 | 358,996 | 493,069 | 380,838 | 1,098,744 | 0 | 5,081,676 |

**M1. Held phytomers by their own rough-layout scale (status summed)**

| PA | <0.05 | 0.05-0.2 | >=0.2 |
|---|---|---|---|
| 2 trunk | 0 | 0 | 92 |
| 3 low_fork | 0 | 0 | 14 |
| 4 leader | 0 | 0 | 41 |
| 6 limb | 4,759 | 1,861 | 17,191 |
| 7 bough | 34,607 | 13,410 | 105,578 |
| 8 sprig | 0 | 0 | 0 |
| 9 branch | 1,853,445 | 392,922 | 1,082,432 |
| 12 shoot | 44,691 | 17,252 | 75,126 |
| 13 twig | 1,470,710 | 424,515 | 1,450,343 |
| 14 short | 1,143,435 | 574,970 | 2,851,364 |
| all | 4,551,647 | 1,424,930 | 5,582,181 |

**M1. Living tips by axis faintness** (by status: {'open': 719340})

| PA | <0.05 | 0.05-0.2 | >=0.2 | no pencil | summed f |
|---|---|---|---|---|---|
| 6 limb | 20 | 7 | 41 | 0 | 37.4 |
| 7 bough | 189 | 69 | 403 | 0 | 351 |
| 9 branch | 16,308 | 3,555 | 11,949 | 0 | 9,714 |
| 12 shoot | 1,766 | 472 | 1,507 | 0 | 1,197 |
| 13 twig | 71,987 | 15,537 | 48,753 | 0 | 38,307 |
| 14 short | 238,621 | 64,431 | 243,725 | 0 | 192,369 |
| all | 328,891 | 84,071 | 306,378 | 0 | 241,976 |

**M1. The grower's leaf (last cycle's deposits), m², and share of the whole**

| PA | m² | share | share <0.05 | share 0.05-0.2 | share >=0.2 | share no pencil |
|---|---|---|---|---|---|---|
| 6 limb | 0.399 | 0.000305 | 2.83e-06 | 2.06e-06 | 0.0003 | 0 |
| 7 bough | 3.72 | 0.00284 | 2.63e-05 | 1.94e-05 | 0.0028 | 0 |
| 9 branch | 83.1 | 0.0635 | 0.0011 | 0.00105 | 0.0614 | 0 |
| 12 shoot | 7.6 | 0.00581 | 5.72e-05 | 8.73e-05 | 0.00567 | 0 |
| 13 twig | 269 | 0.205 | 0.00276 | 0.00421 | 0.198 | 0 |
| 14 short | 944 | 0.722 | 0.00436 | 0.0128 | 0.705 | 0 |
| all | 1,308 | 1 | 0.00831 | 0.0181 | 0.974 | 0 |

**M4. Last cycle's leaf deposits rounding to zero quanta on all 8 corners**

| bin | deposits | area m² | zero | zero share (count) | zero area m² | zero share (area) | net area lost to rounding m² |
|---|---|---|---|---|---|---|---|
| <0.05 | 627,922 | 10.9 | 112,988 | 0.18 | 0.039 | 0.00359 | 0.168 |
| 0.05-0.2 | 195,703 | 23.7 | 347 | 0.00177 | 0.000288 | 1.21e-05 | 0.0222 |
| >=0.2 | 829,684 | 1,273 | 46 | 5.54e-05 | 3.02e-05 | 2.37e-08 | 0.0137 |

**Tops: faint laterals whose bearer node is not faint, and what is held under them (themselves included)**

| band | PA | tops | axes held | phytomers held | leaf m² |
|---|---|---|---|---|---|
| <0.05 on bearer >=0.05 | 6 limb | 2 | 705,553 | 3,011,124 | 7.66 |
| <0.05 on bearer >=0.05 | 7 bough | 5 | 17,701 | 72,014 | 0.0401 |
| <0.05 on bearer >=0.05 | 9 branch | 319 | 32,688 | 179,015 | 0.643 |
| <0.05 on bearer >=0.05 | 13 twig | 1,842 | 11,438 | 51,126 | 0.347 |
| <0.05 on bearer >=0.05 | 14 short | 6,543 | 6,543 | 32,453 | 0.194 |
| <0.2 on bearer >=0.2 | 6 limb | 2 | 981,577 | 4,227,830 | 9.39 |
| <0.2 on bearer >=0.2 | 7 bough | 3 | 16,921 | 96,417 | 1.08 |
| <0.2 on bearer >=0.2 | 9 branch | 402 | 41,744 | 275,525 | 5.66 |
| <0.2 on bearer >=0.2 | 13 twig | 2,252 | 14,535 | 75,054 | 2.69 |
| <0.2 on bearer >=0.2 | 14 short | 9,277 | 9,277 | 53,798 | 1.49 |

**Settled tree: phytomers under the drawing's 0.2 mm floor (radius x scale)**

| PA | held | under 0.2 mm | share |
|---|---|---|---|
| 2 trunk | 92 | 0 | 0 |
| 3 low_fork | 14 | 0 | 0 |
| 4 leader | 41 | 0 | 0 |
| 6 limb | 22,116 | 197 | 0.00891 |
| 7 bough | 148,370 | 2,117 | 0.0143 |
| 9 branch | 2,864,258 | 1,255,360 | 0.438 |
| 12 shoot | 95,095 | 41,546 | 0.437 |
| 13 twig | 2,640,232 | 1,324,017 | 0.501 |
| 14 short | 3,405,616 | 1,265,519 | 0.372 |
| all | 9,175,834 | 3,888,756 | 0.424 |

**M3. Faint laterals <0.05: realised subtree over expectation**

| PA | metric | roots | p10 | p50 | p90 | sum real | sum expected | sum ratio |
|---|---|---|---|---|---|---|---|---|
| 6 limb | leaf/bL | 20 | 0.0156 | 0.0776 | 0.127 | 19 | 231 | 0.0824 |
| 6 limb | leaf/fL | 20 | 0.0156 | 0.0776 | 0.127 | 19 | 231 | 0.0824 |
| 6 limb | records/fN | 20 | 1.65 | 4 | 16.6 | 5,970,980 | 3,281,509 | 1.82 |
| 6 limb | scale/bN | 20 | 0.0438 | 0.0785 | 0.346 | 133,381 | 3,281,509 | 0.0406 |
| 6 limb | scale/fN | 20 | 0.0438 | 0.0785 | 0.346 | 133,381 | 3,281,509 | 0.0406 |
| 6 limb | wood/bW | 20 | 0.0796 | 0.157 | 0.607 | 1,719 | 22,684 | 0.0758 |
| 6 limb | wood/fW | 20 | 0.0796 | 0.157 | 0.607 | 1,719 | 22,684 | 0.0758 |
| 7 bough | leaf/bL | 190 | 0.0131 | 0.0767 | 0.175 | 7.18 | 90.9 | 0.079 |
| 7 bough | leaf/fL | 190 | 0.0131 | 0.0767 | 0.175 | 7.18 | 90.9 | 0.079 |
| 7 bough | records/fN | 190 | 1.43 | 4.09 | 20.3 | 2,814,882 | 1,365,201 | 2.06 |
| 7 bough | scale/bN | 190 | 0.0257 | 0.0937 | 0.543 | 57,328 | 1,365,201 | 0.042 |
| 7 bough | scale/fN | 190 | 0.0257 | 0.0937 | 0.543 | 57,328 | 1,365,201 | 0.042 |
| 7 bough | wood/bW | 190 | 0.0538 | 0.163 | 0.688 | 755 | 9,419 | 0.0801 |
| 7 bough | wood/fW | 190 | 0.0538 | 0.163 | 0.688 | 755 | 9,419 | 0.0801 |
| 9 branch | leaf/bL | 32,810 | 0.00138 | 0.0568 | 0.362 | 25.5 | 221 | 0.115 |
| 9 branch | leaf/fL | 32,810 | 0.00139 | 0.0569 | 0.362 | 25.5 | 219 | 0.117 |
| 9 branch | records/fN | 32,810 | 2.4 | 14.2 | 398 | 14,294,140 | 1,775,412 | 8.05 |
| 9 branch | scale/bN | 32,810 | 0.0336 | 0.133 | 0.662 | 180,502 | 1,785,341 | 0.101 |
| 9 branch | scale/fN | 32,810 | 0.0337 | 0.133 | 0.664 | 180,502 | 1,775,412 | 0.102 |
| 9 branch | wood/bW | 32,810 | 0.0703 | 0.231 | 0.8 | 2,268 | 12,531 | 0.181 |
| 9 branch | wood/fW | 32,810 | 0.0704 | 0.231 | 0.8 | 2,268 | 12,459 | 0.182 |
| 13 twig | leaf/bL | 161,126 | 0 | 0.0165 | 0.451 | 8.44 | 75 | 0.113 |
| 13 twig | leaf/fL | 161,126 | 0 | 0.0188 | 0.457 | 8.44 | 70.1 | 0.12 |
| 13 twig | records/fN | 161,126 | 0 | 22.4 | 749 | 2,386,056 | 160,373 | 14.9 |
| 13 twig | scale/bN | 161,126 | 0 | 0.267 | 0.907 | 48,012 | 174,480 | 0.275 |
| 13 twig | scale/fN | 161,126 | 0 | 0.277 | 0.907 | 48,012 | 160,373 | 0.299 |
| 13 twig | wood/bW | 161,126 | 0 | 0.369 | 0.907 | 460 | 1,273 | 0.361 |
| 13 twig | wood/fW | 161,126 | 0 | 0.373 | 0.932 | 460 | 1,172 | 0.392 |
| 14 short | leaf/bL | 727,750 | 0 | 0 | 0.376 | 5.7 | 53.7 | 0.106 |
| 14 short | leaf/fL | 727,750 | 0 | 0 | 0.392 | 5.7 | 51.2 | 0.111 |
| 14 short | records/fN | 727,750 | 0 | 0 | 160 | 1,142,723 | 72,489 | 15.8 |
| 14 short | scale/bN | 727,750 | 0 | 0 | 1.11 | 25,000 | 79,089 | 0.316 |
| 14 short | scale/fN | 727,750 | 0 | 0 | 1.11 | 25,000 | 72,489 | 0.345 |
| 14 short | wood/bW | 727,750 | 0 | 0 | 1.11 | 150 | 475 | 0.316 |
| 14 short | wood/fW | 727,750 | 0 | 0 | 1.11 | 150 | 435 | 0.345 |

**M3. Faint laterals 0.05-0.2: realised subtree over expectation**

| PA | metric | roots | p10 | p50 | p90 | sum real | sum expected | sum ratio |
|---|---|---|---|---|---|---|---|---|
| 6 limb | leaf/bL | 6 | 0.021 | 0.0239 | 0.146 | 3.78 | 145 | 0.026 |
| 6 limb | leaf/fL | 6 | 0.021 | 0.0239 | 0.146 | 3.78 | 145 | 0.026 |
| 6 limb | records/fN | 6 | 1.56 | 2.69 | 4.07 | 2,874,260 | 2,087,048 | 1.38 |
| 6 limb | scale/bN | 6 | 0.0363 | 0.135 | 0.392 | 80,452 | 2,087,048 | 0.0385 |
| 6 limb | scale/fN | 6 | 0.0363 | 0.135 | 0.392 | 80,452 | 2,087,048 | 0.0385 |
| 6 limb | wood/bW | 6 | 0.0708 | 0.225 | 0.496 | 1,044 | 14,421 | 0.0724 |
| 6 limb | wood/fW | 6 | 0.0708 | 0.225 | 0.496 | 1,044 | 14,421 | 0.0724 |
| 7 bough | leaf/bL | 69 | 0.0178 | 0.0306 | 0.145 | 3.88 | 73 | 0.0532 |
| 7 bough | leaf/fL | 69 | 0.0178 | 0.0306 | 0.145 | 3.88 | 73 | 0.0532 |
| 7 bough | records/fN | 69 | 0.924 | 1.88 | 5.8 | 1,230,743 | 1,175,639 | 1.05 |
| 7 bough | scale/bN | 69 | 0.0285 | 0.0925 | 0.685 | 55,690 | 1,175,639 | 0.0474 |
| 7 bough | scale/fN | 69 | 0.0285 | 0.0925 | 0.685 | 55,690 | 1,175,639 | 0.0474 |
| 7 bough | wood/bW | 69 | 0.0564 | 0.163 | 0.751 | 672 | 8,093 | 0.083 |
| 7 bough | wood/fW | 69 | 0.0564 | 0.163 | 0.751 | 672 | 8,093 | 0.083 |
| 9 branch | leaf/bL | 8,035 | 0 | 0.0541 | 0.483 | 46.6 | 232 | 0.201 |
| 9 branch | leaf/fL | 8,035 | 0 | 0.0542 | 0.486 | 46.6 | 224 | 0.208 |
| 9 branch | records/fN | 8,035 | 0.698 | 3.06 | 9.97 | 4,659,300 | 1,940,602 | 2.4 |
| 9 branch | scale/bN | 8,035 | 0.0373 | 0.178 | 0.736 | 274,994 | 1,962,129 | 0.14 |
| 9 branch | scale/fN | 8,035 | 0.0375 | 0.18 | 0.751 | 274,994 | 1,940,602 | 0.142 |
| 9 branch | wood/bW | 8,035 | 0.0736 | 0.28 | 0.824 | 3,032 | 13,744 | 0.221 |
| 9 branch | wood/fW | 8,035 | 0.0739 | 0.282 | 0.834 | 3,032 | 13,583 | 0.223 |
| 13 twig | leaf/bL | 37,210 | 0 | 0.053 | 0.535 | 19.5 | 101 | 0.193 |
| 13 twig | leaf/fL | 37,210 | 0 | 0.0542 | 0.548 | 19.5 | 93 | 0.21 |
| 13 twig | records/fN | 37,210 | 0.657 | 4.03 | 11.5 | 895,389 | 256,123 | 3.5 |
| 13 twig | scale/bN | 37,210 | 0.063 | 0.367 | 0.907 | 98,727 | 280,078 | 0.352 |
| 13 twig | scale/fN | 37,210 | 0.0726 | 0.378 | 0.93 | 98,727 | 256,123 | 0.385 |
| 13 twig | wood/bW | 37,210 | 0.0931 | 0.463 | 0.932 | 865 | 2,018 | 0.429 |
| 13 twig | wood/fW | 37,210 | 0.101 | 0.474 | 0.956 | 865 | 1,847 | 0.468 |
| 14 short | leaf/bL | 197,099 | 0 | 0 | 0.651 | 16.7 | 76.6 | 0.218 |
| 14 short | leaf/fL | 197,099 | 0 | 0 | 0.687 | 16.7 | 72 | 0.232 |
| 14 short | records/fN | 197,099 | 0 | 3.12 | 14.1 | 574,850 | 122,197 | 4.7 |
| 14 short | scale/bN | 197,099 | 0 | 0.307 | 1.23 | 68,364 | 136,107 | 0.502 |
| 14 short | scale/fN | 197,099 | 0 | 0.316 | 1.26 | 68,364 | 122,197 | 0.559 |
| 14 short | wood/bW | 197,099 | 0 | 0.307 | 1.23 | 410 | 817 | 0.502 |
| 14 short | wood/fW | 197,099 | 0 | 0.316 | 1.26 | 410 | 733 | 0.559 |

**M2. Stages**

| stage | s | heap held GiB | heap most in stage GiB | VmHWM GiB |
|---|---|---|---|---|
| growth | 90.5 | 7.11 | 8.23 | 7.31 |
| reader | 6.3 | 7.11 | 7.81 | 7.54 |
| settle_from_copy | 5.5 | 8.65 | 12 | 11 |

**M2. Grower footprint by store at end of growth**

| store | records | MiB | allocations |
|---|---|---|---|
| axes.phytomers | 11,558,758 | 2,067 | 1,106,617 |
| axes | 5,081,676 | 1,470 | 1 |
| pencils | 5,081,676 | 1,318 | 1 |
| draws | 5,081,676 | 912 | 1 |
| draws.units | 2,611,650 | 155 | 1,106,617 |
| successor | 5,081,676 | 152 | 1 |
| working.links | 4,690,360 | 150 | 1 |
| working.by_node | 4,690,360 | 150 | 1 |
| pencils.path | 25,275,083 | 133 | 2,461,047 |
| draws.nodes | 11,558,758 | 122 | 1,106,617 |
| retire.fate | 4,690,360 | 74.8 | 1 |
| leaves | 1,653,309 | 64 | 1 |
| horizon | 5,081,676 | 50.7 | 1 |
| units | 5,081,676 | 50.7 | 1 |
| roots | 5,081,676 | 50.7 | 1 |
| working.pa | 4,690,360 | 49.8 | 1 |
| working.by_node.each | 4,690,359 | 41.9 | 845,235 |
| draws.shade | 2,611,650 | 38.7 | 1,106,617 |
| working.at | 4,690,360 | 24.9 | 1 |
| working.light | 4,690,360 | 24.9 | 1 |
| working.cost | 4,690,360 | 24.9 | 1 |
| working.sub | 4,690,360 | 24.9 | 1 |
| working.incoming | 4,690,360 | 24.9 | 1 |
| working.bud | 4,690,360 | 24.9 | 1 |
| retire.woken | 4,690,360 | 24.9 | 1 |
| live | 719,340 | 16 | 1 |
| working.list | 821,831 | 8 | 1 |
| retire.fading | 604,709 | 8 | 1 |
| working.open | 4,690,360 | 6.23 | 1 |
| working.living | 4,690,360 | 6.23 | 1 |
| successor.each | 153,070 | 5.37 | 176,057 |
| working.closed | 406,044 | 4 | 1 |
| woken | 842 | 0.0391 | 1 |
| asleep.each | 191 | 0.0179 | 15 |
| asleep | 136 | 0.00513 | 1 |
| working.rest | 1 | 0.000244 | 1 |
| working.kids | 0 | 0.000244 | 1 |
| retire.pending | 3 | 3.05e-05 | 1 |
| axes.units | 0 | 0 | 0 |
| axes.joined | 0 | 0 | 0 |
| marked | 0 | 0 | 1 |
| pencils.inherited | 0 | 0 | 0 |
| retire.log | 0 | 0 | 1 |


### RUN census european-beech 40 1002

```
FOOT counted_gib 0.1738 axes_open_or_kept 45164 retired_roots 14085 inside_retired 5887
HEAD age 40 axes 65136 pencils 65136 live 33230 leaves 106025 lasting false
CHECK deposits replicated 106025 grower 106025 area replicated 113.1742 grower 113.1742
TOPSOTHER 0 carried-on axes turning faint on a bearer not faint: 1
TOPSOTHER 1 carried-on axes turning faint on a bearer not faint: 0
M3WOKEN faint woken laterals left out: 0
SCALE 1
exit 0 wall_s 1
```

**M1. Held phytomers by their axis's faintness (PA x bin x open/closed/retired-held)**

| PA | <0.05 open | <0.05 closed | <0.05 retired-held | 0.05-0.2 open | 0.05-0.2 closed | 0.05-0.2 retired-held | >=0.2 open | >=0.2 closed | >=0.2 retired-held | no pencil | total |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 trunk | 0 | 0 | 0 | 0 | 0 | 0 | 60 | 0 | 0 | 0 | 60 |
| 4 leader | 0 | 0 | 0 | 0 | 0 | 0 | 180 | 0 | 0 | 0 | 180 |
| 6 limb | 94 | 0 | 0 | 0 | 0 | 0 | 811 | 0 | 0 | 0 | 905 |
| 7 bough | 389 | 0 | 0 | 0 | 0 | 0 | 1,072 | 0 | 0 | 0 | 1,461 |
| 8 sprig | 206 | 0 | 0 | 28 | 0 | 0 | 1,383 | 0 | 0 | 0 | 1,617 |
| 9 branch | 5,742 | 43 | 0 | 447 | 0 | 0 | 27,574 | 24 | 0 | 0 | 33,830 |
| 12 shoot | 15,278 | 809 | 0 | 1,632 | 17 | 0 | 78,650 | 2,877 | 0 | 0 | 99,263 |
| 14 short | 68,138 | 11,216 | 0 | 10,810 | 1,175 | 0 | 303,588 | 39,273 | 0 | 0 | 434,200 |
| all | 89,847 | 12,068 | 0 | 12,917 | 1,192 | 0 | 413,318 | 42,174 | 0 | 0 | 571,516 |

**M1. Held axes (PA x bin x open/closed/retired-held)**

| PA | <0.05 open | <0.05 closed | <0.05 retired-held | 0.05-0.2 open | 0.05-0.2 closed | 0.05-0.2 retired-held | >=0.2 open | >=0.2 closed | >=0.2 retired-held | no pencil | total |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2 trunk | 0 | 0 | 0 | 0 | 0 | 0 | 4 | 0 | 0 | 0 | 4 |
| 4 leader | 0 | 0 | 0 | 0 | 0 | 0 | 11 | 0 | 0 | 0 | 11 |
| 6 limb | 6 | 0 | 0 | 0 | 0 | 0 | 48 | 0 | 0 | 0 | 54 |
| 7 bough | 28 | 0 | 0 | 0 | 0 | 0 | 81 | 0 | 0 | 0 | 109 |
| 8 sprig | 7 | 0 | 0 | 1 | 0 | 0 | 29 | 0 | 0 | 0 | 37 |
| 9 branch | 170 | 3 | 7 | 14 | 0 | 1 | 916 | 5 | 69 | 0 | 1,185 |
| 12 shoot | 1,291 | 62 | 244 | 143 | 4 | 21 | 7,329 | 251 | 1,062 | 0 | 10,407 |
| 14 short | 4,916 | 550 | 3,606 | 769 | 76 | 486 | 25,950 | 2,500 | 14,476 | 0 | 53,329 |
| all | 6,418 | 615 | 3,857 | 927 | 80 | 508 | 34,368 | 2,756 | 15,607 | 0 | 65,136 |

**M1. Held phytomers by their own rough-layout scale (status summed)**

| PA | <0.05 | 0.05-0.2 | >=0.2 |
|---|---|---|---|
| 2 trunk | 0 | 0 | 60 |
| 4 leader | 0 | 0 | 180 |
| 6 limb | 95 | 1 | 809 |
| 7 bough | 389 | 8 | 1,064 |
| 8 sprig | 208 | 31 | 1,378 |
| 9 branch | 5,770 | 437 | 27,623 |
| 12 shoot | 16,089 | 1,718 | 81,456 |
| 14 short | 79,386 | 11,765 | 343,049 |
| all | 101,937 | 13,960 | 455,619 |

**M1. Living tips by axis faintness** (by status: {'open': 33230})

| PA | <0.05 | 0.05-0.2 | >=0.2 | no pencil | summed f |
|---|---|---|---|---|---|
| 4 leader | 0 | 0 | 1 | 0 | 1 |
| 6 limb | 1 | 0 | 9 | 0 | 8.49 |
| 7 bough | 8 | 0 | 17 | 0 | 15.1 |
| 8 sprig | 6 | 1 | 28 | 0 | 23 |
| 9 branch | 93 | 8 | 559 | 0 | 477 |
| 12 shoot | 537 | 61 | 3,669 | 0 | 3,206 |
| 14 short | 4,382 | 678 | 23,172 | 0 | 19,327 |
| all | 5,027 | 748 | 27,455 | 0 | 23,058 |

**M1. The grower's leaf (last cycle's deposits), m², and share of the whole**

| PA | m² | share | share <0.05 | share 0.05-0.2 | share >=0.2 | share no pencil |
|---|---|---|---|---|---|---|
| 4 leader | 0.00943 | 8.34e-05 | 0 | 0 | 8.34e-05 | 0 |
| 6 limb | 0.083 | 0.000733 | 3.03e-06 | 0 | 0.00073 | 0 |
| 7 bough | 0.131 | 0.00116 | 1.82e-05 | 0 | 0.00114 | 0 |
| 8 sprig | 0.212 | 0.00187 | 9.45e-06 | 9.06e-06 | 0.00185 | 0 |
| 9 branch | 4.15 | 0.0367 | 0.000156 | 5.62e-05 | 0.0365 | 0 |
| 12 shoot | 15.4 | 0.136 | 0.000332 | 0.000302 | 0.135 | 0 |
| 14 short | 93.2 | 0.823 | 0.00275 | 0.00389 | 0.817 | 0 |
| all | 113 | 1 | 0.00327 | 0.00426 | 0.992 | 0 |

**M4. Last cycle's leaf deposits rounding to zero quanta on all 8 corners**

| bin | deposits | area m² | zero | zero share (count) | zero area m² | zero share (area) | net area lost to rounding m² |
|---|---|---|---|---|---|---|---|
| <0.05 | 16,732 | 0.37 | 239 | 0.0143 | 0.000181 | 0.00049 | -0.000255 |
| 0.05-0.2 | 2,473 | 0.482 | 0 | 0 | 0 | 0 | 0.000119 |
| >=0.2 | 86,820 | 112 | 1 | 1.15e-05 | 2.79e-07 | 2.48e-09 | 0.000733 |

**Tops: faint laterals whose bearer node is not faint, and what is held under them (themselves included)**

| band | PA | tops | axes held | phytomers held | leaf m² |
|---|---|---|---|---|---|
| <0.05 on bearer >=0.05 | 6 limb | 1 | 1,015 | 12,573 | 0.144 |
| <0.05 on bearer >=0.05 | 7 bough | 3 | 5,591 | 84,924 | 0.184 |
| <0.05 on bearer >=0.05 | 9 branch | 7 | 248 | 1,955 | 0.158 |
| <0.05 on bearer >=0.05 | 12 shoot | 23 | 109 | 900 | 0.0269 |
| <0.05 on bearer >=0.05 | 14 short | 109 | 109 | 1,850 | 0.0118 |
| <0.2 on bearer >=0.2 | 6 limb | 1 | 1,015 | 12,573 | 0.144 |
| <0.2 on bearer >=0.2 | 7 bough | 3 | 5,591 | 84,924 | 0.184 |
| <0.2 on bearer >=0.2 | 8 sprig | 1 | 49 | 267 | 0.0238 |
| <0.2 on bearer >=0.2 | 9 branch | 17 | 841 | 11,837 | 1.09 |
| <0.2 on bearer >=0.2 | 12 shoot | 75 | 351 | 4,506 | 0.172 |
| <0.2 on bearer >=0.2 | 14 short | 370 | 370 | 6,062 | 0.156 |

**Settled tree: phytomers under the drawing's 0.2 mm floor (radius x scale)**

| PA | held | under 0.2 mm | share |
|---|---|---|---|
| 2 trunk | 60 | 0 | 0 |
| 4 leader | 180 | 0 | 0 |
| 6 limb | 905 | 0 | 0 |
| 7 bough | 1,461 | 37 | 0.0253 |
| 8 sprig | 1,617 | 19 | 0.0118 |
| 9 branch | 33,794 | 1,328 | 0.0393 |
| 12 shoot | 96,976 | 13,620 | 0.14 |
| 14 short | 391,709 | 72,380 | 0.185 |
| all | 526,702 | 87,384 | 0.166 |

**M3. Faint laterals <0.05: realised subtree over expectation**

| PA | metric | roots | p10 | p50 | p90 | sum real | sum expected | sum ratio |
|---|---|---|---|---|---|---|---|---|
| 6 limb | leaf/bL | 1 | 0.211 | 0.211 | 0.211 | 0.144 | 0.684 | 0.211 |
| 6 limb | leaf/fL | 1 | 0.211 | 0.211 | 0.211 | 0.144 | 0.684 | 0.211 |
| 6 limb | records/fN | 1 | 6.25 | 6.25 | 6.25 | 12,573 | 2,013 | 6.25 |
| 6 limb | scale/bN | 1 | 0.252 | 0.252 | 0.252 | 508 | 2,013 | 0.252 |
| 6 limb | scale/fN | 1 | 0.252 | 0.252 | 0.252 | 508 | 2,013 | 0.252 |
| 6 limb | wood/bW | 1 | 0.275 | 0.275 | 0.275 | 5.99 | 21.8 | 0.275 |
| 6 limb | wood/fW | 1 | 0.275 | 0.275 | 0.275 | 5.99 | 21.8 | 0.275 |
| 7 bough | leaf/bL | 8 | 0.122 | 0.455 | 0.995 | 0.215 | 0.624 | 0.345 |
| 7 bough | leaf/fL | 8 | 0.122 | 0.455 | 0.995 | 0.215 | 0.624 | 0.345 |
| 7 bough | records/fN | 8 | 4.3 | 22.8 | 39 | 86,684 | 2,557 | 33.9 |
| 7 bough | scale/bN | 8 | 0.187 | 0.332 | 0.995 | 807 | 2,557 | 0.316 |
| 7 bough | scale/fN | 8 | 0.187 | 0.332 | 0.995 | 807 | 2,557 | 0.316 |
| 7 bough | wood/bW | 8 | 0.186 | 0.383 | 1 | 9.04 | 25.7 | 0.352 |
| 7 bough | wood/fW | 8 | 0.186 | 0.383 | 1 | 9.04 | 25.7 | 0.352 |
| 8 sprig | leaf/bL | 7 | 0.282 | 0.408 | 1.17 | 0.0913 | 0.135 | 0.676 |
| 8 sprig | leaf/fL | 7 | 0.282 | 0.408 | 1.17 | 0.0913 | 0.135 | 0.676 |
| 8 sprig | records/fN | 7 | 9.39 | 44.8 | 81.9 | 32,030 | 422 | 76 |
| 8 sprig | scale/bN | 7 | 0.398 | 0.41 | 1.15 | 286 | 422 | 0.679 |
| 8 sprig | scale/fN | 7 | 0.398 | 0.41 | 1.15 | 286 | 422 | 0.679 |
| 8 sprig | wood/bW | 7 | 0.369 | 0.501 | 1.14 | 3.22 | 4.62 | 0.697 |
| 8 sprig | wood/fW | 7 | 0.369 | 0.501 | 1.14 | 3.22 | 4.62 | 0.697 |
| 9 branch | leaf/bL | 173 | 0.215 | 0.797 | 1.3 | 0.614 | 1.06 | 0.58 |
| 9 branch | leaf/fL | 173 | 0.215 | 0.81 | 1.3 | 0.614 | 0.82 | 0.749 |
| 9 branch | records/fN | 173 | 11.4 | 64.3 | 126 | 152,562 | 3,355 | 45.5 |
| 9 branch | scale/bN | 173 | 0.284 | 0.797 | 1.2 | 2,138 | 4,151 | 0.515 |
| 9 branch | scale/fN | 173 | 0.284 | 0.801 | 1.23 | 2,138 | 3,355 | 0.637 |
| 9 branch | wood/bW | 173 | 0.287 | 0.799 | 1.21 | 23.5 | 40.9 | 0.574 |
| 9 branch | wood/fW | 173 | 0.287 | 0.801 | 1.3 | 23.5 | 32.7 | 0.717 |
| 12 shoot | leaf/bL | 1,316 | 0.298 | 0.949 | 1.6 | 0.298 | 0.356 | 0.836 |
| 12 shoot | leaf/fL | 1,316 | 0.3 | 0.957 | 1.62 | 0.298 | 0.3 | 0.992 |
| 12 shoot | records/fN | 1,316 | 24.6 | 92.7 | 160 | 83,725 | 1,290 | 64.9 |
| 12 shoot | scale/bN | 1,316 | 0.369 | 0.902 | 1.33 | 1,101 | 1,447 | 0.761 |
| 12 shoot | scale/fN | 1,316 | 0.369 | 0.902 | 1.33 | 1,101 | 1,290 | 0.853 |
| 12 shoot | wood/bW | 1,316 | 0.516 | 0.953 | 1.25 | 10.9 | 13 | 0.841 |
| 12 shoot | wood/fW | 1,316 | 0.516 | 0.957 | 1.27 | 10.9 | 11.4 | 0.952 |
| 14 short | leaf/bL | 5,466 | 0 | 1.27 | 2.35 | 0.311 | 0.376 | 0.828 |
| 14 short | leaf/fL | 5,466 | 0 | 1.37 | 2.39 | 0.311 | 0.269 | 1.16 |
| 14 short | records/fN | 5,466 | 27.9 | 136 | 206 | 79,354 | 781 | 102 |
| 14 short | scale/bN | 5,466 | 0.738 | 1.27 | 1.63 | 1,233 | 1,242 | 0.993 |
| 14 short | scale/fN | 5,466 | 0.82 | 1.29 | 1.66 | 1,233 | 781 | 1.58 |
| 14 short | wood/bW | 5,466 | 0.738 | 1.27 | 1.63 | 7.4 | 7.45 | 0.993 |
| 14 short | wood/fW | 5,466 | 0.82 | 1.29 | 1.66 | 7.4 | 4.69 | 1.58 |

**M3. Faint laterals 0.05-0.2: realised subtree over expectation**

| PA | metric | roots | p10 | p50 | p90 | sum real | sum expected | sum ratio |
|---|---|---|---|---|---|---|---|---|
| 8 sprig | leaf/bL | 1 | 0.561 | 0.561 | 0.561 | 0.0238 | 0.0424 | 0.561 |
| 8 sprig | leaf/fL | 1 | 0.561 | 0.561 | 0.561 | 0.0238 | 0.0424 | 0.561 |
| 8 sprig | records/fN | 1 | 5.5 | 5.5 | 5.5 | 267 | 48.6 | 5.5 |
| 8 sprig | scale/bN | 1 | 0.684 | 0.684 | 0.684 | 33.2 | 48.6 | 0.684 |
| 8 sprig | scale/fN | 1 | 0.684 | 0.684 | 0.684 | 33.2 | 48.6 | 0.684 |
| 8 sprig | wood/bW | 1 | 0.602 | 0.602 | 0.602 | 0.627 | 1.04 | 0.602 |
| 8 sprig | wood/fW | 1 | 0.602 | 0.602 | 0.602 | 0.627 | 1.04 | 0.602 |
| 9 branch | leaf/bL | 14 | 0.318 | 0.458 | 1.13 | 0.977 | 1.41 | 0.695 |
| 9 branch | leaf/fL | 14 | 0.382 | 0.731 | 3.96 | 0.977 | 0.437 | 2.23 |
| 9 branch | records/fN | 14 | 3.1 | 5.48 | 9.35 | 11,184 | 1,947 | 5.74 |
| 9 branch | scale/bN | 14 | 0.336 | 0.506 | 0.891 | 5,197 | 7,942 | 0.654 |
| 9 branch | scale/fN | 14 | 0.407 | 0.848 | 5.56 | 5,197 | 1,947 | 2.67 |
| 9 branch | wood/bW | 14 | 0.38 | 0.544 | 0.885 | 51.8 | 72.1 | 0.719 |
| 9 branch | wood/fW | 14 | 0.475 | 0.785 | 4.61 | 51.8 | 18.5 | 2.8 |
| 12 shoot | leaf/bL | 144 | 0.219 | 0.853 | 1.48 | 0.304 | 0.388 | 0.785 |
| 12 shoot | leaf/fL | 144 | 0.32 | 0.902 | 1.66 | 0.304 | 0.29 | 1.05 |
| 12 shoot | records/fN | 144 | 3.6 | 7.51 | 14.5 | 8,415 | 1,165 | 7.22 |
| 12 shoot | scale/bN | 144 | 0.311 | 0.88 | 1.29 | 1,120 | 1,628 | 0.688 |
| 12 shoot | scale/fN | 144 | 0.38 | 0.897 | 1.48 | 1,120 | 1,165 | 0.961 |
| 12 shoot | wood/bW | 144 | 0.46 | 0.902 | 1.29 | 10.9 | 14.4 | 0.757 |
| 12 shoot | wood/fW | 144 | 0.538 | 0.911 | 1.36 | 10.9 | 10.4 | 1.05 |
| 14 short | leaf/bL | 845 | 0 | 1.14 | 2.28 | 0.44 | 0.582 | 0.756 |
| 14 short | leaf/fL | 845 | 0 | 1.37 | 2.35 | 0.44 | 0.375 | 1.17 |
| 14 short | records/fN | 845 | 5.3 | 10 | 19.2 | 11,985 | 1,091 | 11 |
| 14 short | scale/bN | 845 | 0.439 | 1.17 | 1.56 | 2,001 | 1,971 | 1.02 |
| 14 short | scale/fN | 845 | 0.803 | 1.32 | 1.94 | 2,001 | 1,091 | 1.83 |
| 14 short | wood/bW | 845 | 0.439 | 1.17 | 1.56 | 12 | 11.8 | 1.02 |
| 14 short | wood/fW | 845 | 0.803 | 1.32 | 1.94 | 12 | 6.55 | 1.83 |

**M2. Stages**

| stage | s | heap held GiB | heap most in stage GiB | VmHWM GiB |
|---|---|---|---|---|
| growth | 0.4 | 0.174 | 0.18 | 0.192 |
| reader | 0.1 | 0.174 | 0.183 | 0.193 |
| settle_from_copy | 0.2 | 0.251 | 0.299 | 0.307 |

**M2. Grower footprint by store at end of growth**

| store | records | MiB | allocations |
|---|---|---|---|
| axes.phytomers | 571,516 | 106 | 36,303 |
| pencils | 65,136 | 14.6 | 1 |
| axes | 65,136 | 14.5 | 1 |
| draws | 65,136 | 9 | 1 |
| draws.units | 142,758 | 6.8 | 36,303 |
| draws.nodes | 571,516 | 6.21 | 36,303 |
| pencils.path | 838,358 | 4.53 | 51,717 |
| leaves | 106,025 | 4 | 1 |
| draws.shade | 142,758 | 1.7 | 36,303 |
| successor | 65,136 | 1.5 | 1 |
| working.links | 57,669 | 1.5 | 1 |
| working.by_node | 57,669 | 1.5 | 1 |
| live | 33,230 | 1 | 1 |
| retire.fate | 57,669 | 0.844 | 1 |
| horizon | 65,136 | 0.5 | 1 |
| units | 65,136 | 0.5 | 1 |
| roots | 65,136 | 0.5 | 1 |
| working.pa | 57,669 | 0.5 | 1 |
| working.list | 34,246 | 0.5 | 1 |
| working.by_node.each | 57,668 | 0.305 | 8,827 |
| retire.woken | 57,669 | 0.281 | 1 |
| working.at | 57,669 | 0.25 | 1 |
| working.light | 57,669 | 0.25 | 1 |
| working.cost | 57,669 | 0.25 | 1 |
| working.sub | 57,669 | 0.25 | 1 |
| working.incoming | 57,669 | 0.25 | 1 |
| working.bud | 57,669 | 0.25 | 1 |
| working.open | 57,669 | 0.0625 | 1 |
| working.living | 57,669 | 0.0625 | 1 |
| working.closed | 3,223 | 0.0312 | 1 |
| retire.fading | 3,133 | 0.0312 | 1 |
| successor.each | 348 | 0.0106 | 348 |
| working.rest | 1 | 0.000244 | 1 |
| working.kids | 0 | 0.000244 | 1 |
| axes.units | 0 | 0 | 0 |
| axes.joined | 0 | 0 | 0 |
| marked | 0 | 0 | 1 |
| asleep | 0 | 0 | 1 |
| asleep.each | 0 | 0 | 0 |
| woken | 0 | 0 | 1 |
| pencils.inherited | 0 | 0 | 0 |
| retire.pending | 0 | 0 | 1 |
| retire.log | 0 | 0 | 1 |


### RUN census norway-spruce 40 1002

```
FOOT counted_gib 1.0255 axes_open_or_kept 312037 retired_roots 344976 inside_retired 48869
HEAD age 40 axes 705882 pencils 705882 live 106843 leaves 409114 lasting true
CHECK deposits replicated 409114 grower 409114 area replicated 160.0891 grower 160.0891
TOPSOTHER 0 carried-on axes turning faint on a bearer not faint: 3
TOPSOTHER 1 carried-on axes turning faint on a bearer not faint: 11
M3WOKEN faint woken laterals left out: 148
SCALE 1
exit 0 wall_s 5
```

**M1. Held phytomers by their axis's faintness (PA x bin x open/closed/retired-held)**

| PA | <0.05 open | <0.05 closed | <0.05 retired-held | 0.05-0.2 open | 0.05-0.2 closed | 0.05-0.2 retired-held | >=0.2 open | >=0.2 closed | >=0.2 retired-held | no pencil | total |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 seedling | 0 | 0 | 0 | 0 | 0 | 0 | 9 | 0 | 0 | 0 | 9 |
| 1 sapling | 0 | 0 | 0 | 0 | 0 | 0 | 56 | 0 | 0 | 0 | 56 |
| 2 trunk | 0 | 0 | 0 | 0 | 0 | 0 | 238 | 0 | 0 | 0 | 238 |
| 8 sprig | 0 | 0 | 0 | 24 | 0 | 0 | 3,508 | 0 | 0 | 0 | 3,532 |
| 9 branch | 0 | 0 | 0 | 116 | 0 | 0 | 7,792 | 0 | 0 | 0 | 7,908 |
| 10 branchlet | 531 | 0 | 0 | 6,087 | 192 | 0 | 338,793 | 6,609 | 0 | 0 | 352,212 |
| 11 spur | 183 | 1 | 0 | 1,997 | 74 | 0 | 68,630 | 1,246 | 0 | 0 | 72,131 |
| 12 shoot | 1,366 | 3,500 | 0 | 5,870 | 14,490 | 0 | 220,538 | 486,088 | 0 | 0 | 731,852 |
| all | 2,080 | 3,501 | 0 | 14,094 | 14,756 | 0 | 639,564 | 493,943 | 0 | 0 | 1,167,938 |

**M1. Held axes (PA x bin x open/closed/retired-held)**

| PA | <0.05 open | <0.05 closed | <0.05 retired-held | 0.05-0.2 open | 0.05-0.2 closed | 0.05-0.2 retired-held | >=0.2 open | >=0.2 closed | >=0.2 retired-held | no pencil | total |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 seedling | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 1 sapling | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 2 trunk | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 8 sprig | 0 | 0 | 0 | 1 | 0 | 0 | 160 | 0 | 0 | 0 | 161 |
| 9 branch | 0 | 0 | 0 | 1 | 0 | 0 | 135 | 0 | 0 | 0 | 136 |
| 10 branchlet | 30 | 1 | 2 | 287 | 8 | 41 | 17,606 | 335 | 1,785 | 0 | 20,095 |
| 11 spur | 23 | 1 | 2 | 207 | 6 | 15 | 9,235 | 196 | 588 | 0 | 10,273 |
| 12 shoot | 779 | 1,010 | 2,891 | 3,344 | 4,248 | 14,424 | 130,287 | 144,134 | 374,097 | 0 | 675,214 |
| all | 832 | 1,012 | 2,895 | 3,840 | 4,262 | 14,480 | 157,426 | 144,665 | 376,470 | 0 | 705,882 |

**M1. Held phytomers by their own rough-layout scale (status summed)**

| PA | <0.05 | 0.05-0.2 | >=0.2 |
|---|---|---|---|
| 0 seedling | 0 | 0 | 9 |
| 1 sapling | 0 | 0 | 56 |
| 2 trunk | 0 | 0 | 238 |
| 8 sprig | 0 | 24 | 3,508 |
| 9 branch | 0 | 116 | 7,792 |
| 10 branchlet | 1,800 | 9,435 | 340,977 |
| 11 spur | 197 | 2,034 | 69,900 |
| 12 shoot | 4,684 | 19,992 | 707,176 |
| all | 6,681 | 31,601 | 1,129,656 |

**M1. Living tips by axis faintness** (by status: {'open': 106843})

| PA | <0.05 | 0.05-0.2 | >=0.2 | no pencil | summed f |
|---|---|---|---|---|---|
| 2 trunk | 0 | 0 | 1 | 0 | 1 |
| 8 sprig | 0 | 0 | 24 | 0 | 17.5 |
| 9 branch | 0 | 1 | 134 | 0 | 96.5 |
| 10 branchlet | 10 | 86 | 8,154 | 0 | 5,848 |
| 11 spur | 23 | 204 | 9,033 | 0 | 6,468 |
| 12 shoot | 504 | 2,156 | 86,513 | 0 | 61,209 |
| all | 537 | 2,447 | 103,859 | 0 | 73,640 |

**M1. The grower's leaf (last cycle's deposits), m², and share of the whole**

| PA | m² | share | share <0.05 | share 0.05-0.2 | share >=0.2 | share no pencil |
|---|---|---|---|---|---|---|
| 2 trunk | 0.0606 | 0.000379 | 0 | 0 | 0.000379 | 0 |
| 8 sprig | 0.622 | 0.00389 | 0 | 0 | 0.00389 | 0 |
| 9 branch | 2.21 | 0.0138 | 0 | 2.16e-05 | 0.0138 | 0 |
| 10 branchlet | 47 | 0.294 | 1.24e-05 | 0.000676 | 0.293 | 0 |
| 11 spur | 5.36 | 0.0335 | 7.05e-06 | 0.000164 | 0.0333 | 0 |
| 12 shoot | 105 | 0.655 | 0.000276 | 0.00334 | 0.651 | 0 |
| all | 160 | 1 | 0.000295 | 0.00421 | 0.995 | 0 |

**M4. Last cycle's leaf deposits rounding to zero quanta on all 8 corners**

| bin | deposits | area m² | zero | zero share (count) | zero area m² | zero share (area) | net area lost to rounding m² |
|---|---|---|---|---|---|---|---|
| <0.05 | 1,625 | 0.0472 | 61 | 0.0375 | 4.31e-05 | 0.000913 | 0.000511 |
| 0.05-0.2 | 8,518 | 0.673 | 0 | 0 | 0 | 0 | 0.00111 |
| >=0.2 | 398,971 | 159 | 27 | 6.77e-05 | 1.32e-05 | 8.26e-08 | 0.0116 |

**Tops: faint laterals whose bearer node is not faint, and what is held under them (themselves included)**

| band | PA | tops | axes held | phytomers held | leaf m² |
|---|---|---|---|---|---|
| <0.05 on bearer >=0.05 | 10 branchlet | 31 | 418 | 1,614 | 0.0185 |
| <0.05 on bearer >=0.05 | 12 shoot | 558 | 558 | 1,638 | 0.0919 |
| <0.2 on bearer >=0.2 | 8 sprig | 1 | 3,165 | 13,997 | 0.732 |
| <0.2 on bearer >=0.2 | 10 branchlet | 114 | 1,801 | 6,781 | 0.377 |
| <0.2 on bearer >=0.2 | 12 shoot | 1,955 | 1,955 | 5,670 | 0.561 |

**Settled tree: phytomers under the drawing's 0.2 mm floor (radius x scale)**

| PA | held | under 0.2 mm | share |
|---|---|---|---|
| 0 seedling | 9 | 0 | 0 |
| 1 sapling | 56 | 0 | 0 |
| 2 trunk | 238 | 0 | 0 |
| 8 sprig | 3,532 | 0 | 0 |
| 9 branch | 7,908 | 2 | 0.000253 |
| 10 branchlet | 348,822 | 1,235 | 0.00354 |
| 11 spur | 71,405 | 762 | 0.0107 |
| 12 shoot | 611,294 | 24,340 | 0.0398 |
| all | 1,043,264 | 26,339 | 0.0252 |

**M3. Faint laterals <0.05: realised subtree over expectation**

| PA | metric | roots | p10 | p50 | p90 | sum real | sum expected | sum ratio |
|---|---|---|---|---|---|---|---|---|
| 10 branchlet | leaf/bL | 12 | 0.681 | 1 | 1.13 | 0.00683 | 0.0149 | 0.458 |
| 10 branchlet | leaf/fL | 12 | 0.871 | 1.01 | 1.21 | 0.00683 | 0.00703 | 0.971 |
| 10 branchlet | records/fN | 12 | 7.15 | 22 | 56.2 | 693 | 40.5 | 17.1 |
| 10 branchlet | scale/bN | 12 | 0.278 | 0.608 | 1.01 | 26.8 | 219 | 0.122 |
| 10 branchlet | scale/fN | 12 | 0.283 | 0.709 | 1.01 | 26.8 | 40.5 | 0.662 |
| 10 branchlet | wood/bW | 12 | 0.227 | 0.589 | 1.01 | 0.449 | 4.08 | 0.11 |
| 10 branchlet | wood/fW | 12 | 0.23 | 0.681 | 1.01 | 0.449 | 0.767 | 0.585 |
| 12 shoot | leaf/bL | 1,789 | 0.0315 | 1.17 | 1.19 | 0.146 | 0.296 | 0.494 |
| 12 shoot | leaf/fL | 1,789 | 0.585 | 1.17 | 1.19 | 0.146 | 0.0782 | 1.87 |
| 12 shoot | records/fN | 1,789 | 15 | 40.4 | 225 | 4,866 | 121 | 40.3 |
| 12 shoot | scale/bN | 1,789 | 0.0315 | 1.17 | 1.17 | 232 | 454 | 0.512 |
| 12 shoot | scale/fN | 1,789 | 0.585 | 1.17 | 1.17 | 232 | 121 | 1.92 |
| 12 shoot | wood/bW | 1,789 | 0.0315 | 1.17 | 1.17 | 4.65 | 9.07 | 0.512 |
| 12 shoot | wood/fW | 1,789 | 0.585 | 1.17 | 1.17 | 4.65 | 2.42 | 1.92 |

**M3. Faint laterals 0.05-0.2: realised subtree over expectation**

| PA | metric | roots | p10 | p50 | p90 | sum real | sum expected | sum ratio |
|---|---|---|---|---|---|---|---|---|
| 8 sprig | leaf/bL | 1 | 0.864 | 0.864 | 0.864 | 0.732 | 0.847 | 0.864 |
| 8 sprig | leaf/fL | 1 | 0.864 | 0.864 | 0.864 | 0.732 | 0.847 | 0.864 |
| 8 sprig | records/fN | 1 | 2.45 | 2.45 | 2.45 | 13,997 | 5,710 | 2.45 |
| 8 sprig | scale/bN | 1 | 0.326 | 0.326 | 0.326 | 1,864 | 5,710 | 0.326 |
| 8 sprig | scale/fN | 1 | 0.326 | 0.326 | 0.326 | 1,864 | 5,710 | 0.326 |
| 8 sprig | wood/bW | 1 | 0.297 | 0.297 | 0.297 | 32 | 108 | 0.297 |
| 8 sprig | wood/fW | 1 | 0.297 | 0.297 | 0.297 | 32 | 108 | 0.297 |
| 10 branchlet | leaf/bL | 183 | 0.486 | 0.995 | 1.27 | 0.719 | 1.05 | 0.686 |
| 10 branchlet | leaf/fL | 183 | 0.757 | 1.01 | 1.33 | 0.719 | 0.698 | 1.03 |
| 10 branchlet | records/fN | 183 | 1.88 | 3.36 | 7.59 | 12,723 | 3,991 | 3.19 |
| 10 branchlet | scale/bN | 183 | 0.233 | 0.342 | 1.01 | 1,842 | 5,643 | 0.326 |
| 10 branchlet | scale/fN | 183 | 0.26 | 0.496 | 1.02 | 1,842 | 3,991 | 0.461 |
| 10 branchlet | wood/bW | 183 | 0.176 | 0.316 | 1.01 | 31.7 | 107 | 0.296 |
| 10 branchlet | wood/fW | 183 | 0.201 | 0.47 | 1.02 | 31.7 | 75.6 | 0.42 |
| 12 shoot | leaf/bL | 7,592 | 0.144 | 1.17 | 1.19 | 1.82 | 2.2 | 0.827 |
| 12 shoot | leaf/fL | 7,592 | 0.585 | 1.17 | 1.19 | 1.82 | 1.67 | 1.09 |
| 12 shoot | records/fN | 7,592 | 3.55 | 8.49 | 14.9 | 20,360 | 2,586 | 7.87 |
| 12 shoot | scale/bN | 7,592 | 0.144 | 1.17 | 1.17 | 2,824 | 3,379 | 0.836 |
| 12 shoot | scale/fN | 7,592 | 0.585 | 1.17 | 1.17 | 2,824 | 2,586 | 1.09 |
| 12 shoot | wood/bW | 7,592 | 0.144 | 1.17 | 1.17 | 56.5 | 67.6 | 0.836 |
| 12 shoot | wood/fW | 7,592 | 0.585 | 1.17 | 1.17 | 56.5 | 51.7 | 1.09 |

**M2. Stages**

| stage | s | heap held GiB | heap most in stage GiB | VmHWM GiB |
|---|---|---|---|---|
| growth | 3.1 | 1.03 | 1.09 | 0.899 |
| reader | 0.2 | 1.03 | 1.08 | 0.899 |
| settle_from_copy | 0.7 | 1.22 | 1.66 | 1.4 |

**M2. Grower footprint by store at end of growth**

| store | records | MiB | allocations |
|---|---|---|---|
| pencils | 705,882 | 252 | 1 |
| axes | 705,882 | 232 | 1 |
| axes.phytomers | 1,167,938 | 182 | 239,184 |
| draws | 705,882 | 144 | 1 |
| draws.units | 558,361 | 34.6 | 239,184 |
| successor | 705,882 | 24 | 1 |
| working.links | 655,342 | 24 | 1 |
| working.by_node | 655,342 | 24 | 1 |
| leaves | 409,114 | 16 | 1 |
| retire.fate | 655,342 | 14.6 | 1 |
| draws.nodes | 1,167,938 | 10.7 | 239,184 |
| pencils.path | 2,443,959 | 10.6 | 575,782 |
| draws.shade | 558,361 | 8.66 | 239,184 |
| horizon | 705,882 | 8 | 1 |
| units | 705,882 | 8 | 1 |
| roots | 705,882 | 8 | 1 |
| working.pa | 655,342 | 8 | 1 |
| retire.woken | 655,342 | 4.85 | 1 |
| working.at | 655,342 | 4 | 1 |
| working.light | 655,342 | 4 | 1 |
| working.cost | 655,342 | 4 | 1 |
| working.sub | 655,342 | 4 | 1 |
| working.incoming | 655,342 | 4 | 1 |
| working.bud | 655,342 | 4 | 1 |
| working.by_node.each | 655,341 | 3.54 | 26,774 |
| live | 106,843 | 2.02 | 1 |
| retire.fading | 149,202 | 2 | 1 |
| working.open | 655,342 | 1 | 1 |
| working.list | 111,558 | 1 | 1 |
| working.living | 655,342 | 1 | 1 |
| working.closed | 43,264 | 0.5 | 1 |
| successor.each | 10,411 | 0.318 | 10,411 |
| woken | 6,675 | 0.312 | 1 |
| asleep.each | 2,177 | 0.215 | 25 |
| asleep | 70 | 0.00256 | 1 |
| working.rest | 1 | 0.000977 | 1 |
| working.kids | 0 | 0.000977 | 1 |
| retire.pending | 6 | 6.1e-05 | 1 |
| axes.units | 0 | 0 | 0 |
| axes.joined | 0 | 0 | 0 |
| marked | 0 | 0 | 1 |
| pencils.inherited | 0 | 0 | 0 |
| retire.log | 0 | 0 | 1 |

