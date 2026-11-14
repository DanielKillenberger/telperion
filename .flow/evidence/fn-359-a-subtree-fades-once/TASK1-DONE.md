# fn-359 task 1: a subtree fades once, 12 Nov 2026

The builder (Opus), on `fn-359-a-subtree-fades-once`. Base `0289bdd02`. The commits:

- `9b85a00e5`: the R1 reading.
- `5d1251209`: the R2 test, red on its own.
- `a243d12ef`: the fix.
- `12899fb3c`: the re-pin.

Every number below was measured in this session unless it is marked otherwise.

## R1, the reading, and the stop

`READING.md`. `girth::shares` multiplied the raw fade down every bearer, so the girth needed the product form. That is the spec's stop, and the work returned to the host. The host ruled on the stop (12 Nov 2026; written into the spec's design section): the girth fades once too, with the scale.

## The change

- `shed.rs`, in `shed_past`, parents first: each kept axis has an applied fade `A = min(A(bearer), own fade)`, with `A(seed) = 1`. A continuation's own fade is 1, so its `A` is its bearer's.
- Its vigour is multiplied by `A / A(bearer)`, computed in f32.
  - Where the bearer's `A` is 1, the factor is the own fade, to the bit, as before.
  - Where the own fade is no less than the bearer's `A`, the factor is 1.
- `Shed` carries `applied`, each kept axis's `A`. `Shed::index` and `Shed::fade` are gone, because nothing read them but `girth::shares`.
- The raw own fade stays local to `shed_past`. The loss test is the only thing that reads it, and it reads it as before, so what is shed does not change.
- `girth::shares` is deleted. The settle passes `Shed::applied` as the shares.
  - `shares` multiplied nothing but the fade down the bearers, so nothing else had to be kept.
  - An axis's share and the fade its scale carries are now one number.
- Docs (R6): `shed.rs`'s module doc, `girth.rs`'s module and `thicken` docs, and `docs/pipeline.md`'s girth paragraph each say that a subtree fades once.

### The closed form (read, as the host asked)

- `closed_form::kept` is test-only, "the reference the predictor's counts are held to".
- Its `Kept::shed` fades each child table by its own time-only fade, `clamp(delay + 1 - (m - s), 0, 1)`, which has no `alive` floor. A nested table is faded again at its powers.
- Its bound test, `no_wood_reaches_past_the_extent_bound`, multiplies the bound by the lateral's settled vigour and its node's scale. Below the lateral, fading once gives each axis a factor between the product form's and 1. Each descendant's scale relative to the lateral is `A(desc) / A(lateral) <= 1`.
- All four `closed_form::kept` tests and the seven `closed_form::predictor` tests passed unchanged after the change, so the run went on.

## R2

`crates/telperion-space/src/shed/tests.rs`:

- `a_line_idle_past_its_delay_fades_once_to_its_living_apex`. A trunk, a limb lateral and three relays. The last relay's apex lives at presence 1/8, and the line is idle past its delay (2) at age 20.
  - Red on the base, failing on its own commit: `the last relay at the lateral's scale`, left 0.000244140625 (1/4096, the fourth power), right 0.125.
  - Green after. The lateral's base scale is 0.125, and the last relay's base scale equals it. Both girth shares are 0.125, and the last relay's share equals the lateral's.
- `whole_presences_fade_nothing_kept`. The same line with its apex whole, at 10, 12 and 30. The shed tree equals the grown tree to the bit, every share is 1, and every base scale is 1. Green before and after.

## R3, continuity

The narrow suites were run after the change: `shed::`, `girth::`, `grow::`, `closed_form::kept`, `closed_form::predictor`, `unfold::` and `geometry::`, at 4 threads. 69 passed, 1 failed, 3 ignored, in 164 s.

- The failure was the digest pin `grow::growing_tests::a_growing_phytomer_is_read_as_it_was` (R5). It was re-pinned and passes.
- The rest passed unchanged. They include every fade, shed and by-degree test there, and the equalities:
  - paused: `a_tree_paused_and_grown_on_is_the_tree_grown_at_once`;
  - year: `the_tree_at_a_year_is_the_horizons_prefix` and `a_year_is_a_state_not_a_horizon`;
  - repeat and cores: `many_cores_grow_the_tree_one_does`;
  - let-go and retirement: `letting_go_of_retired_records_leaves_the_tree_as_it_was`, `the_working_set_and_retirement_leave_the_tree_as_it_was` and `a_retired_root_let_go_is_read_as_it_was`.
