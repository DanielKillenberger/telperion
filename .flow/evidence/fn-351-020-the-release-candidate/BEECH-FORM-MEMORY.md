# The beech's `form` test: where the grower's memory goes

Measured 13 Nov 2026. Head: `.worktrees/fn-362` at 810e90bf3 (branch fn-362-a-runs-rings-have-the-sides-its-widest). Base: `.worktrees/gate-base`, detached at 5ebab2de0 (the commit GATE-FAILURES.md calls the base). Both built with `--profile ci` in their own `target/`, through `telperion-heavy`. A temporary read-only method on `Paused` (axes, phytomers, living apexes, sleeping buds, last cycle's leaves and their summed area) and an example were applied to both trees and removed again with `git apply -R`; head is clean, base shows only the three untracked files it had before.

## 1. The test

`crates/telperion-space/tests/form.rs:187`, `the_beech_grows_at_every_age_of_its_sheet`:

```
for age in [10, 20, 40, 80] {
    for seed in [1, 7] {
        let request = Request { age, seed, light: Light::NEUTRAL };
        let tree = grow(&beech(), request).unwrap_or_else(...);
        assert!(tree.axes[0].phytomers[0].radius > 0.0);
    }
}
```

Ages 10, 20, 40, 80; seeds 1 and 7; eight separate `grow` calls in turn on one thread (a fresh grower each, none continued, none in parallel; `grow` is `grow_cycle`: `Paused::with`, `grow_to(age)`, `finish`, which settles). The tree is dropped at the end of each iteration. It asserts the call returns Ok and the trunk's first phytomer has radius > 0. No mesh is built. Budget: 4,096 MiB (no entry in `.config/gate-budgets.toml`).

## 2. The test's peak alone

Peak = `ru_maxrss` of the test process at exit (equal to its VmHWM), read by a Python wrapper (`bfm/peak.py`) around the test binary run with `--exact`, not by sampling. Run twice at head, twice at base (the first pair ran before a rebuild; the table gives the rebuilt run, the first in brackets).

| Tree | Peak | Passes the test's own asserts | Over 4,096 MiB |
|---|---:|---|---|
| Head | 9,028 MiB (9,017) | yes (exit 0) | yes, by 4,932 MiB |
| Base | 1,869 MiB (1,842) | yes (exit 0) | no |

Head is 4.83 times base. The peak is the largest case alone: beech seed 7, age 80 grown by itself peaks at 8,961 MiB at head (1,793 MiB at base).

## 3. Per age, at the head

Each (age, seed) grown fresh in its own process (the test does not continue one grower). "Grown" columns are the grower after `grow_to(age)`, before the settle. Held phytomers are the sum over held axes; living apexes are `live`; sleeping buds are `asleep` (all 0); leaves and area are the last cycle's leaf list (count, sum of its areas). Retired roots and the other axis kinds are the footprint's `axes` (open, closed, retired roots, inside retired). RSS is the process's VmRSS after `grow_to`. Footprint bytes are the footprint reader's counted total (fn-357/fn-358's `Paused::footprint`; length and capacity times record size). "Whole" is `grow()` itself: the settled tree's phytomers and axes, and the process VmHWM at exit.

| Age | Seed | Held axes | Held phytomers | Living apexes | Sleeping buds | Retired roots | Open / closed / inside | Leaves (last cycle) | Leaf area | RSS after grow (MiB) | Footprint (MiB) | Whole: settled phytomers / axes | Whole: peak (MiB) |
|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|---|---:|
| 10 | 1 | 646 | 4,062 | 525 | 0 | 28 | 601 / 17 / 0 | 1,394 | 2.0 | 5.0 | 1.2 | 3,989 / 601 | 11 |
| 10 | 7 | 880 | 6,337 | 686 | 0 | 50 | 802 / 28 / 0 | 2,058 | 2.9 | 5.4 | 1.7 | 6,208 / 801 | 11 |
| 20 | 1 | 4,008 | 41,984 | 2,439 | 0 | 778 | 3,008 / 194 / 28 | 7,866 | 9.6 | 13.8 | 9.1 | 39,435 / 3,018 | 15 |
| 20 | 7 | 4,775 | 46,694 | 2,839 | 0 | 975 | 3,454 / 264 / 82 | 8,576 | 11.5 | 15.5 | 12.3 | 42,728 / 3,482 | 17 |
| 40 | 1 | 70,038 | 827,615 | 46,042 | 0 | 7,933 | 56,975 / 4,350 / 780 | 149,434 | 130.2 | 194.6 | 194.6 | 767,806 / 57,560 | 295 |
| 40 | 7 | 138,188 | 1,287,768 | 80,996 | 0 | 24,086 | 98,701 / 6,624 / 8,777 | 256,537 | 311.5 | 325.5 | 328.9 | 1,203,296 / 99,309 | 486 |
| 80 | 1 | 1,234,880 | 16,300,325 | 651,168 | 0 | 224,839 | 853,854 / 100,055 / 56,132 | 2,263,171 | 2,492.6 | 3,370.8 | 3,285.9 | 14,514,750 / 873,989 | 5,456 |
| 80 | 7 | 1,846,068 | 27,138,515 | 1,098,002 | 0 | 205,811 | 1,435,426 / 167,417 / 37,414 | 3,804,218 | 4,227.8 | 5,483.1 | 5,742.6 | 24,178,429 / 1,469,354 | 8,961 |

Footprint by store at age 80, head (top stores by bytes; the example printed the 14 largest):

| Store | Seed 1 records | Seed 1 MiB | Seed 7 records | Seed 7 MiB |
|---|---:|---:|---:|---:|
| axes.phytomers | 16,300,325 | 1,611.8 | 27,138,515 | 2,681.7 |
| pencils | 1,234,880 | 324.8 | 1,846,068 | 586.9 |
| axes | 1,234,880 | 291.6 | 1,846,068 | 654.6 |
| draws.units | 4,090,270 | 186.5 | 6,810,663 | 311.1 |
| draws | 1,234,880 | 181.0 | 1,846,068 | 406.3 |
| draws.nodes | 16,300,325 | 179.1 | 27,138,515 | 298.0 |
| leaves | 2,263,171 | 128.0 | 3,804,218 | 128.0 |
| pencils.path | 22,351,027 | 121.9 | 32,399,118 | 177.1 |
| draws.shade | 4,090,270 | 46.6 | 6,810,663 | 77.8 |
| successor | 1,234,880 | 30.2 | 1,846,068 | 67.7 |
| working.links | 1,125,523 | 26.5 | 1,659,322 | 48.3 |
| working.by_node | 1,125,523 | 26.5 | 1,659,322 | 48.3 |
| live | 651,168 | 16.0 | 1,098,002 | 32.0 |
| retire.fate | 1,125,523 | 13.2 | 1,659,322 | 24.2 |

## 3b. The same at the base, the three largest ages

The base's footprint reports axes as three kinds (open, closed, retired), with no stands and no inside-retired kind; its fields differ from the head's. Held phytomers are the base's own 136-byte `Phytomer`; the base has no `Growing`.

| Age | Seed | Held axes | Held phytomers | Living apexes | Sleeping buds | Axes by kind (base's three) | Leaves | Leaf area | RSS after grow (MiB) | Footprint (MiB) | Whole: settled phytomers / axes | Whole: peak (MiB) |
|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|---|---:|
| 20 | 1 | 2,309 | 13,767 | 930 | 0 | 1,511 / 473 / 325 | 2,868 | 3.3 | 8.6 | 5.4 | 11,923 / 1,353 | 11 |
| 20 | 7 | 2,148 | 11,091 | 770 | 0 | 1,242 / 492 / 414 | 2,173 | 2.6 | 7.8 | 4.7 | 9,449 / 1,101 | 11 |
| 40 | 1 | 30,874 | 161,621 | 10,871 | 0 | 17,485 / 6,095 / 7,294 | 32,936 | 26.6 | 65.5 | 60.1 | 139,117 / 15,556 | 69 |
| 40 | 7 | 46,165 | 259,267 | 19,486 | 0 | 29,800 / 8,466 / 7,899 | 55,092 | 71.3 | 104.5 | 102.3 | 226,731 / 26,848 | 110 |
| 80 | 1 | 428,190 | 2,446,627 | 129,908 | 0 | 234,865 / 145,934 / 47,391 | 444,198 | 519.1 | 918.7 | 1,006.6 | 2,019,189 / 201,189 | 1,015 |
| 80 | 7 | 858,707 | 4,207,420 | 227,571 | 0 | 407,972 / 240,823 / 209,912 | 772,212 | 862.7 | 1,642.0 | 1,645.0 | 3,482,593 / 350,348 | 1,793 |

(The "11 MiB" peaks at ages 10 and 20 are the wrapper's own Python/child floor, not the tree; the example's VmHWM at those ages is 5 to 17 MiB.)

## 4. Bytes per held phytomer and per held axis at age 80

Footprint = the reader's counted bytes; RSS = VmRSS after `grow_to`; Peak = the whole `grow()` process peak (grower plus settle) over the held count.

| | Held phytomers | Held axes | Footprint B/phytomer | RSS B/phytomer | Peak B/phytomer | Footprint B/axis | RSS B/axis | Peak B/axis |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Head, seed 7 | 27,138,515 | 1,846,068 | 221.9 | 211.9 | 346.2 | 3,262 | 3,114 | 5,090 |
| Head, seed 1 | 16,300,325 | 1,234,880 | 211.4 | 216.8 | 350.9 | 2,790 | 2,862 | 4,633 |
| Base, seed 7 | 4,207,420 | 858,707 | 410.0 | 409.2 | 446.9 | 2,009 | 2,005 | 2,190 |
| Base, seed 1 | 2,446,627 | 428,190 | 431.4 | 393.7 | 435.1 | 2,465 | 2,250 | 2,486 |

Head holds 6.45 times the base's phytomers (seed 7) and 2.15 times its axes at age 80, with roughly half the bytes per phytomer.

## 4b. Which store grew most, base to head, age 80

MiB of footprint bytes (capacity times record). Sorted by growth, seed 7, then seed 1.

| Store | Seed 7 base | Seed 7 head | Seed 7 delta | Seed 1 base | Seed 1 head | Seed 1 delta |
|---|---:|---:|---:|---:|---:|---:|
| axes.phytomers | 769.0 | 2,681.7 | +1,912.6 | 448.0 | 1,611.8 | +1,163.8 |
| axes | 217.3 | 654.6 | +437.3 | 156.9 | 291.6 | +134.7 |
| pencils | 179.8 | 586.9 | +407.1 | 129.8 | 324.8 | +194.9 |
| draws | 134.9 | 406.3 | +271.4 | 97.4 | 181.0 | +83.6 |
| draws.units | 47.0 | 311.1 | +264.2 | 27.2 | 186.5 | +159.3 |
| draws.nodes | 45.2 | 298.0 | +252.7 | 26.4 | 179.1 | +152.7 |
| pencils.path | 0 (not listed) | 177.1 | +177.1 | 0 (not listed) | 121.9 | +121.9 |
| leaves | 32.0 | 128.0 | +96.0 | 16.0 | 128.0 | +112.0 |
| draws.shade | 23.5 | 77.8 | +54.3 | 13.6 | 46.6 | +33.0 |
| successor | 22.5 | 67.7 | +45.2 | 16.2 | 30.2 | +13.9 |

`axes.phytomers` records per byte: base seed 7, 4,207,420 records in 769.0 MiB (191.6 B each by capacity); head, 27,138,515 in 2,681.7 MiB (103.6 B each). `pencils.path` and `live` show 0 at the base only because they are outside the base's 14 printed stores, not because they are absent. The base's `units`, `roots`, `horizon`, `working.pa` and `retire.frozen` stores are not in the head's list (head seed 1: `units`, `roots`, `horizon` 0).

## Unknown

- The settle's own share of the peak was not split by store: the whole process peaks at 8,961 MiB (seed 7, age 80) against 5,483 MiB resident after `grow_to`, a 3,478 MiB difference, but the settle's allocations were not itemised.
- Footprint stores below the 14 printed per case were not recorded, so a store absent from the tables may be small, not zero.
- Leaf area is the sum of the areas in the last cycle's leaf list, not the whole tree's leaf area. Sleeping buds read 0 at every age and side (the beech has none at these ages by this counter).
- The base footprint's `axes` kinds are three, not four, so "retired roots" and "inside retired" are not separable at the base.
- The 8 GB scope's own accounting (cgroup, including page cache) was not read; the figures here are process RSS.
- The peak is one run each (two at head and base, one pair before and one after a rebuild; they agree within 1%).

## Closing

At the head the test's peak is 9,028 MiB (base 1,869 MiB), and it comes from beech seed 7 at age 80, which alone peaks at 8,961 MiB after holding 27,138,515 phytomers and 1,846,068 axes (1,098,002 living apexes, 205,811 retired roots) at 5,483 MiB resident before the settle. That is 222 bytes of footprint (212 resident, 346 at the whole-process peak) per held phytomer and 3,262 per held axis, against 410 and 2,009 at the base's 4,207,420 phytomers and 858,707 axes. The store that grew most is `axes.phytomers`, from 769.0 to 2,681.7 MiB at seed 7.

Side effect to note: this run overwrote same-named files already in the scratchpad (`build.sh`, `peak.py`, `runall.sh`, `build-head.log`, `build-base.log`, `probe-head.patch`, `probe-base.patch`) before moving its own copies into `bfm/`.