- `woken_buds_that_stop_at_once_retire_as_the_reference_has_them` passed after the change, run alone: about 210 s (3 tests in 212.7 s).
  - It did not finish inside the base run's 595 s limit, so the base has no result for it (FRICTION.md).
  - It runs over the gate's 120 s per-test cutoff and is not on `.config/gate-budgets.toml`'s list. That predates this change and is not checked here.
- No test that checked the product form had to change.
- Stop check: the spec's other stop asks whether a tree on the narrow suites is drawn larger than its grower's rough layout. No test on the narrow suites failed by drawing larger, but no test here compares the two directly, so the stop is unchecked (none read here).

## R4, the oak, before and after

- The before and after probes were built in this session from the same scratch probe crate, against this worktree's crates: before at `5d1251209`, after at `a243d12ef`.
- The probe is fn-352's, with the diagnosis's tips reader, `Paused::a5_tips`. It was added uncommitted for the runs and then removed.
- The "both" column counts the living tips the grower holds above scale 0.1 that are drawn under 0.01 (the diagnosis's reader). The share is taken of the matched tips above 0.1.

| tree | living tips (grower) | matched | grower > 0.1 | both, before | both, after | share, before | share, after |
|---|---|---|---|---|---|---|---|
| 1001 at 120 | 189,672 | 112,953 | 101,518 | 2,411 | 38 | 2.4% | 0.04% |
| 1001 at 200 | 55,499 | 33,697 | 30,803 | 952 | 1 | 3.1% | 0.003% |
| 1001 at 250 | 8,955 | 5,702 | 4,313 | 2,369 | 6 | 54.9% | 0.1% |
| 1001 at 280 | 5,711 | 3,599 | 2,520 | 2,304 | 12 | 91.4% | 0.5% |
| 1001 at 300 | 4,334 | 2,630 | 1,963 | 1,963 | 1 | 100% | 0.05% |
| 1007 at 120 | 295,913 | 170,011 | 149,849 | 5,296 | 235 | 3.5% | 0.2% |
| 1012 at 120 | 244,453 | 141,026 | 122,859 | 3,912 | 134 | 3.2% | 0.1% |

The before rows reproduce the diagnosis's table exactly. The grower's own columns are the same before and after, since growth did not change.

| tree | run | height m | drawn leaves | drawn leaf m2 | phytomers drawn | trunk at 1.3 m cm | three largest limbs' base cm | one-thread s | peak MiB |
|---|---|---|---|---|---|---|---|---|---|
| 1001 at 120 | before | 27.26 | 408,921 | 1068.9 | 2,593,693 | 107.4 | 54.8, 49.6, 47.1 | 115.8 | 3216 |
| 1001 at 120 | after | 27.26 | 415,061 | 1085.0 | 2,689,618 | 107.4 | 54.8, 49.6, 47.1 | 130.8 | 3308 |
| 1001 at 200 | before | 32.73 | 110,881 | 290.1 | 818,736 | 133.3 | 77.1, 66.6, 54.5 | 185.4 | 1471 |
| 1001 at 200 | after | 32.73 | 113,361 | 296.6 | 861,768 | 133.3 | 77.1, 66.6, 54.5 | 191.3 | 1489 |
| 1001 at 250 | before | 28.23 | 5,038 | 13.2 | 58,078 | 134.6 | 70.1, 53.2, 8.1 | 149.6 | 1216 |
| 1001 at 250 | after | 28.23 | 6,454 | 16.9 | 91,952 | 134.6 | 70.1, 53.2, 8.1 | 161.0 | 1216 |
| 1001 at 280 | before | 21.74 | 1,112 | 2.9 | 7,840 | 134.7 | 70.1, 36.3 | not run | |
| 1001 at 280 | after | 21.73 | 1,882 | 4.9 | 37,293 | 134.7 | 70.1, 36.3 | not run | |
| 1001 at 300 | before | 7.79 | 0 | 0.0 | 187 | 134.7 | 36.4, 25.5 | 230.1 | 1200 |
| 1001 at 300 | after | 7.79 | 87 | 0.2 | 15,458 | 134.7 | 36.4, 25.5 | 215.8 | 1202 |
| 1007 at 120 | before | 27.12 | 628,874 | 1644.1 | 3,660,852 | 138.3 | 75.6, 53.8, 53.0 | not run | |
| 1007 at 120 | after | 27.12 | 637,577 | 1666.9 | 3,796,227 | 138.3 | 75.6, 53.8, 53.0 | not run | |
| 1012 at 120 | before | 25.57 | 498,385 | 1303.0 | 2,993,794 | 117.9 | 74.3, 60.8, 36.0 | not run | |
| 1012 at 120 | after | 25.57 | 503,998 | 1317.7 | 3,104,882 | 117.9 | 74.3, 60.8, 36.0 | not run | |

One-thread seconds come from one run each, on one pinned core of a shared machine, inside the queue, so they are noisy.

- At 300 and 250 the before and after reads ran back to back.
- At 120 and 200 the reads were 8 to 11 minutes apart (the before reads came from the first cost run). The +13% at 120 is one pair, not a measured cost: the tree draws 3.7% more phytomers.
- The cost reads of 1001 at 280 and of seeds 1007 and 1012 were not run. The timebox ran out: the paired run was stopped at 18:30, after 1001 at 250, and nothing was left running.

The trunk column is the probe's diameter at 1.3 m, not at the trunk's base. The limbs are the three largest bases off the trunk path. Neither moves on any tree. The height moves by at most 6 mm (seed 1001 at 280).

### The stills (`sheet-1001-250-300.jpg`)

Is the tree after worse in any still? Not as a tree. One new feature in two of the stills may be read as a defect: a short, thick stub.

- **300, whole and leafless.** Both show the same 7.8 m post. After, three small growths stand on its upper part:
  - a short, thick, rounded stub on the left side, about two thirds up;
  - a small leafy sprig on the left, higher up;
  - a thin short branch on the right near the top.

  The post is still a post. The line's living wood is drawn at about a tenth of its grown size, as the design gives (0.104 on the diagnosis's line), so 15,458 phytomers and 87 leaves stand where 187 phytomers stood. The tree is no taller.
- **250, whole and leafless.** The same tree, the same height and limbs. After, there are a few more small twigs and leaf clusters on the inner limbs. Near the fork there is a short, thickened stub where the left limb leaves the trunk.
- **The stub, at both ages.** It is wood drawn at a radius of `sqrt(0.132)` of its section (0.36) but at about a tenth of its length, so it reads as a knob. The host ruled that the girth fades with the scale; the stub is what that ruling draws here. Whether it reads as a defect is the host's call.

## R5, the digest pins

| pin | tree | moved | the shed's count after the change: kept axes faded below one / nested under a bearer below one |
|---|---|---|---|
| `grow::growing_tests` oak seed 1 | `oak()`, 20, unlaid | yes, 6125320244909560580 to 10364051823450299320 | 16 / 1 |
| oak seed 7 | | no | 34 / 0 |
| beech seed 1 | `beech()`, 30 | yes, 14165751050450917155 to 2408497720773702210 | 90 / 4 |
| beech seed 7 | | no | 90 / 0 |
| spruce seed 1 | `spruce()`, 30 | yes, 17982918076840580537 to 10160230377130830586 | 295 / 2 |
| spruce seed 7 | | yes, 14152319693416367426 to 13617150108993612047 | 253 / 1 |
| `suite::rachis::the_axes_move_no_leaf` | the palm at 40 | no (passes) | not read |
| `suite::species` digests.json, 49 seeds | four presets at their preset ages | not run: the four `fixed_*` tests are `#[ignore]` (the scaffold's grower, which no build grows now) | not read |

- A pin moved exactly where a fade below one nests under a bearer's applied fade below one. Trees with fades below one but none nested did not move.
- The count was read by a measurement-only counter in `shed_past` and the probe's `nest` mode, both removed.
- The other digests in the workspace are compared between two runs, not committed.

## Commands

```
telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-space --lib shed::tests           # red on 5d1251209, green on a243d12ef
telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-space --lib -- shed:: girth:: grow:: closed_form::kept closed_form::predictor unfold:: geometry:: --skip woken_buds_that_stop_at_once
telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-space --lib -- grow::growing_tests woken_buds_that_stop_at_once
telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-core --lib -- suite::species::fixed_ suite::rachis::the_axes_move_no_leaf
# R4: the scratch probe (fn-352's, against this worktree, its own target dir); $F = the session scratchpad's fn359/
bash $F/build.sh before|after          # telperion-heavy 8 --threads 16 -- cargo build --profile ci --manifest-path $F/probe/Cargo.toml --target-dir $F/probe-target
bash $F/r4.sh before|after measure     # telperion-heavy 16 --gpu 2 --threads 16: probe tree (stills of 1001 at 250, 300) and probe diag (tips)
bash $F/r4.sh before|after cost        # telperion-heavy 12 --threads 1 -- taskset -c 3 probe cost, one job per tree
python3 -I $F/tables.py tables; python3 -I $F/tables.py sheet <dest>
```

Raw logs are in `raw/`, which is ignored.
